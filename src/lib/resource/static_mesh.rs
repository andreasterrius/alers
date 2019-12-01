use data::buffer::Buffer;
use data::id::Id;
#[derive(Debug)]
pub struct StaticMesh {
  pub id: Id,
  pub vertices: Buffer<f32>,
  pub indices: Option<Buffer<i32>>,
}

impl StaticMesh {
  pub fn new(vertices : Buffer<f32>, indices : Option<Buffer<i32>>) -> StaticMesh {
    StaticMesh {
      id: Id::new(),
      vertices,
      indices,
    }
  }
}

impl_id!(StaticMesh, id);