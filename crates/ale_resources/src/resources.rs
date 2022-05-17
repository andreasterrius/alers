use std::any::TypeId;

use ale_data::alevec::AleVec;

use crate::{font, mesh, texture};
use crate::font::Font;
use crate::mesh::Mesh;
use crate::shader::{Shader, ShaderStash};
use crate::stash::Stash;
use crate::texture::Texture;

pub struct Resources {
  pub meshes: Stash<Mesh, mesh::LoadError, mesh::Loader>,
  pub textures: Stash<Texture, texture::LoadError, texture::Loader>,
  pub shaders: ShaderStash,
  pub fonts: Stash<Font, font::LoadError, font::Loader>,
}

impl Resources {
  pub fn new() -> Resources {
    Resources {
      meshes: Stash::new(),
      textures: Stash::new(),
      shaders: ShaderStash::new(),
      fonts: Stash::new(),
    }
  }
}
