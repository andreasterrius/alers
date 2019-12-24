use crate::data::id::Id;
use crate::data::rect2d::Rect2d;

pub struct Cubemap {
  id : Id,
  dimension : Rect2d,
}

impl Cubemap {
  pub fn new(dimension : Rect2d) -> Cubemap {
    Cubemap {
      id : Id::new(),
      dimension
    }
  }

  pub fn get_dimension(&self) -> &Rect2d {
    &self.dimension
  }
}

impl_id!(Cubemap, id);

