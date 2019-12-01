use resource::static_mesh::StaticMesh;
use resource::shader::ShaderFile;

pub mod fbx;
pub mod fbx_convert;
pub mod shader;
pub mod static_mesh;
pub mod resources;

pub trait ResourceEventObserver {
  fn on_static_mesh_loaded(&mut self, meshes : &Vec<StaticMesh>);

  fn on_shader_loaded(&mut self, shader: &ShaderFile);
}