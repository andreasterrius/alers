use data::buffer::Buffer;

#[derive(Debug)]
pub struct SimpleStaticMesh {
  pub vertices : Buffer<f64>,
  pub indices: Option<Buffer<i32>>,
}

impl SimpleStaticMesh {
}

pub struct SimpleStaticMeshBuilder {
}

impl SimpleStaticMeshBuilder {
  pub fn new () -> SimpleStaticMeshBuilder {
    SimpleStaticMeshBuilder {

    }
  }
}