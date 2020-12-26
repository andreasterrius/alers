use ale_autoid::*;
use ale_resource::{Resource, ResourcePile};
use std::fs;
use std::fs::File;
use std::io::Read;

pub struct Shader {
  pub id: ShaderId,
  pub vertex_shader: String,
  pub fragment_shader: String,
}

impl Shader {
  pub fn new(vertex_shader: String, fragment_shader: String) -> Shader {
    Shader {
      id: ShaderId::new(),
      vertex_shader,
      fragment_shader,
    }
  }
}

struct_id!(ShaderId);
struct_id_impl!(ShaderId, Shader, id);

pub trait ShaderLoader {
  fn load_shader(&mut self, vs_path: &str, fs_path: &str) -> Resource<Shader>;
}

impl ShaderLoader for ResourcePile {
  fn load_shader(&mut self, vs_path: &str, fs_path: &str) -> Resource<Shader> {
    let mut vs_path = self.get_resource_path(vs_path);
    let mut fs_path = self.get_resource_path(fs_path);

    let vs_c = fs::read_to_string(vs_path).unwrap();
    let fs_c = fs::read_to_string(fs_path).unwrap();

    let shader = Shader::new(vs_c, fs_c);
    self.register(shader)
  }
}
