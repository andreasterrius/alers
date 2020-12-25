use crate::buffer::{Buffer, BufferBuilder, SeparateBufferBuilder};
use ale_autoid::*;
use ale_ecs::{Entity, EntityBuilder};

pub mod buffer;

#[derive(Debug)]
pub struct Mesh {
  pub id: MeshId,
  pub vertices: Buffer<f32>,
  pub indices: Option<Buffer<i32>>,
}

struct_id!(MeshId);
struct_id_impl!(MeshId, Mesh, id);

impl Mesh {
  pub fn new(vertices: Buffer<f32>, indices: Option<Buffer<i32>>) -> Mesh {
    Mesh {
      id: MeshId::new(),
      vertices,
      indices,
    }
  }

  pub fn new_cube() -> Mesh {
    let vertices = BufferBuilder::new(vec![
      // back face
      -1.0f32, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // bottom-left
      1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, // top-right
      1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 0.0, // bottom-right
      1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, // top-right
      -1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // bottom-left
      -1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0, // top-left
      // front face
      -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom-left
      1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, // bottom-right
      1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // top-right
      1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // top-right
      -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, // top-left
      -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom-left
      // left face
      -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // top-right
      -1.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, 1.0, // top-left
      -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // bottom-left
      -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // bottom-left
      -1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, // bottom-right
      -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // top-right
      // right face
      1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, // top-left
      1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, // bottom-right
      1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top-right
      1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, // bottom-right
      1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, // top-left
      1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, // bottom-left
      // bottom face
      -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, // top-right
      1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 1.0, 1.0, // top-left
      1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, // bottom-left
      1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, // bottom-left
      -1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, // bottom-right
      -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, // top-right
      // top face
      -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top-left
      1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom-right
      1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0, // top-right
      1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom-right
      -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top-left
      -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, // bottom-left
    ])
    .info("vertex", 3)
    .info("normal", 3)
    .info("uv", 2)
    .build()
    .unwrap();

    Mesh::new(vertices, None)
  }

  pub fn new_plane() -> Mesh {
    let vertices = BufferBuilder::new(vec![0.0f32, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0])
      .info("vertex", 2)
      .build()
      .unwrap();

    Mesh::new(vertices, None)
  }

  pub fn new_ndc_plane() -> Mesh {
    let vertices = BufferBuilder::new(vec![
      -1.0f32, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 0.0, 1.0,
      1.0, 1.0, 1.0,
    ])
    .info("vertex", 2)
    .info("texcoords", 2)
    .build()
    .unwrap();

    Mesh::new(vertices, None)
  }
}

pub trait MeshComponent {
  fn with_mesh(&mut self, mesh: &Mesh);
}
