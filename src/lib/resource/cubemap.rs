use crate::math::rect::Rect;

pub struct Cubemap {
  id: CubemapId,
  dimension: Rect,
}

impl Cubemap {
  pub fn new(dimension: Rect) -> Cubemap {
    Cubemap {
      id: CubemapId::new(),
      dimension,
    }
  }

  pub fn get_dimension(&self) -> &Rect {
    &self.dimension
  }
}

struct_id!(CubemapId);
struct_id_impl!(CubemapId, Cubemap, id);
