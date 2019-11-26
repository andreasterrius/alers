use resource::shader::ShaderFile;
use resource::static_mesh::StaticMesh;

pub struct Object<'a> {
  pub mesh : &'a StaticMesh,
  pub shader : &'a ShaderFile
}