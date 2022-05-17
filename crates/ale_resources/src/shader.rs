use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fs, io};

use thiserror::Error;

use crate::shader::LoadError::ShaderSourceIOError;
use crate::stash::{Load, Stash};
use crate::{struct_id, struct_id_impl};

pub struct ShaderStash {
  pub stash: Stash<Shader, LoadError, Loader>,
  /* TODO: You can implement custom loader in this struct */
}

impl ShaderStash {
  pub fn new() -> ShaderStash {
    ShaderStash { stash: Stash::new() }
  }
}

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

#[derive(Error, Debug)]
pub enum LoadError {
  #[error("{} shader source io error: {:?}", .1, .0)]
  ShaderSourceIOError(io::Error, String),
}

pub struct Loader;
impl Load<Shader, LoadError> for Loader {
  fn load(&self, path: &str) -> Result<Vec<Shader>, LoadError> {
    let vertex_shader_path = PathBuf::from_str(&format!("{}.vert", path)).unwrap();
    let fragment_shader_path = PathBuf::from_str(&format!("{}.frag", path)).unwrap();
    let geom_shader_path = PathBuf::from_str(&format!("{}.geom", path)).unwrap();

    let vertex_shader = match fs::read_to_string(vertex_shader_path) {
      Ok(str) => str,
      Err(err) => return Err(ShaderSourceIOError(err, "vertex shader".to_owned())),
    };
    let fragment_shader = match fs::read_to_string(fragment_shader_path) {
      Ok(str) => str,
      Err(err) => return Err(ShaderSourceIOError(err, "fragment shader".to_owned())),
    };
    let geom_shader = match fs::read_to_string(geom_shader_path) {
      Ok(str) => Some(str),
      Err(err) => None,
    };

    match geom_shader {
      None => Ok(vec![Shader::new(vertex_shader, fragment_shader)]),
      Some(geom_shader) => Ok(vec![Shader::new_geom(vertex_shader, fragment_shader, geom_shader)]),
    }
  }
}

impl Default for Loader {
  fn default() -> Self {
    Loader
  }
}
