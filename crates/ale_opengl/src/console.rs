use crate::mesh::OpenGLMeshContext;
use crate::shader::OpenGLShaderContext;
use crate::text::OpenGLTextContext;
use ale_camera::CameraRenderInfo;
use ale_console::Console;
use ale_font::Font;
use ale_math::Vector2;

pub fn ale_opengl_console_render(
  opengl_text_context: &mut OpenGLTextContext,
  camera_render_info: &CameraRenderInfo,
  console: &Console,
  screen_size: Vector2<i32>,
  font: &mut Font,
) {
  if console.has_focus {
    let mut curr_pos = Vector2::new(0.0, screen_size.y as f32);
    let font_size = 24;

    // Render current line buffer
    let font_layout = font.layout(
      font_size,
      &format!("{}_", &console.line_buffer),
      Some(screen_size.clone()),
    );
    curr_pos.y -= font_size as f32;
    opengl_text_context.render_layout(camera_render_info, &font_layout, &font, curr_pos);

    // Render existing lines
    for line in console.lines.iter().rev() {
      let font_layout = font.layout(font_size, line, Some(screen_size.clone()));

      curr_pos.y -= font_size as f32;
      opengl_text_context.render_layout(camera_render_info, &font_layout, &font, curr_pos);

      if curr_pos.y < 0.0 {
        break;
      }
    }
  }
}
