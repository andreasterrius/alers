use crate::ui::{UIRenderInfo, UI};
use ale_input::Input;
use ale_math::color::Color;
use ale_math::rect::Rect;
use std::rc::Rc;

pub struct Panel {
  rect: Rect,
  color: Color,

  is_hidden: bool,
}

impl Panel {
  pub fn new(rect: Rect, background_color: Color) -> Panel {
    Panel {
      rect,
      color: background_color,
      is_hidden: false,
    }
  }

  pub fn input(&mut self, input: &Vec<Input>) {}

  pub fn get_ui_render_info(&self) -> UIRenderInfo {
    UIRenderInfo {
      color: self.color.clone(),
      rect: self.rect.clone(),
    }
  }
}
