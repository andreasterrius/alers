use crate::Mesh;

pub struct MeshSDF {
  dist: Vec<Vec<f32>>,
}

pub fn ale_mesh_sdf_new(mesh: &Mesh) -> MeshSDF {
  for v in mesh.vertices.element_iter("vertex") {}

  MeshSDF { dist: vec![] }
}
