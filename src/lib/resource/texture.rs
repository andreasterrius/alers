use image::{GenericImageView, ImageError};

use data::id::Id;

pub struct Texture {
  id: Id,
  data: Vec<u8>,
  width: u32,
  height: u32,

  texture_wrap: TextureWrap,
  texture_magnification: TextureMagnification,
}

impl Texture {
  pub fn new(data: Vec<u8>, width: u32, height: u32) -> Texture {
    Texture {
      id: Id::new(),
      data,
      width,
      height,
      texture_wrap: TextureWrap { x: TextureWrapType::Repeat, y: TextureWrapType::Repeat },
      texture_magnification: TextureMagnification { min: TextureMagnificationType::Linear, max: TextureMagnificationType::Linear }
    }
  }

  pub fn load(path: &str) -> Result<Texture, LoadTextureError> {
    let i = image::open(path)?;

    // TODO: i.raw_pixels() clones the underlying bytes
    Ok(Texture::new(i.raw_pixels(), i.width(), i.height()))
  }

  pub fn as_ptr(&self) -> *const u8 {
    &self.data[0]
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn get_wrap(&self) -> &TextureWrap {
    &self.texture_wrap
  }

  pub fn get_magnification(&self) -> &TextureMagnification {
    &self.texture_magnification
  }
}

impl_id!(Texture, id);

pub struct TextureWrap {
  pub x : TextureWrapType,
  pub y : TextureWrapType,
}

pub enum TextureWrapType {
  ClampToEdge,
  MirroredRepeat,
  Repeat,
}

pub struct TextureMagnification {
  pub min : TextureMagnificationType,
  pub max : TextureMagnificationType,
}

pub enum TextureMagnificationType {
  Nearest,
  Linear,
}


#[derive(Debug)]
pub enum LoadTextureError {
  ImageError(ImageError)
}

impl From<ImageError> for LoadTextureError {
  fn from(e: ImageError) -> Self {
    LoadTextureError::ImageError(e)
  }
}

