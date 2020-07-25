use crate::data::id::Id;
use crate::math::transform::Transform;
use crate::renderer::opengl::shader::ShaderVariable;

pub struct PawnEntity {
  pub transform: Transform,
  pub static_mesh_id: Id,
  pub shader_id: Id,
  pub textures: Vec<Id>,
  pub shader_variables: Vec<ShaderVariable>,
}
