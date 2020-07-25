use crate::data::color::Color;
use crate::math::rect::Rect;
use crate::ui::UI;

pub struct Panel {
  rect: Rect,
  background_color: Color,

  childs: Vec<UI>,
}

impl Panel {
  pub fn new(rect: Rect, background_color: Color) -> Panel {
    Panel {
      rect,
      background_color,
      childs: vec![],
    }
  }

  pub fn add_child(&mut self, ui: UI) {
    self.childs.push(ui);
  }
}
