use ale_math::transform::AleTransform;
use ale_mesh::MeshId;
use ale_shader::ShaderId;
use ale_texture::TextureId;
use ale_variable::Variable;

pub struct PawnEntity {
  pub transform: AleTransform,
  pub static_mesh_id: MeshId,
  pub shader_id: ShaderId,
  pub textures: Vec<TextureId>,
  pub shader_variables: Vec<Variable>,
}
