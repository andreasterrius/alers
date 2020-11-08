use ale_math::transform::Transform;
use crate::renderer::opengl::shader::ShaderVariable;
use crate::resource::shader::ShaderFileId;
use crate::resource::texture::TextureId;
use ale_mesh::MeshId;

pub struct PawnEntity {
  pub transform: Transform,
  pub static_mesh_id: MeshId,
  pub shader_id: ShaderFileId,
  pub textures: Vec<TextureId>,
  pub shader_variables: Vec<ShaderVariable>,
}
