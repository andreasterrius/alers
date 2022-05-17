use crate::raw;
use crate::raw::{create_buffer, CreateBufferError};
use ale_resources::mesh::Mesh;
use thiserror::Error;

pub struct OpenGLMesh {
  pub vao: u32,
  pub _vbo: u32,
  pub ebo: Option<u32>,
  pub draw_size: u32, //indices size, or vertex size
}

impl OpenGLMesh {
  pub fn new(mesh: &Mesh) -> Result<OpenGLMesh, OpenGLMeshError> {
    let (vao, vbo, ebo, draw_size) = unsafe { create_buffer(&mesh.vertices, &mesh.indices)? };
    Ok(OpenGLMesh {
      vao,
      _vbo: vbo,
      ebo,
      draw_size,
    })
  }

  pub fn activate(&self) {
    unsafe {
      raw::bind_vao(self.vao);
    }
  }

  pub fn draw(&self) {
    unsafe {
      match self.ebo {
        None => raw::draw_arrays(0, self.draw_size),
        Some(_) => raw::draw_elements(self.draw_size),
      }
    }
  }
}

#[derive(Error, Debug)]
pub enum OpenGLMeshError {
  #[error("(OpenGLMeshError::CreateBufferError)")]
  CreateBufferError(#[from] CreateBufferError),
}
