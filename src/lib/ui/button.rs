use crate::ui::UIRenderInfo;
use ale_math::rect::Rect;
use ale_math::color::Color;
use ale_input::Input;

pub struct Button {
  rect: Rect,
  is_hidden: bool,

  background_color: Color,
  on_hover_color: Color,
  on_press_color: Color,
}

impl Button {
  pub fn input(&mut self, input: &Vec<Input>) {}

  pub fn get_ui_render_info(&self) -> UIRenderInfo {
    UIRenderInfo {
      color: self.background_color.clone(),
      rect: self.rect.clone(),
    }
  }
}
