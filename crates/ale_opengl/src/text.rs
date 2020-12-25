use crate::mesh::{OpenGLMesh, OpenGLMeshContext};
use crate::shader::{OpenGLShader, OpenGLShaderContext};
use crate::texture::OpenGLTexture;
use ale_camera::CameraRenderInfo;
use ale_font::{Font, FontLayout, FontTextureKey};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::Vector2;
use ale_mesh::Mesh;
use ale_shader::Shader;
use std::collections::HashMap;

pub struct OpenGLTextContext {
  pub(crate) glyph_texture: HashMap<FontTextureKey, OpenGLTexture>,

  pub(crate) text_2d_shader: OpenGLShader,
  pub(crate) plane_mesh: OpenGLMesh,
}

impl OpenGLTextContext {
  pub fn new() -> OpenGLTextContext {
    OpenGLTextContext {
      glyph_texture: Default::default(),
      text_2d_shader: OpenGLShader::new(&Shader::new(
        include_str!("../resources/text_2d.vert").to_owned(),
        include_str!("../resources/text_2d.frag").to_owned(),
      ))
      .unwrap(),
      plane_mesh: OpenGLMesh::new(&Mesh::new_plane()).unwrap(),
    }
  }

  pub fn render(
    &mut self,
    camera_render_info: &CameraRenderInfo,
    font: &mut Font,
    font_size: i32,
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
      self.glyph_texture.entry(l.font_texture_key.clone()).or_insert_with(|| {
        let font_texture = match font.textures.get(&l.font_texture_key) {
          None => panic!("Unable to render, font glyph wasn't rasterized"),
          Some(ft) => ft,
        };

        unsafe { OpenGLTexture::new(&font_texture.texture).unwrap() }
      });

      let opengl_texture = self.glyph_texture.get(&l.font_texture_key).unwrap();

      opengl_texture.render(
        &self.plane_mesh,
        &self.text_2d_shader,
        origin + Vector2::new(l.offset_min.x as f32, l.offset_min.y as f32),
        &Color::from_rgb(1.0, 1.0, 1.0),
        &camera_render_info,
      )
    }
  }
}
