use ale_opengl::raw::{create_texture, CreateTextureError};
use ale_texture::Texture;

#[derive(Debug)]
pub enum TextureError {
  CreateTextureError(CreateTextureError),
}

impl From<CreateTextureError> for TextureError {
  fn from(e: CreateTextureError) -> Self {
    TextureError::CreateTextureError(e)
  }
}

pub struct TextureDrawInfo {
  pub texture: u32,
}

impl TextureDrawInfo {
  pub fn new(texture: &Texture) -> Result<TextureDrawInfo, TextureError> {
    let texture = unsafe { create_texture(texture)? };
    Ok(TextureDrawInfo { texture: texture.0 })
  }
}
