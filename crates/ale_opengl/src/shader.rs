use crate::raw::{create_shader, CreateShaderError};
use ale_math::{Vector3, Vector4};
use ale_shader::{ale_shader_new, Shader};
use std::collections::HashMap;

pub struct OpenGLShaderId(pub u32);

pub struct OpenGLShaderContext {
  pub(crate) shader: HashMap<OpenGLShaderId, OpenGLShader>,

  pub(crate) text_2d_shader: OpenGLShader,
}

pub fn ale_opengl_shader_context_new() -> OpenGLShaderContext {
  let flat_2d_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../resources/text_2d.vert").to_owned(),
    include_str!("../resources/text_2d.frag").to_owned(),
  ))
  .unwrap();

  OpenGLShaderContext {
    shader: HashMap::new(),
    text_2d_shader: flat_2d_shader,
  }
}

pub struct OpenGLShader {
  pub id: u32,
}

pub fn ale_opengl_shader_new(shader: &Shader) -> Result<OpenGLShader, OpenGLShaderError> {
  let shader = unsafe { create_shader(&shader.vertex_shader, &shader.fragment_shader)? };
  Ok(OpenGLShader { id: shader })
}

#[derive(Clone)]
pub struct OpenGLShaderVariable {
  pub name: String,
  pub opengl_shader_variable_type: OpenGLShaderVariableType,
}

pub fn ale_opengl_shader_variable_new(
  name: String,
  opengl_shader_variable_type: OpenGLShaderVariableType,
) -> OpenGLShaderVariable {
  OpenGLShaderVariable {
    name,
    opengl_shader_variable_type,
  }
}

#[derive(Clone)]
pub enum OpenGLShaderVariableType {
  F32_1(f32),
  F32_3(Vector3<f32>),
  F32_4(Vector4<f32>),
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
