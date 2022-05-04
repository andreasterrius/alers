use ale_resources::texture::Texture;

use crate::raw::{create_texture, CreateTextureError};

pub struct TextureDrawInfo {
  pub texture: u32,
}

impl TextureDrawInfo {
  pub fn new(texture: &Texture) -> Result<TextureDrawInfo, CreateTextureError> {
    let texture = unsafe { create_texture(texture)? };
    Ok(TextureDrawInfo { texture: texture.0 })
  }
}
