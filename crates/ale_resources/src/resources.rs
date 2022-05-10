use crate::font::Font;
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::texture::Texture;
use ale_data::alevec::AleVec;

pub struct Resources {
  pub meshes: AleVec<Mesh>,
  pub textures: AleVec<Texture>,
  pub shaders: AleVec<Shader>,
  pub fonts: AleVec<Font>,
}

impl Resources {
  pub fn new() -> Resources {
    Resources {
      meshes: AleVec::new(),
      textures: AleVec::new(),
      shaders: AleVec::new(),
      fonts: AleVec::new(),
    }
  }
}
