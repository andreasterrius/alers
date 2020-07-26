use crate::data::color::Color;
use crate::math::rect::Rect;
use crate::ui::button::Button;
use crate::ui::panel::Panel;
use crate::ui::text::Text;

pub mod button;
pub mod panel;
pub mod text;

pub enum UI {
  Button(Button),
  Panel(Panel),
  Text(Text),
}

//impl UI {
//  pub fn get_ui_render_info(&self) -> Vec<UIRenderInfo> {
//    match self {
//      UI::Button(button) => button.get_ui_render_info(),
//      UI::Panel(panel) => panel.get_ui_render_info(),
//      UI::Text(text) => text.get_ui_render_info(),
//    }
//  }
//}

pub struct UIRenderInfo {
  rect: Rect,
  color: Color,
}
