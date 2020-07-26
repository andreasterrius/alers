use crate::data::color::Color;
use crate::math::rect::Rect;
use crate::ui::UIRenderInfo;

pub struct Button {
  rect: Rect,
  is_hidden: bool,

  background_color: Color,
  on_hover_color: Color,
  on_press_color: Color,
}

impl Button {
  pub fn get_ui_render_info(&self) -> Vec<UIRenderInfo> {
    return vec![UIRenderInfo {
      rect: self.rect.clone(),
      color: self.background_color.clone(),
    }];
  }
}
