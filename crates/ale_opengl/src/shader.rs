use crate::raw;
use crate::raw::{create_shader, CreateShaderError};
use ale_math::{Vector3, Vector4};
use ale_shader::Shader;
use ale_variable::Variable;
use std::collections::HashMap;

pub struct OpenGLShaderId(pub u32);

pub struct OpenGLShaderContext {
  pub(crate) shader: HashMap<OpenGLShaderId, OpenGLShader>,
}

impl OpenGLShaderContext {
  pub fn new() -> OpenGLShaderContext {
    OpenGLShaderContext { shader: HashMap::new() }
  }
}

pub struct OpenGLShader {
  pub id: u32,
}

impl OpenGLShader {
  pub fn new(shader: &Shader) -> Result<OpenGLShader, OpenGLShaderError> {
    let shader = unsafe { create_shader(&shader.vertex_shader, &shader.fragment_shader)? };
    Ok(OpenGLShader { id: shader })
  }

  pub fn activate(&self, shader_vars: &Vec<Variable>) {
    unsafe {
      raw::use_shader(self.id);

      for shader_variable in shader_vars {
        match shader_variable {
          Variable::F32_1(name, ff) => raw::uniform1f(self.id, &name, *ff),
          Variable::F32_3(name, vec) => raw::uniform3f(self.id, &name, vec.x, vec.y, vec.z),
          Variable::F32_4(name, vec) => raw::uniform4f(self.id, &name, vec.x, vec.y, vec.z, vec.w),
          Variable::Bool(name, ff) => raw::uniform1i(self.id, &name, if *ff { 1 } else { 0 }),
          Variable::Void(_) => {}
        }
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
