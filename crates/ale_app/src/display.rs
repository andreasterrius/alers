use ale_math::rect::Rect;
use ale_world::components::Id;

#[derive(Clone)]
pub enum TargetMonitor {
  PRIMARY,
  SECOND,
  THIRD,
  FOURTH,
  FIFTH,
}

#[derive(Clone)]
pub struct DisplaySetting {
  pub id: u32,
  // client level identifier
  pub dimension: Rect,
  pub target: TargetMonitor,
}

impl DisplaySetting {
  pub fn new(dimension: Rect) -> DisplaySetting {
    DisplaySetting { id: 0, dimension, target: TargetMonitor::PRIMARY }
  }

  pub fn get_dimension(&self) -> &Rect {
    &self.dimension
  }
}
