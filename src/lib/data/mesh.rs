use data::buffer::Buffer;

pub struct SimpleStaticMesh {
  vertices : Buffer<f64>,
  elements : Buffer<i32>,
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