use ale_math::Vector2;
use ale_texture::{ale_texture_new, Texture, TexturePixel};
use rusttype::{point, vector, Scale};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct Font {
  pub(crate) intern_font: rusttype::Font<'static>,

  pub textures: HashMap<FontTextureKey, FontTexture>,
}

#[derive(Debug)]
pub struct FontTexture {
  pub glyph_id: u16,
  pub font_size: i32,
  pub texture: Texture,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FontTextureKey {
  pub glyph_id: u16,
  pub font_size: i32,
}

pub struct FontLayout {
  pub font_texture_key: FontTextureKey,
  pub offset: Vector2<f32>,
}

pub fn ale_font_load(path: &str) -> Font {
  let font = {
    let mut font_file = File::open(&path).unwrap();
    let mut font_data = vec![];
    font_file.read_to_end(&mut font_data);
    rusttype::Font::try_from_vec(font_data).expect("Error constructing a Font from bytes")
  };

  return Font {
    intern_font: font,
    textures: HashMap::new(),
  };
}

pub fn ale_font_layout(font: &mut Font, font_size: i32, text: &str) -> Vec<FontLayout> {
  let scale = Scale {
    x: font_size as f32,
    y: font_size as f32,
  };

  let v_metrics = font.intern_font.v_metrics(scale);
  let offset = point(0.0, v_metrics.ascent);

  let glyphs: Vec<_> = font.intern_font.layout(text, scale, offset).collect();
  let mut layouts = vec![];

  for g in glyphs {
    if let Some(bb) = g.pixel_bounding_box() {

      //let width = g.unpositioned().h_metrics().advance_width.ceil() as usize;
      //let height = v_metrics.ascent.ceil() as usize;
      let width = (bb.max.x - bb.min.x) as usize;
      let height = (bb.max.y - bb.min.y) as usize;

      let font_raster_key = FontTextureKey {
        glyph_id: g.id().0,
        font_size,
      };

      font.textures.entry(font_raster_key.clone()).or_insert_with(|| {
        let mut raster: Vec<f32> = vec![0.0; width * height];
        g.draw(|x, y, v| {
          raster[x as usize + y as usize * width] = v;
        });

        let texture = ale_texture_new(TexturePixel::RgbF32(raster), width as u32, height as u32, 1);

        FontTexture {
          glyph_id: g.id().0,
          font_size,
          texture,
        }
      });

      layouts.push(FontLayout {
        font_texture_key: font_raster_key,
        offset: Vector2::new(bb.min.x as f32, bb.min.y as f32),
      });
    }
  }

  layouts
}
