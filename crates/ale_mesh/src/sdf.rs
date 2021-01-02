use crate::iter::ale_mesh_triangle_iter_new;
use crate::Mesh;

pub struct MeshSDF {
  dist: Vec<Vec<f32>>,
}

pub fn ale_mesh_sdf_new(mesh: &Mesh) -> MeshSDF {
  for tris in ale_mesh_triangle_iter_new(mesh) {}

  MeshSDF { dist: vec![] }
}
