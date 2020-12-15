use crate::raw;
use crate::raw::{create_shader, CreateShaderError};
use ale_math::{Vector3, Vector4};
use ale_shader::{ale_shader_new, Shader};
use ale_variable::Variable;
use std::collections::HashMap;

pub struct OpenGLShaderId(pub u32);

pub struct OpenGLShaderContext {
  pub(crate) shader: HashMap<OpenGLShaderId, OpenGLShader>,

  // Use this to render text
  pub(crate) text_2d_shader: OpenGLShader,

  // Use this to render render frames
  pub(crate) render_frame_shader: OpenGLShader,
}

pub fn ale_opengl_shader_context_new() -> OpenGLShaderContext {
  let text_2d_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../resources/text_2d.vert").to_owned(),
    include_str!("../resources/text_2d.frag").to_owned(),
  ))
  .unwrap();

  let render_frame_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../resources/fxaa.vert").to_owned(),
    include_str!("../resources/fxaa.frag").to_owned(),
  ))
  .unwrap();

  OpenGLShaderContext {
    shader: HashMap::new(),
    text_2d_shader,
    render_frame_shader,
  }
}

pub struct OpenGLShader {
  pub id: u32,
}

pub fn ale_opengl_shader_new(shader: &Shader) -> Result<OpenGLShader, OpenGLShaderError> {
  let shader = unsafe { create_shader(&shader.vertex_shader, &shader.fragment_shader)? };
  Ok(OpenGLShader { id: shader })
}

pub fn ale_opengl_shader_activate(shader: &OpenGLShader, shader_vars: &Vec<Variable>) {
  unsafe {
    raw::use_shader(shader.id);

    for shader_variable in shader_vars {
      match shader_variable {
        Variable::F32_1(name, ff) => raw::uniform1f(shader.id, &name, *ff),
        Variable::F32_3(name, vec) => raw::uniform3f(shader.id, &name, vec.x, vec.y, vec.z),
        Variable::F32_4(name, vec) => raw::uniform4f(shader.id, &name, vec.x, vec.y, vec.z, vec.w),
        Variable::Bool(name, ff) => raw::uniform1i(shader.id, &name, if *ff { 1 } else { 0 }),
        Variable::Void(_) => {}
      }
    }
  }
}

#[derive(Debug)]
pub enum OpenGLShaderError {
  CompilationError(CreateShaderError),
}

impl From<CreateShaderError> for OpenGLShaderError {
  fn from(error: CreateShaderError) -> Self {
    OpenGLShaderError::CompilationError(error)
  }
}
