use std::fs::File;

use ale_autoid::*;
use hdrldr::LoadError;
use image::{GenericImageView, ImageError};

#[derive(Debug)]
pub struct Texture {
  pub id: TextureId,
  pub data: TexturePixel,
  pub width: u32,
  pub height: u32,
  pub channel_count: u32,

  pub wrap: TextureWrap,
  pub magnification: TextureMagnification,
}

struct_id!(TextureId);
struct_id_impl!(TextureId, Texture, id);

pub fn ale_texture_new(data: TexturePixel, width: u32, height: u32, channel_count: u32) -> Texture {
  Texture {
    id: TextureId::new(),
    data,
    width,
    height,
    channel_count,
    wrap: TextureWrap {
      x: TextureWrapType::ClampToEdge,
      y: TextureWrapType::ClampToEdge,
    },
    magnification: TextureMagnification {
      min: TextureMagnificationType::Linear,
      max: TextureMagnificationType::Linear,
    },
  }
}

#[derive(Debug)]
pub struct TextureWrap {
  pub x: TextureWrapType,
  pub y: TextureWrapType,
}

#[derive(Debug)]
pub enum TextureWrapType {
  ClampToEdge,
  MirroredRepeat,
  Repeat,
}

#[derive(Debug)]
pub struct TextureMagnification {
  pub min: TextureMagnificationType,
  pub max: TextureMagnificationType,
}

#[derive(Debug)]
pub enum TextureMagnificationType {
  Nearest,
  Linear,
}

#[derive(Debug)]
pub enum TexturePixel {
  // 8 Bytes per channel, but null data
  RgbU8Null,

  // 8 Bytes per channel
  RgbU8(Vec<u8>),

  // 32 Bytes per channel
  RgbF32(Vec<f32>),
}

#[derive(Debug)]
pub enum LoadTextureError {
  ImageError(ImageError),
  FileNotFound,
}

impl From<ImageError> for LoadTextureError {
  fn from(e: ImageError) -> Self {
    LoadTextureError::ImageError(e)
  }
}

impl From<std::io::Error> for LoadTextureError {
  fn from(_e: std::io::Error) -> Self {
    LoadTextureError::FileNotFound
  }
}

impl From<hdrldr::LoadError> for LoadTextureError {
  fn from(_: LoadError) -> Self {
    LoadTextureError::FileNotFound
  }
}

pub fn ale_texture_load(path: &str) -> Result<Texture, LoadTextureError> {
  if path.ends_with(".hdr") {
    let i = hdrldr::load(File::open(path)?)?;

    let mut v = vec![];
    for p in i.data {
      v.push(p.r);
      v.push(p.g);
      v.push(p.b);
    }

    let v = intern_flip_byte_vertically(&v, i.width as u32, i.height as u32, 3);
    Ok(ale_texture_new(
      TexturePixel::RgbF32(v),
      i.width as u32,
      i.height as u32,
      3,
    ))
  } else {
    let i = image::open(path)?;

    // TODO: i.raw_pixels() clones underlying bytes, find a way that doesn't
    let v = intern_flip_byte_vertically(&i.to_bytes(), i.width() as u32, i.height() as u32, 3);
    Ok(ale_texture_new(TexturePixel::RgbU8(v), i.width(), i.height(), 3))
  }
}

fn intern_flip_byte_vertically<T: Clone>(v: &Vec<T>, width: u32, height: u32, channel_count: u32) -> Vec<T> {
  let column_size = (width * channel_count) as usize;
  let row_size = height as usize;
  let mut flipped = vec![];
  for row in (0..row_size).rev() {
    let first_row_idx = row * column_size;
    flipped.extend_from_slice(&v[first_row_idx..first_row_idx + column_size]);
  }
  flipped
}

#[test]
fn test_flip_image_vertically() {
  let k = vec![
    0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0f32, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0f32, 13.0, 14.0, 15.0, 16.0, 17.0,
  ];

  let _t = Texture::from_data(TexturePixel::RgbF32(k), 2, 3, 3);

  let k = vec![
    0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0f32, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
  ];

  let t = Texture::from_data(TexturePixel::RgbF32(k), 3, 2, 3);
  println!("{:?}", t.clone_data_flip_vertically());
}
