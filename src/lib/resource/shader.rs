pub struct ShaderFile {
  pub id: ShaderFileId,
  pub vertex_shader: String,
  pub fragment_shader: String,
}

impl ShaderFile {
  pub fn new(vertex_shader: String, fragment_shader: String) -> ShaderFile {
    ShaderFile {
      id: ShaderFileId::new(),
      vertex_shader,
      fragment_shader,
    }
  }
}

struct_id!(ShaderFileId);
struct_id_impl!(ShaderFileId, ShaderFile, id);
