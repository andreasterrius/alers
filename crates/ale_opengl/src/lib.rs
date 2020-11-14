use crate::raw::{create_texture, OpenGLTextureId};
use ale_font::{ale_font_layout, Font, FontTexture, FontTextureKey};
use ale_texture::{ale_texture_new, TexturePixel};
use std::collections::HashMap;

pub mod raw;

pub struct OpenGLTextureContext {
  glyph_texture: HashMap<FontTextureKey, OpenGLTextureId>,
}

pub fn ale_opengl_texture_context_new() -> OpenGLTextureContext {
  OpenGLTextureContext {
    glyph_texture: HashMap::new(),
  }
}

pub fn ale_opengl_text_render(
  opengl_texture_context: &mut OpenGLTextureContext,
  font: &mut Font,
  font_size: i32,
  text: &str,
) {
  let layouts = ale_font_layout(font, font_size, text);

  for l in &layouts {
    opengl_texture_context
      .glyph_texture
      .entry(l.font_texture_key.clone())
      .or_insert_with(|| {
        let font_texture = match font.textures.get(&l.font_texture_key) {
          None => panic!("Unable to render, font glyph wasn't rasterized"),
          Some(ft) => ft,
        };

        unsafe { create_texture(&font_texture.texture).unwrap() }
      });
  }
}
