use crate::data::rect2d::Rect2d;

#[derive(Clone)]
pub struct DisplayInfo {
  pub dimension: Rect2d,
}

impl DisplayInfo {
  pub fn new(dimension: Rect2d) -> DisplayInfo {
    DisplayInfo {
      dimension,
    }
  }

  pub fn get_dimension(&self) -> &Rect2d {
    &self.dimension
  }
}