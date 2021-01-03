use crate::mesh::{ale_opengl_mesh_new, OpenGLMesh};
use crate::shader::{ale_opengl_shader_new, OpenGLShader};
use ale_mesh::{ale_mesh_plane_new, Mesh};
use ale_shader::ale_shader_new;

pub struct OpenGLRaymarchContext {
  raymarch_shader: OpenGLShader,

  plane_mesh: OpenGLMesh,
}

pub fn ale_opengl_raymarch_new() -> OpenGLRaymarchContext {
  let raymarch_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../../../resources/shaders/raymarch.vert").to_owned(),
    include_str!("../../../resources/shaders/raymarch.frag").to_owned(),
  ))
  .unwrap();

  let plane_mesh = ale_opengl_mesh_new(&ale_mesh_plane_new()).unwrap();

  OpenGLRaymarchContext {
    raymarch_shader,
    plane_mesh,
  }
}

pub fn ale_opengl_raymarch_render(opengl_raymarch_context : &OpenGLRaymarchContext) {

}