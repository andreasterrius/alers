use crate::ui::UI;
use ale_math::rect::Rect;
use ale_input::Input;

pub struct Text {
  rect: Rect,
  is_hidden: bool,
}

impl Text {
  pub fn input(&mut self, input: &Vec<Input>) {}
}
