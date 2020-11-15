use crate::ui::button::Button;
use crate::ui::panel::Panel;
use crate::ui::text::Text;
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_input::Input;

pub mod button;
pub mod panel;
pub mod text;

pub enum UI {
  Button(Button),
  Panel(Panel),
}

impl UI {
  pub fn input(&mut self, inputs: &Vec<Input>) {
    match self {
      UI::Button(button) => button.input(inputs),
      UI::Panel(panel) => panel.input(inputs),
    }
  }

  pub fn get_ui_render_info(&self) -> UIRenderInfo {
    match self {
      UI::Button(button) => button.get_ui_render_info(),
      UI::Panel(panel) => panel.get_ui_render_info(),
    }
  }
}

pub struct UIRenderInfo {
  pub color: Color,
  pub rect: Rect,
}
