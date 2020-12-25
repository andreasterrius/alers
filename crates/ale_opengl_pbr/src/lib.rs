use ale_camera::CameraRenderInfo;
use ale_math::transform::Transform;
use ale_math::{Matrix, Matrix4};
use ale_mesh::{Mesh, MeshId};
use ale_opengl::mesh::{OpenGLMesh, OpenGLMeshContext};
use ale_opengl::raw;
use ale_opengl::shader::OpenGLShader;
use ale_opengl_envmap::OpenGLEnvmap;
use ale_shader::Shader;
use ale_variable::Variable;
use std::collections::HashMap;

pub struct OpenGLPBRContext {
  pub(crate) pbr_shader: OpenGLShader,
}

impl OpenGLPBRContext {
  pub fn new() -> OpenGLPBRContext {
    let pbr_shader = OpenGLShader::new(&Shader::new(
      include_str!("../resources/pbr.vert").to_owned(),
      include_str!("../resources/pbr.frag").to_owned(),
    ))
    .unwrap();

    OpenGLPBRContext { pbr_shader }
  }

  pub fn render(
    opengl_pbr_context: &OpenGLPBRContext,
    opengl_mesh_context: &mut OpenGLMeshContext,
    opengl_envmap: Option<&OpenGLEnvmap>,
    transform: &mut Transform,
    mesh: &Mesh,
    camera_render_info: &CameraRenderInfo,
    shader_variables: &Vec<Variable>,
  ) {
    unsafe {
      let shader = &opengl_pbr_context.pbr_shader;
      let ogl_mesh = opengl_mesh_context.register(mesh);

      shader.activate(shader_variables);
      if let Some(opengl_envmap) = opengl_envmap {
        opengl_envmap.bind_to_shader(shader, "irradianceMap");
      }

      // Bind textures here
      // for i in 0..texture_ids.len() {
      //   let texture_draw_info = match context.get_texture(&texture_ids[i]) {
      //     None => continue,
      //     Some(x) => x,
      //   };
      //
      //   raw::active_texture((i + 1) as u32);
      //   raw::bind_texture(texture_draw_info.id.0);
      // }

      let camera_position = camera_render_info.position;
      raw::uniform3f(
        shader.id,
        "camPos",
        camera_position.x,
        camera_position.y,
        camera_position.z,
      );

      // Pass uniforms
      raw::matrix4f(shader.id, "model", transform.matrix().as_ptr());
      raw::matrix4f(shader.id, "view", camera_render_info.view.as_ptr());
      raw::matrix4f(shader.id, "projection", camera_render_info.projection.as_ptr());

      // Bind Array Buffer
      ogl_mesh.render();
    }
  }
}
