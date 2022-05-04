use crate::{struct_id, struct_id_impl};

pub struct Shader {
  pub id: ShaderId,
  pub vertex_shader: String,
  pub fragment_shader: String,
  pub geometry_shader: Option<String>,
}

impl Shader {
  pub fn new(vertex_shader: String, fragment_shader: String) -> Shader {
    Shader {
      id: ShaderId::new(),
      vertex_shader,
      fragment_shader,
      geometry_shader: None,
    }
  }

  pub fn new_geom(vertex_shader: String, fragment_shader: String, geometry_shader: String) -> Shader {
    Shader {
      id: ShaderId::new(),
      vertex_shader,
      fragment_shader,
      geometry_shader: Some(geometry_shader),
    }
  }
}

struct_id!(ShaderId);
struct_id_impl!(ShaderId, Shader, id);
