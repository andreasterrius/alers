use rusttype::{point, Scale};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct Font {
  pub(crate) intern_font: rusttype::Font<'static>,

  pub(crate) font_rasters: HashMap<FontRasterKey, FontRaster>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct FontRasterKey {
  pub(crate) a: char,
  pub(crate) font_size: i32,
}

#[derive(Debug)]
pub struct FontRaster {}

pub fn ale_font_load(path: &str) -> Font {
  let font = {
    let mut font_file = File::open(&path).unwrap();
    let mut font_data = vec![];
    font_file.read_to_end(&mut font_data);
    rusttype::Font::try_from_vec(font_data).expect("Error constructing a Font from bytes")
  };

  return Font {
    intern_font: font,
    font_rasters: HashMap::new(),
  };
}

pub fn ale_font_raster(font: &mut Font, font_size: i32, text: &str) {
  // 2x scale in x direction to counter the aspect ratio of monospace characters.
  let scale = Scale {
    x: font_size as f32 * 2.0,
    y: font_size as f32,
  };

  let v_metrics = font.intern_font.v_metrics(scale);
  let offset = point(0.0, v_metrics.ascent);

  let glyphs: Vec<_> = font.intern_font.layout(text, scale, offset).collect();

  for g in glyphs {
    if let Some(bb) = g.pixel_bounding_box() {
      let width = g.unpositioned().h_metrics().advance_width.ceil() as usize;
      let height = v_metrics.ascent.ceil() as usize;

      let mut bytes: Vec<f32> = vec![0.0; width * height];
      g.draw(|x, y, v| {
        bytes[x as usize + y as usize * width] = v;
      });
    }
  }
}
