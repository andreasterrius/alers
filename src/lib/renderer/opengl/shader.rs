use crate::renderer::opengl::raw::{CreateShaderError, create_shader};
use crate::resource::shader::ShaderFile;
use cgmath::{Vector3, Vector4};

#[derive(Debug)]
pub enum ShaderError {
  CompilationError(CreateShaderError)
}

impl From<CreateShaderError> for ShaderError {
  fn from(error: CreateShaderError) -> Self {
    ShaderError::CompilationError(error)
  }
}

pub struct ShaderDrawInfo {
  pub shader: u32,
}

impl ShaderDrawInfo {
  pub fn new(shader: &ShaderFile) -> Result<ShaderDrawInfo, ShaderError> {
    let shader = unsafe { create_shader(&shader.vertex_shader, &shader.fragment_shader)? };
    Ok(ShaderDrawInfo { shader })
  }
}

pub struct ShaderVariable {
  pub name: String,
  pub variable_type: ShaderVariableType,
}

impl ShaderVariable {
  pub fn new(name: String, variable_type: ShaderVariableType) -> ShaderVariable {
    ShaderVariable {
      name,
      variable_type,
    }
  }
}

pub enum ShaderVariableType {
  F32_3(Vector3<f32>),
  F32_4(Vector4<f32>),
}