use ale_math::rect::Rect;

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
  pub dimension: Rect,
  pub initial_target: TargetMonitor,
}

impl DisplaySetting {
  pub fn new(dimension: Rect) -> DisplaySetting {
    DisplaySetting { dimension, initial_target: TargetMonitor::PRIMARY }
  }

  pub fn get_dimension(&self) -> &Rect {
    &self.dimension
  }
}
