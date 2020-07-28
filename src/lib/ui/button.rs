use crate::data::color::Color;
use crate::input::Input;
use crate::math::rect::Rect;
use crate::resource::shader::ShaderFileId;
use crate::ui::UIRenderInfo;

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
