use crate::raw;
use crate::raw::{create_buffer, CreateBufferError};
use crate::resource_pile::{OpenGLResourceLoader, OpenGLResourcePile, OpenGLResourceRouter, OpenGLResourceType};
use crate::route_loader;
use ale_autoid::Identifiable;
use ale_mesh::{Mesh, MeshId};
use std::collections::HashMap;
use std::ops::Deref;

pub struct OpenGLMeshId(pub u32);

pub struct OpenGLMeshContext {
  pub(crate) mesh: HashMap<MeshId, OpenGLMesh>,
}

impl OpenGLMeshContext {
  pub fn new() -> OpenGLMeshContext {
    OpenGLMeshContext { mesh: HashMap::new() }
  }

  pub fn register(&mut self, mesh: &Mesh) -> &OpenGLMesh {
    self.mesh.entry(mesh.uid()).or_insert(OpenGLMesh::new(mesh).unwrap())
  }
}

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

  pub fn render(&self) {
    unsafe {
      raw::bind_vao(self.vao);
      match self.ebo {
        None => raw::draw_arrays(0, self.draw_size),
        Some(_) => raw::draw_elements(self.draw_size),
      }
    }
  }
}

impl OpenGLResourceType for OpenGLMesh {}

#[derive(Debug)]
pub enum OpenGLMeshError {
  CreateBufferError(CreateBufferError),
}

impl From<CreateBufferError> for OpenGLMeshError {
  fn from(error: CreateBufferError) -> Self {
    OpenGLMeshError::CreateBufferError(error)
  }
}

pub struct OpenGLMeshLoader;

impl OpenGLResourceLoader<Mesh, OpenGLMesh> for OpenGLMeshLoader {
  fn create(&self, opengl_resource_pile: &OpenGLResourcePile, before: &Mesh) -> OpenGLMesh {
    OpenGLMesh::new(before).unwrap()
  }
}
route_loader!(OpenGLMeshLoader, Mesh);
