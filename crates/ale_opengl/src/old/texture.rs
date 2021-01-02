use crate::raw::{create_texture, CreateTextureError};
use ale_texture::Texture;

pub struct TextureDrawInfo {
  pub texture: u32,
}

impl TextureDrawInfo {
  pub fn new(texture: &Texture) -> Result<TextureDrawInfo, CreateTextureError> {
    let texture = unsafe { create_texture(texture)? };
    Ok(TextureDrawInfo { texture: texture.0 })
  }
}
