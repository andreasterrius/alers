use ale_camera::CameraRenderInfo;
use ale_console::Console;
use ale_math::Vector2;
use ale_resources::font::Font;

use crate::renderer::text::TextRenderer;

pub fn ale_opengl_console_render(
  text_renderer: &mut TextRenderer,
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
    text_renderer.render_layout(camera_render_info, &font_layout, &font, curr_pos);

    // Render existing lines
    for line in console.lines.iter().rev() {
      let font_layout = font.layout(font_size, line, Some(screen_size.clone()));

      curr_pos.y -= font_size as f32;
      text_renderer.render_layout(camera_render_info, &font_layout, &font, curr_pos);

      if curr_pos.y < 0.0 {
        break;
      }
    }
  }
}
