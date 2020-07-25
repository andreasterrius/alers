use crate::data::id::Id;
use crate::math::rect::Rect;

pub struct Cubemap {
  id: Id,
  dimension: Rect,
}

impl Cubemap {
  pub fn new(dimension: Rect) -> Cubemap {
    Cubemap { id: Id::new(), dimension }
  }

  pub fn get_dimension(&self) -> &Rect {
    &self.dimension
  }
}

impl_id!(Cubemap, id);
