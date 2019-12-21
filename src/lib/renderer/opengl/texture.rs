use crate::renderer::opengl::raw::{CreateTextureError, create_texture};
use crate::resource::texture::Texture;

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
    Ok(TextureDrawInfo { texture })
  }
}
