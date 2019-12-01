use resource::static_mesh::StaticMesh;

pub mod fbx;
pub mod fbx_convert;
pub mod shader;
pub mod static_mesh;
pub mod resources;

trait ResourceEventListener {
  fn on_static_mesh_loaded(&mut self, meshes : &Vec<StaticMesh>);
}