use image::{ImageError, GenericImageView};

use data::id::Id;

pub struct Texture {
  id: Id,
  data: Vec<u8>,
  width: u32,
  height: u32,
}

impl Texture {
  pub fn new(data: Vec<u8>, width: u32, height: u32) -> Texture {
    Texture {
      id: Id::new(),
      data,
      width,
      height
    }
  }

  pub fn load(path : &str) -> Result<Texture, LoadTextureError>{
    let i = image::open(path)?;

    // TODO: i.raw_pixels() clones the underlying bytes
    Ok(Texture::new(i.raw_pixels(), i.width(), i.height()))
  }

  pub fn as_ptr(&self) -> *const u8{
    &self.data[0]
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }
}

impl_id!(Texture, id);

#[derive(Debug)]
pub enum LoadTextureError {
  ImageError(ImageError)
}

impl From<ImageError> for LoadTextureError {
  fn from(e: ImageError) -> Self {
    LoadTextureError::ImageError(e)
  }
}
