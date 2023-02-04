use std::{fs, io};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use log::info;
use thiserror::Error;

use crate::{struct_id, struct_id_impl};
use crate::shader::LoadError::ShaderSourceIOError;
use crate::stash::{Load, Stash};

const WGSL_EXT: &str = "wgsl";
const VS_MAIN: &str = "vs_main";
const FS_MAIN: &str = "fs_main";
const GS_MAIN: &str = "gs_main";

pub struct ShaderStash {
  pub glsl_stash: Stash<GLSLShader, LoadError, GLSLLoader>,
  pub wgsl_stash: Stash<WGSLShader, LoadError, WGSLLoader>,
  /* TODO: You can implement custom loader in this struct */
}

impl ShaderStash {
  pub fn new() -> ShaderStash {
    ShaderStash { glsl_stash: Stash::new(), wgsl_stash: Stash::new() }
  }
}

pub struct GLSLShader {
  pub id: ShaderId,
  pub vertex_shader: String,
  pub fragment_shader: String,
  pub geometry_shader: Option<String>,
}

impl GLSLShader {
  pub fn new(vertex_shader: String, fragment_shader: String) -> GLSLShader {
    GLSLShader {
      id: ShaderId::new(),
      vertex_shader,
      fragment_shader,
      geometry_shader: None,
    }
  }

  pub fn new_geom(vertex_shader: String, fragment_shader: String, geometry_shader: String) -> GLSLShader {
    GLSLShader {
      id: ShaderId::new(),
      vertex_shader,
      fragment_shader,
      geometry_shader: Some(geometry_shader),
    }
  }
}

pub struct WGSLShader {
  pub id: ShaderId,
  pub source: String,
  pub vert_entry: String,
  pub frag_entry: String,
  pub geom_entry: String,
}

impl WGSLShader {
  pub fn new(source: String,
             vert_entry: String,
             frag_entry: String,
             geom_entry: String) -> WGSLShader {
    WGSLShader {
      id: ShaderId::new(),
      source,
      vert_entry,
      frag_entry,
      geom_entry,
    }
  }
}

struct_id!(ShaderId);
struct_id_impl!(ShaderId, GLSLShader, id);
struct_id_impl!(ShaderId, WGSLShader, id);

#[derive(Error, Debug)]
pub enum LoadError {
  #[error("(LoadError::ShaderSourceIOError)\n\
      Path: {}\n\
      Shader: {}\n\
      Error: {}", .2, .1, .0)]
  ShaderSourceIOError(io::Error, String, String),
}

pub struct GLSLLoader;

impl GLSLLoader {}

impl Load<GLSLShader, LoadError> for GLSLLoader {
  fn load(&self, path: &str) -> Result<Vec<GLSLShader>, LoadError> {
    let vertex_shader_path = PathBuf::from_str(&format!("{}.vert", path)).unwrap();
    let fragment_shader_path = PathBuf::from_str(&format!("{}.frag", path)).unwrap();
    let geom_shader_path = PathBuf::from_str(&format!("{}.geom", path)).unwrap();

    info!("glsl_load, vertex: {}, fragment: {}, geom: {}",
      vertex_shader_path.to_str().unwrap_or("None"),
      fragment_shader_path.to_str().unwrap_or("None"),
      geom_shader_path.to_str().unwrap_or("None")
    );

    let vertex_shader = match fs::read_to_string(vertex_shader_path.clone()) {
      Ok(str) => str,
      Err(err) => {
        return Err(ShaderSourceIOError(
          err,
          "vertex shader".to_owned(),
          vertex_shader_path.to_str().unwrap().to_owned(),
        ));
      }
    };
    let fragment_shader = match fs::read_to_string(fragment_shader_path.clone()) {
      Ok(str) => str,
      Err(err) => {
        return Err(ShaderSourceIOError(
          err,
          "fragment shader".to_owned(),
          fragment_shader_path.to_str().unwrap().to_owned(),
        ));
      }
    };
    let geom_shader = match fs::read_to_string(geom_shader_path) {
      Ok(str) => Some(str),
      Err(err) => None,
    };

    match geom_shader {
      None => Ok(vec![GLSLShader::new(vertex_shader, fragment_shader)]),
      Some(geom_shader) => Ok(vec![GLSLShader::new_geom(vertex_shader, fragment_shader, geom_shader)]),
    }
  }
}

impl Default for GLSLLoader {
  fn default() -> Self {
    GLSLLoader
  }
}

pub struct WGSLLoader;

impl WGSLLoader {}

impl Load<WGSLShader, LoadError> for WGSLLoader {
  //TODO: Support split wgsl files, just do a try read
  fn load(&self, path: &str) -> Result<Vec<WGSLShader>, LoadError> {
    let shader_path = PathBuf::from_str(&format!("{}.{}", path, WGSL_EXT)).unwrap();

    info!("wgsl_shader_load, wgsl: {}", shader_path.to_str().unwrap_or("None"));
    let shader_source = match fs::read_to_string(shader_path.clone()) {
      Ok(str) => str,
      Err(err) => {
        return Err(ShaderSourceIOError(
          err,
          "wgsl shader".to_owned(),
          shader_path.to_str().unwrap().to_owned(),
        ));
      }
    };

    Ok(vec![WGSLShader::new(
      shader_source,
      VS_MAIN.to_owned(),
      FS_MAIN.to_owned(),
      GS_MAIN.to_owned())])
  }
}

impl Default for WGSLLoader {
  fn default() -> Self {
    WGSLLoader
  }
}