use resource::shader::ShaderFile;
use resource::static_mesh::StaticMesh;
use math::transform::Transform;

pub struct Object<'a> {
  pub transform : Transform,
  pub mesh : &'a StaticMesh,
  pub shader : &'a ShaderFile
}