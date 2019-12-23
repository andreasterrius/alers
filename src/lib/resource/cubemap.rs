use crate::data::id::Id;

pub struct Cubemap {
  id : Id,
}

impl Cubemap {
  pub fn new() -> Cubemap {
    Cubemap {
      id : Id::new()
    }
  }
}

impl_id!(Cubemap, id);

