use crate::raw::{create_buffer, CreateBufferError};
use ale_mesh::{ale_mesh_cube_new, ale_mesh_plane_new, Mesh, MeshId};
use std::collections::HashMap;

pub struct OpenGLMeshContext {
  pub(crate) mesh: HashMap<MeshId, OpenGLMesh>,

  pub plane_opengl_mesh: OpenGLMesh,
  pub cube_opengl_mesh: OpenGLMesh,
}

pub fn ale_opengl_mesh_context_new() -> OpenGLMeshContext {
  OpenGLMeshContext {
    mesh: HashMap::new(),
    plane_opengl_mesh: ale_opengl_mesh_new(&ale_mesh_plane_new()).unwrap(),
    cube_opengl_mesh: ale_opengl_mesh_new(&ale_mesh_cube_new()).unwrap(),
  }
}

pub struct OpenGLMesh {
  pub vao: u32,
  pub _vbo: u32,
  pub ebo: Option<u32>,
  pub draw_size: u32, //indices size, or vertex size
}

pub fn ale_opengl_mesh_new(mesh: &Mesh) -> Result<OpenGLMesh, OpenGLMeshError> {
  let (vao, vbo, ebo, draw_size) = unsafe { create_buffer(&mesh.vertices, &mesh.indices)? };
  Ok(OpenGLMesh {
    vao,
    _vbo: vbo,
    ebo,
    draw_size,
  })
}

#[derive(Debug)]
pub enum OpenGLMeshError {
  CreateBufferError(CreateBufferError),
}

impl From<CreateBufferError> for OpenGLMeshError {
  fn from(error: CreateBufferError) -> Self {
    OpenGLMeshError::CreateBufferError(error)
  }
}
