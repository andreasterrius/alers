use ale_autoid::*;

pub struct Shader {
  pub id: ShaderId,
  pub vertex_shader: String,
  pub fragment_shader: String,
}

pub fn ale_shader_new(vertex_shader: String, fragment_shader: String) -> Shader {
  Shader {
    id: ShaderId::new(),
    vertex_shader,
    fragment_shader,
  }
}

struct_id!(ShaderId);
struct_id_impl!(ShaderId, Shader, id);
