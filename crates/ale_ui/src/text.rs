use ale_data::alevec::Key;
use ale_math::Vector2;
use ale_resources::font::Font;
use crate::element::RenderResources;

pub struct Text {
  pos: Vector2<f32>,
  font: Key<Font>,
  font_size: u32,
  text: String,
}

impl Text {
  pub fn new(pos: Vector2<f32>, text: String, font: Key<Font>, font_size: u32) -> Text {
    Text {
      pos,
      text,
      font,
      font_size,
    }
  }

  pub fn render_with(&mut self, rr : &mut RenderResources) {
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
      self.pos,
      &self.text,
      None,
    )
  }
}
