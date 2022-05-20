use ale_data::alevec::Key;
use ale_math::{Vector2, Zero};
use ale_resources::font::Font;

use crate::element::RenderResources;
use crate::layout::Layout;

pub struct Text {
  pub(crate) layout: Layout,
  font: Key<Font>,
  font_size: u32,
  text: String,
}

impl Text {
  pub fn new(pos: Vector2<i32>, text: String, font: Key<Font>, font_size: u32) -> Text {
    Text {
      layout: Layout::new_local(pos, Vector2::zero()),
      text,
      font,
      font_size,
    }
  }

  pub fn render_with(&mut self, rr: &mut RenderResources) {
    let font = match rr.resources.fonts.get_mut(self.font) {
      None => {
        return;
      }
      Some(font) => font,
    };

    rr.text_renderer.render(
      &rr.camera_render_info,
      font,
      self.font_size,
      Vector2::new(
        self.layout.global_position.x as f32,
        self.layout.global_position.y as f32,
      ),
      &self.text,
      None,
    )
  }
}
