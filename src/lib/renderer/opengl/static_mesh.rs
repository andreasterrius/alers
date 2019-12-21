use crate::resource::static_mesh::StaticMesh;
use crate::renderer::opengl::raw::{CreateBufferError, create_buffer};

pub struct StaticMeshDrawInfo {
  pub vao: u32,
  pub _vbo: u32,
  pub ebo: Option<u32>,
  pub draw_size: u32, //indices size, or vertex size
}

impl StaticMeshDrawInfo {
  pub fn new(mesh: &StaticMesh) -> Result<StaticMeshDrawInfo, StaticMeshError> {
    let (vao, vbo, ebo, draw_size) = unsafe { create_buffer(&mesh.vertices, &mesh.indices)? };
    Ok(StaticMeshDrawInfo { vao, _vbo: vbo, ebo, draw_size })
  }
}

#[derive(Debug)]
pub enum StaticMeshError {
  CreateBufferError(CreateBufferError)
}

impl From<CreateBufferError> for StaticMeshError {
  fn from(error: CreateBufferError) -> Self {
    StaticMeshError::CreateBufferError(error)
  }
}