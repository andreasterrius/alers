use crate::mesh::{ale_opengl_mesh_new, OpenGLMesh, OpenGLMeshContext};
use crate::shader::{ale_opengl_shader_new, OpenGLShader, OpenGLShaderContext};
use crate::texture::{ale_opengl_texture_new, ale_opengl_texture_render, OpenGLTexture, OpenGLTextureContext};
use ale_camera::CameraRenderInfo;
use ale_font::{ale_font_layout, Font, FontLayout, FontTextureKey};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::Vector2;
use ale_mesh::ale_mesh_plane_new;
use ale_shader::ale_shader_new;
use std::collections::HashMap;

pub struct OpenGLTextFontContext {
  text_2d_shader: OpenGLShader,
  plane_opengl_mesh: OpenGLMesh,
  glyph_texture: HashMap<FontTextureKey, OpenGLTexture>,
}

pub fn ale_opengl_text_font_context_new() -> OpenGLTextFontContext {
  let text_2d_shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../../../resources/shaders/text_2d.vert").to_owned(),
    include_str!("../../../resources/shaders/text_2d.frag").to_owned(),
  ))
  .unwrap();

  let plane_opengl_mesh = ale_opengl_mesh_new(&ale_mesh_plane_new()).unwrap();

  OpenGLTextFontContext {
    text_2d_shader,
    plane_opengl_mesh,
    glyph_texture: Default::default(),
  }
}

pub fn ale_opengl_text_render(
  opengl_text_font_context: &mut OpenGLTextFontContext,
  camera_render_info: &CameraRenderInfo,
  font: &mut Font,
  font_size: i32,
  origin: Vector2<f32>,
  text: &str,
  bounds: Option<Vector2<i32>>,
) {
  let layout = ale_font_layout(font, font_size, text, bounds);
  ale_opengl_text_render_layout(opengl_text_font_context, camera_render_info, &layout, font, origin);
}

pub fn ale_opengl_text_render_layout(
  opengl_text_font_context: &mut OpenGLTextFontContext,
  camera_render_info: &CameraRenderInfo,
  font_layout: &FontLayout,
  font: &Font,
  origin: Vector2<f32>,
) {
  for l in &font_layout.glyphs {
    opengl_text_font_context
      .glyph_texture
      .entry(l.font_texture_key.clone())
      .or_insert_with(|| {
        let font_texture = match font.textures.get(&l.font_texture_key) {
          None => panic!("Unable to render, font glyph wasn't rasterized"),
          Some(ft) => ft,
        };

        unsafe { ale_opengl_texture_new(&font_texture.texture).unwrap() }
      });

    let opengl_texture = opengl_text_font_context.glyph_texture.get(&l.font_texture_key).unwrap();

    ale_opengl_texture_render(
      &opengl_text_font_context.plane_opengl_mesh,
      &opengl_text_font_context.text_2d_shader,
      &opengl_texture,
      origin + Vector2::new(l.offset_min.x as f32, l.offset_min.y as f32),
      &Color::from_rgb(1.0, 1.0, 1.0),
      &camera_render_info,
    )
  }
}
