use ale_math::transform::Transform;
use ale_mesh::MeshId;
use ale_texture::TextureId;
use ale_opengl::shader::OpenGLShaderVariable;
use ale_shader::ShaderId;

pub struct PawnEntity {
  pub transform: Transform,
  pub static_mesh_id: MeshId,
  pub shader_id: ShaderId,
  pub textures: Vec<TextureId>,
  pub shader_variables: Vec<OpenGLShaderVariable>,
}
