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
}

#[derive(Error, Debug)]
pub enum OpenGLMeshError {
  #[error("create buffer error")]
  CreateBufferError(#[from] CreateBufferError),
}
