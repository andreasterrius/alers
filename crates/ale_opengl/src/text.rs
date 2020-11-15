use crate::mesh::OpenGLMeshContext;
use crate::shader::OpenGLShaderContext;
use crate::texture::{ale_opengl_texture_new, ale_opengl_texture_render, OpenGLTextureContext};
use ale_camera::CameraRenderInfo;
use ale_font::{ale_font_layout, Font};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::Vector2;

pub fn ale_opengl_text_render(
  opengl_texture_context: &mut OpenGLTextureContext,
  opengl_mesh_context: &OpenGLMeshContext,
  opengl_shader_context: &OpenGLShaderContext,
  camera_render_info: &CameraRenderInfo,
  font: &mut Font,
  font_size: i32,
  position: Vector2<f32>,
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

        unsafe { ale_opengl_texture_new(&font_texture.texture).unwrap() }
      });

    let opengl_texture = opengl_texture_context.glyph_texture.get(&l.font_texture_key).unwrap();

    ale_opengl_texture_render(
      &opengl_mesh_context.plane_opengl_mesh,
      &opengl_shader_context.text_2d_shader,
      &opengl_texture,
      position + l.offset,
      &Color::from_rgb(1.0, 1.0, 1.0),
      &camera_render_info,
    )
  }
}
