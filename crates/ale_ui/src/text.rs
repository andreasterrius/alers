use ale_data::alevec::Key;
use ale_math::Vector2;
use ale_resources::font::Font;

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

  pub fn set_text(&mut self, text: String) {
    self.text = text;
  }

  pub fn set_pos(&mut self, pos: Vector2<f32>) {
    self.pos = pos;
  }

  pub fn get_text(&self) -> &str {
    &self.text
  }

  pub fn get_pos(&self) -> Vector2<f32> {
    return self.pos;
  }

  pub fn get_font(&self) -> Key<Font> {
    self.font
  }

  pub fn get_font_size(&self) -> u32 {
    self.font_size
  }
}
