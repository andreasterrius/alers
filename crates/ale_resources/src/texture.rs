use crate::stash::Load;
use crate::{struct_id, struct_id_impl};
use image::imageops::{flip_vertical, flip_vertical_in_place};
use image::ImageError;
use std::fs::File;

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

impl Texture {
  pub fn new(data: TexturePixel, width: u32, height: u32, channel_count: u32) -> Texture {
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

  pub fn load(path: &str) -> Result<Texture, LoadError> {
    if path.ends_with(".hdr") {
      let i = hdrldr::load(File::open(path)?)?;

      let mut v = vec![];
      for p in i.data {
        v.push(p.r);
        v.push(p.g);
        v.push(p.b);
      }

      let v = intern_flip_byte_vertically(&v, i.width as u32, i.height as u32, 3);
      Ok(Texture::new(
        TexturePixel::RgbF32(v),
        i.width as u32,
        i.height as u32,
        3,
      ))
    } else {
      let i = image::open(path)?;
      let width = i.width();
      let height = i.height();
      let bytes = i.into_bytes();

      let v = intern_flip_byte_vertically(&bytes, width, height, 3);
      Ok(Texture::new(TexturePixel::RgbU8(v), width, height, 3))
    }
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
pub enum LoadError {
  ImageError(ImageError),
  FileNotFound,
}

pub struct Loader;
impl Load<Texture, LoadError> for Loader {
  fn load(&self, path: &str) -> Result<Vec<Texture>, LoadError> {
    return match Texture::load(path) {
      Ok(texture) => {
        Ok(vec![texture])
      }
      Err(err) => {
        return Err(err);
      }
    };
  }
}

impl Default for Loader {
  fn default() -> Self {
    Loader
  }
}

impl From<ImageError> for LoadError {
  fn from(e: ImageError) -> Self {
    LoadError::ImageError(e)
  }
}

impl From<std::io::Error> for LoadError {
  fn from(_e: std::io::Error) -> Self {
    LoadError::FileNotFound
  }
}

impl From<hdrldr::LoadError> for LoadError {
  fn from(_: hdrldr::LoadError) -> Self {
    LoadError::FileNotFound
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

  let _t = Texture::new(TexturePixel::RgbF32(k), 2, 3, 3);

  let k = vec![
    0.0f32, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0f32, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0,
  ];

  let t = Texture::new(TexturePixel::RgbF32(k), 3, 2, 3);
  //println!("{:?}", intern_flip_byte_vertically());
}
