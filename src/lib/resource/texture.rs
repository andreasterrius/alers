use std::fs::File;

use image::{GenericImageView, ImageError};

use crate::data::id::Id;

pub struct Texture {
  id: Id,
  data: TexturePixel,
  width: u32,
  height: u32,
  channel_count: u32,

  texture_wrap: TextureWrap,
  texture_magnification: TextureMagnification,
}

impl Texture {
  pub fn new(data: TexturePixel, width: u32, height: u32, channel_count: u32, ) -> Texture {
    Texture {
      id: Id::new(),
      data,
      width,
      height,
      channel_count,
      texture_wrap: TextureWrap { x: TextureWrapType::Repeat, y: TextureWrapType::Repeat },
      texture_magnification: TextureMagnification { min: TextureMagnificationType::Linear, max: TextureMagnificationType::Linear }
    }
  }

  pub fn load(path: &str) -> Result<Texture, LoadTextureError> {
    if path.ends_with(".hdr") {
      let i = hdrldr::load(File::open(path).unwrap()).unwrap();

      let mut v = vec!();
      for p in i.data {
        v.push(p.r);
        v.push(p.g);
        v.push(p.b);
      }

      let v = flip_byte_vertically(&v, i.width as u32, i.height as u32, 3);
      Ok(Texture::new(TexturePixel::RgbF32(v), i.width as u32, i.height as u32, 3))
    } else {
      let i = image::open(path)?;

      // TODO: i.raw_pixels() clones underlying bytes, find a way that doesn't
      let v = flip_byte_vertically(&i.raw_pixels(), i.width() as u32, i.height() as u32, 3);
      Ok(Texture::new(TexturePixel::RgbF8(v), i.width(), i.height(), 3))
    }
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn clone_data_flip_vertically(&self) -> TexturePixel {
    match &self.data {
      TexturePixel::RgbF8(v) => TexturePixel::RgbF8(flip_byte_vertically(v, self.width, self.height, self.channel_count)),
      TexturePixel::RgbF32(v) => TexturePixel::RgbF32(flip_byte_vertically(v, self.width, self.height, self.channel_count)),
    }
  }

  pub fn get_wrap(&self) -> &TextureWrap {
    &self.texture_wrap
  }

  pub fn get_magnification(&self) -> &TextureMagnification {
    &self.texture_magnification
  }

  pub fn get_data(&self) -> &TexturePixel {
    &self.data
  }
}

impl_id!(Texture, id);

pub struct TextureWrap {
  pub x: TextureWrapType,
  pub y: TextureWrapType,
}

pub enum TextureWrapType {
  ClampToEdge,
  MirroredRepeat,
  Repeat,
}

pub struct TextureMagnification {
  pub min: TextureMagnificationType,
  pub max: TextureMagnificationType,
}

pub enum TextureMagnificationType {
  Nearest,
  Linear,
}

#[derive(Debug)]
pub enum TexturePixel {
  RgbF8(Vec<u8>),
  //  8 Bytes per channel
  RgbF32(Vec<f32>), // 32 Bytes per channel
}

#[derive(Debug)]
pub enum LoadTextureError {
  ImageError(ImageError),
}

impl From<ImageError> for LoadTextureError {
  fn from(e: ImageError) -> Self {
    LoadTextureError::ImageError(e)
  }
}

fn flip_byte_vertically<T: Clone>(v: &Vec<T>, width: u32, height: u32, channel_count: u32) -> Vec<T> {
  let column_size = (width * channel_count) as usize;
  let row_size = height as usize;
  let mut flipped = vec!();
  for row in (0..row_size).rev() {
    let first_row_idx = row * column_size;
    flipped.extend_from_slice(&v[first_row_idx..first_row_idx + column_size]);
  }
  flipped
}

#[test]
fn test_flip_image_vertically() {
  let k =
    vec!(
      0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0,
      6.0f32, 7.0, 8.0, 9.0, 10.0, 11.0,
      12.0f32, 13.0, 14.0, 15.0, 16.0, 17.0,
    );

  let _t = Texture::new(TexturePixel::RgbF32(k), 2, 3, 3);

  let k =
    vec!(
      0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0,
      9.0f32, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
    );

  let t = Texture::new(TexturePixel::RgbF32(k), 3, 2, 3);
  println!("{:?}", t.clone_data_flip_vertically());
}