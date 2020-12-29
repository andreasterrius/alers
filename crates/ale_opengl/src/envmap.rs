use crate::mesh::OpenGLMesh;
use crate::raw;
use crate::resource_pile::{OpenGLResource, OpenGLResourceLoader, OpenGLResourcePile, OpenGLResourceType};
use crate::route_loader;
use crate::shader::OpenGLShader;
use crate::texture::OpenGLTexture;
use ale_camera::CameraRenderInfo;
use ale_math::{perspective, Deg, EuclideanSpace, Matrix, Matrix4, Point3, Vector2, Vector3};
use ale_mesh::Mesh;
use ale_resource::{Resource, ResourcePile};
use ale_shader::{Shader, ShaderLoader};
use ale_texture::envmap::Envmap;
use ale_texture::Texture;

pub struct OpenGLEnvmap {
  pub(crate) cube_mesh: OpenGLResource<OpenGLMesh>,

  pub(crate) equirect_shader: OpenGLResource<OpenGLShader>,
  pub(crate) irradiance_shader: OpenGLResource<OpenGLShader>,
  pub(crate) skybox_shader: OpenGLResource<OpenGLShader>,

  pub(crate) cubemap_size: Vector2<u32>,
  pub(crate) cubemap: u32,

  pub(crate) convoluted_cubemap_size: Vector2<u32>,
  pub(crate) convoluted_cubemap: u32,
}

enum ProjectionTarget {
  Cubemap(u32),
  Texture2d(OpenGLResource<OpenGLTexture>),
}

impl OpenGLEnvmap {
  pub fn new(
    hdr_texture: OpenGLResource<OpenGLTexture>,
    equirect_shader: OpenGLResource<OpenGLShader>,
    irradiance_shader: OpenGLResource<OpenGLShader>,
    skybox_shader: OpenGLResource<OpenGLShader>,
    cube_mesh: OpenGLResource<OpenGLMesh>,
  ) -> OpenGLEnvmap {
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

    intern_envmap_project(
      &context.cube_mesh.read(),
      &context.equirect_shader.read(),
      ProjectionTarget::Texture2d(hdr_texture),
      cubemap_size,
      cubemap,
    );
    intern_envmap_project(
      &context.cube_mesh.read(),
      &context.irradiance_shader.read(),
      ProjectionTarget::Cubemap(context.cubemap),
      convoluted_cubemap_size,
      convoluted_cubemap,
    );

    context
  }

  pub fn render(&self, camera_render_info: &CameraRenderInfo) {
    unsafe {
      let shader = self.skybox_shader.read().id;
      let cubemap = self.convoluted_cubemap;
      let cube_mesh = &self.cube_mesh;

      raw::use_shader(shader);
      raw::uniform1i(shader, "environmentMap", 0);
      raw::matrix4f(shader, "view", camera_render_info.view.as_ptr());
      raw::matrix4f(shader, "projection", camera_render_info.projection.as_ptr());
      raw::active_texture(0);
      raw::bind_cubemap(cubemap);

      cube_mesh.read().render();
    }
  }

  pub fn bind_to_shader(&self, opengl_shader: &OpenGLShader, shader_variable_name: &str) {
    unsafe {
      raw::uniform1i(opengl_shader.id, shader_variable_name, 0);
      raw::active_texture(0);
      raw::bind_cubemap(self.convoluted_cubemap);
    }
  }
}

fn intern_envmap_project(
  cube_mesh: &OpenGLMesh,
  shader: &OpenGLShader,
  projection_target: ProjectionTarget,
  projection_size: Vector2<u32>,
  cubemap_id: u32, // render target
) {
  let cube_mesh = cube_mesh;
  let equirect_shader = shader;
  let (ori_view_x, ori_view_y) = unsafe { raw::get_viewport_size() };
  let original_viewport_size = Vector2::new(ori_view_x, ori_view_y);

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
        raw::bind_texture(c.read().id.0);
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
    raw::set_viewport(0, 0, original_viewport_size.x, original_viewport_size.y);
  }
}

pub struct OpenGLEnvmapLoader;

impl OpenGLResourceLoader<Envmap, OpenGLEnvmap> for OpenGLEnvmapLoader {
  fn create(&self, opengl_resource_pile: &OpenGLResourcePile, resource: &Envmap) -> OpenGLEnvmap {
    let hdr_texture = opengl_resource_pile
      .retrieve(&*resource.texture.read())
      .unwrap()
      .clone();
    let equirect_shader = opengl_resource_pile
      .retrieve(&*resource.equirect_shader.read())
      .unwrap()
      .clone();
    let irradiance_shader = opengl_resource_pile
      .retrieve(&*resource.irradiance_shader.read())
      .unwrap()
      .clone();
    let skybox_shader = opengl_resource_pile
      .retrieve(&*resource.skybox_shader.read())
      .unwrap()
      .clone();
    let cube_mesh = opengl_resource_pile
      .retrieve(&*resource.cube_mesh.read())
      .unwrap()
      .clone();

    OpenGLEnvmap::new(
      hdr_texture,
      equirect_shader,
      irradiance_shader,
      skybox_shader,
      cube_mesh,
    )
  }
}
route_loader!(OpenGLEnvmapLoader, Envmap);

impl OpenGLResourceType for OpenGLEnvmap {}
