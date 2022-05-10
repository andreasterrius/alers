use std::collections::HashMap;
use ale_camera::CameraRenderInfo;

use ale_math::color::Color;
use ale_math::Vector2;
use ale_resources::font::{Font, FontLayout, FontTextureKey};
use ale_resources::mesh::Mesh;
use ale_resources::shader::Shader;

use crate::mesh::OpenGLMesh;
use crate::shader::OpenGLShader;
use crate::texture::OpenGLTexture;

pub struct TextRenderer {
  text_2d_shader: OpenGLShader,
  plane_opengl_mesh: OpenGLMesh,
  glyph_texture: HashMap<FontTextureKey, OpenGLTexture>,
}

impl TextRenderer {
  pub fn new() -> TextRenderer {
    let text_2d_shader = OpenGLShader::new(&Shader::new(
      include_str!("../../../resources/shaders/text_2d.vert").to_owned(),
      include_str!("../../../resources/shaders/text_2d.frag").to_owned(),
    )).unwrap();

    let plane_opengl_mesh = OpenGLMesh::new(&Mesh::new_plane()).unwrap();

    TextRenderer {
      text_2d_shader,
      plane_opengl_mesh,
      glyph_texture: Default::default(),
    }
  }

  pub fn render(
    &mut self,
    camera_render_info: &CameraRenderInfo,
    font: &mut Font,
    font_size: u32,
    origin: Vector2<f32>,
    text: &str,
    bounds: Option<Vector2<i32>>,
  ) {
    let layout = font.layout(font_size, text, bounds);
    self.render_layout(camera_render_info, &layout, font, origin);
  }

  pub fn render_layout(
    &mut self,
    camera_render_info: &CameraRenderInfo,
    font_layout: &FontLayout,
    font: &Font,
    origin: Vector2<f32>,
  ) {
    for l in &font_layout.glyphs {
      self
        .glyph_texture
        .entry(l.font_texture_key.clone())
        .or_insert_with(|| {
          let font_texture = match font.textures.get(&l.font_texture_key) {
            None => panic!("Unable to render, font glyph wasn't rasterized"),
            Some(ft) => ft,
          };

          unsafe { OpenGLTexture::new(&font_texture.texture).unwrap() }
        });

      let opengl_texture = self.glyph_texture.get(&l.font_texture_key).unwrap();

      opengl_texture.render(
        &self.plane_opengl_mesh,
        &self.text_2d_shader,
        origin + Vector2::new(l.offset_min.x as f32, l.offset_min.y as f32),
        &Color::from_rgb(1.0, 1.0, 1.0),
        &camera_render_info,
      )
    }
  }
}
