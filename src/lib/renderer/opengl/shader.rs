use crate::renderer::opengl::raw::{CreateShaderError, create_shader};
use crate::resource::shader::ShaderFile;

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