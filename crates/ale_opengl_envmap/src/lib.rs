use ale_camera::CameraRenderInfo;
use ale_math::{perspective, Deg, EuclideanSpace, Matrix, Matrix4, Point3, Vector2, Vector3};
use ale_mesh::ale_mesh_cube_new;
use ale_opengl::mesh::{ale_opengl_mesh_context_new, ale_opengl_mesh_new, ale_opengl_mesh_render, OpenGLMesh};
use ale_opengl::raw;
use ale_opengl::shader::{ale_opengl_shader_new, OpenGLShader};
use ale_opengl::texture::{ale_opengl_texture_new, OpenGLTexture};
use ale_shader::ale_shader_new;
use ale_texture::Texture;

pub struct OpenGLEnvmap {
  pub(crate) cube_mesh: OpenGLMesh,

  pub(crate) equirect_shader: OpenGLShader,
  pub(crate) irradiance_shader: OpenGLShader,
  pub(crate) skybox_shader: OpenGLShader,

  pub(crate) cubemap_size: Vector2<u32>,
  pub(crate) cubemap: u32,

  pub(crate) convoluted_cubemap_size: Vector2<u32>,
  pub(crate) convoluted_cubemap: u32,
}

enum ProjectionTarget<'a> {
  Cubemap(u32),
  Texture2d(&'a OpenGLTexture),
}

pub fn ale_opengl_envmap_new(hdr_texture: &Texture, window_size: Vector2<u32>) -> OpenGLEnvmap {
  let cube_mesh = ale_opengl_mesh_new(&ale_mesh_cube_new()).unwrap();

  let equirect_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../resources/cubemap.vert").to_owned(),
    include_str!("../resources/equirect.frag").to_owned(),
  ))
  .unwrap();
  let irradiance_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../resources/cubemap.vert").to_owned(),
    include_str!("../resources/irradiance.frag").to_owned(),
  ))
  .unwrap();
  let skybox_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../resources/skybox.vert").to_owned(),
    include_str!("../resources/skybox.frag").to_owned(),
  ))
  .unwrap();

  let cubemap_size = Vector2::new(512, 512);
  let convoluted_cubemap_size = Vector2::new(32, 32);

  let cubemap = unsafe { raw::create_cubemap(cubemap_size.x, cubemap_size.y) };
  let convoluted_cubemap = unsafe { raw::create_cubemap(convoluted_cubemap_size.x, convoluted_cubemap_size.y) };

  let context = OpenGLEnvmap {
    cube_mesh,
    equirect_shader,
    irradiance_shader,
    skybox_shader,
    cubemap_size,
    cubemap,
    convoluted_cubemap_size,
    convoluted_cubemap,
  };

  let opengl_texture = ale_opengl_texture_new(hdr_texture).unwrap();

  intern_envmap_project(
    &context.cube_mesh,
    &context.equirect_shader,
    ProjectionTarget::Texture2d(&opengl_texture),
    cubemap_size,
    cubemap,
    window_size,
  );
  intern_envmap_project(
    &context.cube_mesh,
    &context.irradiance_shader,
    ProjectionTarget::Cubemap(context.cubemap),
    convoluted_cubemap_size,
    convoluted_cubemap,
    window_size,
  );

  context
}

pub fn ale_opengl_envmap_render(opengl_envmap: &OpenGLEnvmap, camera_render_info: &CameraRenderInfo) {
  unsafe {
    let shader = opengl_envmap.skybox_shader.id;
    let cubemap = opengl_envmap.convoluted_cubemap;
    let cube_mesh = &opengl_envmap.cube_mesh;

    raw::use_shader(shader);
    raw::uniform1i(shader, "environmentMap", 0);
    raw::matrix4f(shader, "view", camera_render_info.view.as_ptr());
    raw::matrix4f(shader, "projection", camera_render_info.projection.as_ptr());
    raw::active_texture(0);
    raw::bind_cubemap(cubemap);

    ale_opengl_mesh_render(cube_mesh);
  }
}

pub fn ale_opengl_envmap_bind_to_shader(
  opengl_envmap: &OpenGLEnvmap,
  opengl_shader: &OpenGLShader,
  shader_variable_name: &str,
) {
  unsafe {
    raw::uniform1i(opengl_shader.id, shader_variable_name, 0);
    raw::active_texture(0);
    raw::bind_cubemap(opengl_envmap.convoluted_cubemap);
  }
}

fn intern_envmap_project(
  cube_mesh: &OpenGLMesh,
  shader: &OpenGLShader,
  projection_target: ProjectionTarget,
  projection_size: Vector2<u32>,
  cubemap_id: u32, // render target
  window_size: Vector2<u32>,
) {
  let cube_mesh = cube_mesh;
  let equirect_shader = shader;

  let projection = perspective(Deg(90.0f32), 1.0f32, 0.1f32, 10.0f32);
  let views = vec![
    Matrix4::look_at(Point3::origin(), Point3::new(1.0f32, 0.0, 0.0), -Vector3::unit_y()),
    Matrix4::look_at(Point3::origin(), Point3::new(-1.0f32, 0.0, 0.0), -Vector3::unit_y()),
    Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 1.0, 0.0), Vector3::unit_z()),
    Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, -1.0, 0.0), -Vector3::unit_z()),
    Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, 1.0), -Vector3::unit_y()),
    Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, -1.0), -Vector3::unit_y()),
  ];
  let equirect_shader = equirect_shader.id;

  unsafe {
    let (framebuffer, _) = raw::create_framebuffer_cubemap(projection_size.x, projection_size.y);

    raw::use_shader(equirect_shader);
    raw::uniform1i(equirect_shader, "equirectangularMaps", 0);
    raw::matrix4f(equirect_shader, "projection", projection.as_ptr());

    raw::active_texture(0);
    match projection_target {
      ProjectionTarget::Cubemap(c) => {
        raw::bind_cubemap(c);
      }
      ProjectionTarget::Texture2d(c) => {
        raw::bind_texture(c.id.0);
      }
    };

    raw::set_viewport(0, 0, projection_size.x, projection_size.y);
    raw::bind_framebuffer(framebuffer.0);
    for i in 0..6 {
      raw::matrix4f(equirect_shader, "view", views[i].as_ptr());
      raw::framebuffer_texture2d_cubemap(i as u32, cubemap_id, 0);
      raw::clear_buffer();

      raw::bind_vao(cube_mesh.vao);
      raw::draw_arrays(0, cube_mesh.draw_size);
    }

    raw::bind_vao(0);
    // unbind framebuffer
    raw::bind_framebuffer(0);
    raw::set_viewport(0, 0, window_size.x, window_size.y);
  }
}
