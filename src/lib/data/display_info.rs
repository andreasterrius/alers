use crate::math::rect::Rect;

#[derive(Clone)]
pub struct DisplayInfo {
  pub dimension: Rect,
}

impl DisplayInfo {
  pub fn new(dimension: Rect) -> DisplayInfo {
    DisplayInfo { dimension }
  }

  pub fn get_dimension(&self) -> &Rect {
    &self.dimension
  }
}
