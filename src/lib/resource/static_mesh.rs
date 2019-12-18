use data::buffer::{Buffer, BufferBuilder};
use data::id::Id;

#[derive(Debug)]
pub struct StaticMesh {
  pub id: Id,
  pub vertices: Buffer<f32>,
  pub indices: Option<Buffer<i32>>,
}

impl StaticMesh {
  pub fn new(vertices: Buffer<f32>, indices: Option<Buffer<i32>>) -> StaticMesh {
    StaticMesh {
      id: Id::new(),
      vertices,
      indices,
    }
  }
}

impl_id!(StaticMesh, id);


pub fn create_cube() -> StaticMesh {
  let vertices = BufferBuilder::new(vec!(
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
    -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0  // bottom-left
  )).info("vertex", 3)
    .info("normal", 3)
    .info("uv", 2)
    .build()
    .unwrap();

  StaticMesh::new(vertices, None)
}

/*
*/