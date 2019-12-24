use crate::data::id::Id;
use crate::math::transform::Transform;
use crate::resource::shader::ShaderFile;
use crate::resource::static_mesh::StaticMesh;
use crate::data::id::Identifiable;
use crate::renderer::opengl::shader::ShaderVariable;

pub struct PawnEntity {
  pub transform: Transform,
  pub static_mesh_id: Id,
  pub shader_id: Id,
  pub textures: Vec<Id>,
  pub shader_variables : Vec<ShaderVariable>,
}
