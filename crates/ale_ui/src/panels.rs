use ale_data::alevec::AleVec;
use crate::element::Panel;

pub struct Panels {
  panels: AleVec<Panel>,
}

impl Panels {
  pub fn new() -> Panels {
    Panels {
      panels: AleVec::new()
    }
  }
}