use ale_math::transform::Transform;
use crate::renderer::opengl::shader::ShaderVariable;
use crate::resource::shader::ShaderFileId;
use ale_mesh::MeshId;
use ale_texture::TextureId;

pub struct PawnEntity {
  pub transform: Transform,
  pub static_mesh_id: MeshId,
  pub shader_id: ShaderFileId,
  pub textures: Vec<TextureId>,
  pub shader_variables: Vec<ShaderVariable>,
}
