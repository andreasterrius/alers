use crate::input::Input;
use crate::math::rect::Rect;
use crate::ui::UI;

pub struct Text {
  rect: Rect,
  is_hidden: bool,
}

impl Text {
  pub fn input(&mut self, input: &Vec<Input>) {}
}
