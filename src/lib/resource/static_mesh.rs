use data::buffer::Buffer;
use data::id::{Id};

#[derive(Debug)]
pub struct StaticMesh {
  pub id : Id,
  pub vertices : Buffer<f32>,
  pub indices: Option<Buffer<i32>>,
}

impl StaticMesh {
}

pub struct SimpleStaticMeshBuilder {
}

impl SimpleStaticMeshBuilder {
  pub fn new () -> SimpleStaticMeshBuilder {
    SimpleStaticMeshBuilder {

    }
  }
}

impl_id!(StaticMesh, id);