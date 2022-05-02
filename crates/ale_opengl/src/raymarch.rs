use ale_mesh::Mesh;
use crate::mesh::{OpenGLMesh};
use crate::shader::{OpenGLShader};
use ale_shader::ale_shader_new;

pub struct OpenGLRaymarchContext {
  raymarch_shader: OpenGLShader,

  plane_mesh: OpenGLMesh,
}

pub fn ale_opengl_raymarch_context_new() -> OpenGLRaymarchContext {
  let raymarch_shader = OpenGLShader::new(&ale_shader_new(
    include_str!("../../../resources/shaders/raymarch.vert").to_owned(),
    include_str!("../../../resources/shaders/raymarch.frag").to_owned(),
  ))
    .unwrap();

  let plane_mesh = OpenGLMesh::new(&Mesh::new_plane()).unwrap();

  OpenGLRaymarchContext {
    raymarch_shader,
    plane_mesh,
  }
}

pub fn ale_opengl_raymarch_render(opengl_raymarch_context: &OpenGLRaymarchContext) {}
