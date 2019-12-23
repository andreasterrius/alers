use crate::data::id::Id;
use crate::math::transform::Transform;
use crate::resource::shader::ShaderFile;
use crate::resource::static_mesh::StaticMesh;
use crate::data::id::Identifiable;

pub struct PawnEntity {
  pub transform: Transform,
  pub static_mesh_id: Id,
  pub shader_id: Id,
  pub textures: Vec<Id>,
}
