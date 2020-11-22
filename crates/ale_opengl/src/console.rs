use crate::mesh::OpenGLMeshContext;
use crate::shader::OpenGLShaderContext;
use crate::text::{ale_opengl_text_render, ale_opengl_text_render_layout};
use crate::texture::OpenGLTextureContext;
use ale_camera::CameraRenderInfo;
use ale_console::Console;
use ale_font::{ale_font_layout, Font};
use ale_math::Vector2;

pub fn ale_opengl_console_render(
  opengl_texture_context: &mut OpenGLTextureContext,
  opengl_mesh_context: &OpenGLMeshContext,
  opengl_shader_context: &OpenGLShaderContext,
  camera_render_info: &CameraRenderInfo,
  console: &mut Console,
  screen_size: &Vector2<i32>,
  font: &mut Font,
) {
  if console.has_focus {
    let mut curr_pos = Vector2::new(0.0, screen_size.y as f32);
    let font_size = 24;

    // Render current line buffer
    let font_layout = ale_font_layout(
      font,
      font_size,
      &format!("{}_", &console.line_buffer),
      Some(screen_size.clone()),
    );
    curr_pos.y -= font_size as f32;
    ale_opengl_text_render_layout(
      opengl_texture_context,
      opengl_mesh_context,
      opengl_shader_context,
      camera_render_info,
      &font_layout,
      &font,
      curr_pos,
    );

    // Render existing lines
    for line in console.lines.iter().rev() {
      let font_layout = ale_font_layout(font, font_size, line, Some(screen_size.clone()));

      curr_pos.y -= font_size as f32;
      ale_opengl_text_render_layout(
        opengl_texture_context,
        opengl_mesh_context,
        opengl_shader_context,
        camera_render_info,
        &font_layout,
        &font,
        curr_pos,
      );

      if curr_pos.y < 0.0 {
        break;
      }
    }
  }
}
