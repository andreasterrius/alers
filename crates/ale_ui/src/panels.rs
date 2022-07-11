use crate::element::Panel;
use ale_data::alevec::{AleVec, Key};

pub struct Panels {
  panels: AleVec<Panel>,
}

impl Panels {
  pub fn new() -> Panels {
    Panels { panels: AleVec::new() }
  }

  pub fn get_mut(&mut self, key: Key<Panel>) -> Option<&mut Panel> {
    self.panels.get_mut(key)
  }

  pub fn push(&mut self, panel: Panel) -> Key<Panel> {
    self.panels.push(panel)
  }
}
