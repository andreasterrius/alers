use crate::constant::{CAMERA_POSITION, ENVIRONMENT_MAP, MODEL, PROJECTION, VIEW};
use crate::mesh::{ale_opengl_mesh_new, OpenGLMesh};
use crate::old::cubemap::Cubemap;
use crate::raw;
use crate::shader::{ale_opengl_shader_activate, ale_opengl_shader_new, OpenGLShader};
use crate::texture::{ale_opengl_texture_new, OpenGLTexture};
use ale_camera::CameraRenderInfo;
use ale_math::rect::Rect;
use ale_math::transform::AleTransform;
use ale_math::{perspective, Array, Deg, EuclideanSpace, Matrix, Matrix4, Point3, Vector3};
use ale_mesh::{ale_mesh_cube_new, Mesh, MeshId};
use ale_shader::{ale_shader_new, Shader};
use ale_texture::{ale_texture_load, Texture};
use ale_variable::Variable;
use std::collections::HashMap;

pub struct OpenGLPBRContext {
  pub cube_mesh: OpenGLMesh,

  pub pbr_shader: OpenGLShader,
  pub equirect_shader: OpenGLShader,
  pub irradiance_shader: OpenGLShader,
  pub skybox_shader: OpenGLShader,

  pub hdr_texture: OpenGLTexture,

  pub irradiance_cubemap: u32,
  // OpenGLCubemap
  pub convoluted_cubemap: u32, // OpenGLCubemap

  //This is temporary to be put here, a proper way would be to have this in a single centralized location
  pub mesh: HashMap<MeshId, OpenGLMesh>,
}

pub fn ale_opengl_pbr_context_new(
  hdr_texture: &Texture,
  viewport_size: &Rect,
  meshes: Vec<&mut Mesh>,
) -> OpenGLPBRContext {
  let cube_mesh = ale_opengl_mesh_new(&ale_mesh_cube_new()).unwrap();
  let pbr_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../../../resources/shaders/pbr.vert").to_owned(),
    include_str!("../../../resources/shaders/pbr.frag").to_owned(),
  ))
  .unwrap();
  let equirect_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../../../resources/shaders/cubemap.vert").to_owned(),
    include_str!("../../../resources/shaders/equirect.frag").to_owned(),
  ))
  .unwrap();
  let irradiance_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../../../resources/shaders/cubemap.vert").to_owned(),
    include_str!("../../../resources/shaders/irradiance.frag").to_owned(),
  ))
  .unwrap();
  let skybox_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../../../resources/shaders/skybox.vert").to_owned(),
    include_str!("../../../resources/shaders/skybox.frag").to_owned(),
  ))
  .unwrap();

  let ogl_hdr_texture = ale_opengl_texture_new(hdr_texture).unwrap();

  let cubemap_size = Rect::new(512, 512);
  let cubemap_id = unsafe { raw::create_cubemap(cubemap_size.get_width(), cubemap_size.get_height()) };

  intern_opengl_pbr_equirect_project(
    &equirect_shader,
    ProjectionTarget::Texture2d(&ogl_hdr_texture),
    viewport_size,
    &cubemap_size,
    cubemap_id,
    &cube_mesh,
  );

  let convoluted_cubemap_size = Rect::new(32, 32);
  let convoluted_cubemap_id = unsafe {
    raw::create_cubemap(
      convoluted_cubemap_size.get_width(),
      convoluted_cubemap_size.get_height(),
    )
  };

  intern_opengl_pbr_equirect_project(
    &irradiance_shader,
    ProjectionTarget::Cubemap(cubemap_id),
    viewport_size,
    &convoluted_cubemap_size,
    convoluted_cubemap_id,
    &cube_mesh,
  );

  let mut ogl_mesh = HashMap::new();
  for mesh in meshes {
    ogl_mesh.insert(mesh.uid(), ale_opengl_mesh_new(mesh).unwrap());
  }

  OpenGLPBRContext {
    cube_mesh,
    pbr_shader,
    equirect_shader,
    irradiance_shader,
    skybox_shader,
    hdr_texture: ogl_hdr_texture,
    irradiance_cubemap: cubemap_id,
    convoluted_cubemap: convoluted_cubemap_id,
    mesh: ogl_mesh,
  }
}

pub fn ale_opengl_pbr_render(
  opengl_pbr_context: &OpenGLPBRContext,
  mesh: Vec<(&mut AleTransform, &mut Mesh, &Vector3<f32>)>,
  camera_render_info: &CameraRenderInfo,
  textures: &Vec<OpenGLTexture>,
) {
  unsafe {
    let pbr_shader = &opengl_pbr_context.pbr_shader;
    let cubemap = opengl_pbr_context.convoluted_cubemap;

    // Bind shader
    ale_opengl_shader_activate(
      pbr_shader,
      &vec![
        Variable::F32_3("albedo".to_owned(), Vector3::new(0.7f32, 0.7, 0.7)),
        Variable::F32_1("metallic".to_owned(), 0.0f32),
        Variable::F32_1("roughness".to_owned(), 0.5f32),
        Variable::F32_1("ao".to_owned(), 0.5f32),
      ],
    );

    raw::uniform1i(pbr_shader.id, "irradianceMap", 0);
    raw::active_texture(0);
    raw::bind_cubemap(cubemap);

    // Bind textures here
    for i in 0..textures.len() {
      raw::active_texture((i + 1) as u32);
      raw::bind_texture(textures[i].id.0);
    }

    let camera_position = camera_render_info.position;
    raw::uniform3f(
      pbr_shader.id,
      CAMERA_POSITION,
      camera_position.x,
      camera_position.y,
      camera_position.z,
    );

    raw::matrix4f(pbr_shader.id, VIEW, camera_render_info.view.as_ptr());
    raw::matrix4f(pbr_shader.id, PROJECTION, camera_render_info.projection.as_ptr());

    for (t, m, color) in mesh {
      ale_opengl_shader_activate(pbr_shader, &vec![Variable::F32_3("albedo".to_owned(), *color)]);

      let ogl_mesh = opengl_pbr_context
        .mesh
        .get(&m.uid())
        .expect(&format!("{:?} is not a pbr registered mesh", m.uid()));

      // Pass uniforms
      //let mut t = Transform::new();
      raw::matrix4f(pbr_shader.id, MODEL, t.matrix().as_ptr());

      // Bind Array Buffer
      raw::bind_vao(ogl_mesh.vao);

      // Draw according to EBO
      match ogl_mesh.ebo {
        None => raw::draw_arrays(0, ogl_mesh.draw_size),
        Some(_) => raw::draw_elements(ogl_mesh.draw_size),
      }
    }
  }
}

pub fn ale_opengl_pbr_render_envmap(opengl_pbr_context: &OpenGLPBRContext, camera_render_info: &CameraRenderInfo) {
  unsafe {
    let skybox_shader = &opengl_pbr_context.skybox_shader;
    let convoluted_cubemap = opengl_pbr_context.convoluted_cubemap;
    let cube_mesh = &opengl_pbr_context.cube_mesh;

    raw::use_shader(skybox_shader.id);
    raw::uniform1i(skybox_shader.id, ENVIRONMENT_MAP, 0);
    raw::matrix4f(skybox_shader.id, VIEW, camera_render_info.view.as_ptr());
    raw::matrix4f(skybox_shader.id, PROJECTION, camera_render_info.projection.as_ptr());
    raw::active_texture(0);
    raw::bind_cubemap(convoluted_cubemap);

    raw::bind_vao(cube_mesh.vao);
    match cube_mesh.ebo {
      None => raw::draw_arrays(0, cube_mesh.draw_size),
      Some(_) => raw::draw_elements(cube_mesh.draw_size),
    }
  }
}

pub fn ale_opengl_pbr_render_debug(
  opengl_pbr_context: &OpenGLPBRContext,
  point: Vector3<f32>,
  size: f32,
  color: Vector3<f32>,
  camera_render_info: &CameraRenderInfo,
) {
  unsafe {
    let pbr_shader = &opengl_pbr_context.pbr_shader;
    let cubemap = opengl_pbr_context.convoluted_cubemap;

    // Bind shader
    ale_opengl_shader_activate(
      pbr_shader,
      &vec![
        Variable::F32_3("albedo".to_owned(), color),
        Variable::F32_1("metallic".to_owned(), 0.0f32),
        Variable::F32_1("roughness".to_owned(), 0.5f32),
        Variable::F32_1("ao".to_owned(), 0.5f32),
      ],
    );

    raw::uniform1i(pbr_shader.id, "irradianceMap", 0);
    raw::active_texture(0);
    raw::bind_cubemap(cubemap);

    let camera_position = camera_render_info.position;
    raw::uniform3f(
      pbr_shader.id,
      CAMERA_POSITION,
      camera_position.x,
      camera_position.y,
      camera_position.z,
    );

    raw::matrix4f(pbr_shader.id, VIEW, camera_render_info.view.as_ptr());
    raw::matrix4f(pbr_shader.id, PROJECTION, camera_render_info.projection.as_ptr());

    let mut t = AleTransform::from_position_scale(point, Vector3::from_value(size));
    let ogl_mesh = &opengl_pbr_context.cube_mesh;

    // Pass uniforms
    raw::matrix4f(pbr_shader.id, MODEL, t.matrix().as_ptr());

    // Bind Array Buffer
    raw::bind_vao(ogl_mesh.vao);

    // Draw according to EBO
    match ogl_mesh.ebo {
      None => raw::draw_arrays(0, ogl_mesh.draw_size),
      Some(_) => raw::draw_elements(ogl_mesh.draw_size),
    }
  }
}

pub enum ProjectionTarget<'a> {
  Cubemap(u32),
  Texture2d(&'a OpenGLTexture),
}

fn intern_opengl_pbr_equirect_project(
  equirect_shader: &OpenGLShader,
  projection_target: ProjectionTarget,
  original_dimension: &Rect,
  projection_dimension: &Rect,
  cubemap_id: u32,
  cube_mesh: &OpenGLMesh,
) {
  let projection = perspective(Deg(90.0f32), 1.0f32, 0.1f32, 10.0f32);
  let views = vec![
    Matrix4::look_at(Point3::origin(), Point3::new(1.0f32, 0.0, 0.0), -Vector3::unit_y()),
    Matrix4::look_at(Point3::origin(), Point3::new(-1.0f32, 0.0, 0.0), -Vector3::unit_y()),
    Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 1.0, 0.0), Vector3::unit_z()),
    Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, -1.0, 0.0), -Vector3::unit_z()),
    Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, 1.0), -Vector3::unit_y()),
    Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, -1.0), -Vector3::unit_y()),
  ];

  unsafe {
    let (framebuffer, _) =
      raw::create_framebuffer_cubemap(projection_dimension.get_height(), projection_dimension.get_width());

    raw::use_shader(equirect_shader.id);
    raw::uniform1i(equirect_shader.id, "equirectangularMaps", 0);
    raw::matrix4f(equirect_shader.id, PROJECTION, projection.as_ptr());

    raw::active_texture(0);
    match projection_target {
      ProjectionTarget::Cubemap(c) => {
        raw::bind_cubemap(c);
      }
      ProjectionTarget::Texture2d(c) => {
        raw::bind_texture(c.id.0);
      }
    };

    raw::set_viewport(
      projection_dimension.get_x(),
      projection_dimension.get_y(),
      projection_dimension.get_width(),
      projection_dimension.get_height(),
    );
    raw::bind_framebuffer(framebuffer.0);
    for i in 0..6 {
      raw::matrix4f(equirect_shader.id, VIEW, views[i].as_ptr());
      raw::framebuffer_texture2d_cubemap(i as u32, cubemap_id, 0);
      raw::clear_buffer(0.2f32, 0.3f32, 0.3f32);

      raw::bind_vao(cube_mesh.vao);
      raw::draw_arrays(0, cube_mesh.draw_size);
    }

    raw::bind_vao(0);
    // unbind framebuffer
    raw::bind_framebuffer(0);
    raw::set_viewport(0, 0, original_dimension.get_width(), original_dimension.get_height());
  }
}
