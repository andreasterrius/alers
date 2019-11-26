use data::id::Id;

pub struct ShaderFile {
  id : Id,
  vertex_shader : String,
  fragment_shader : String,
}

impl ShaderFile {
  pub fn new(vertex_shader : String, fragment_shader : String) -> ShaderFile {
    ShaderFile {
      id: Id::new(),
      vertex_shader,
      fragment_shader
    }
  }
}

impl_id!(ShaderFile, id);

