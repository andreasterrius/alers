use std::time::Instant;

pub enum WorldTick {
  FixedStep(FixedStep),
}

impl WorldTick {
  pub fn should_tick(&self) -> bool {
    match self {
      WorldTick::FixedStep(x) => x.should_tick(),
    }
  }

  pub fn tick(&mut self) {
    match self {
      WorldTick::FixedStep(x) => x.tick(),
    }
  }

  pub fn prepare_tick(&mut self) {
    match self {
      WorldTick::FixedStep(x) => x.prepare_tick(),
    }
  }

  pub fn delta_time(&self) -> f32 {
    match self {
      WorldTick::FixedStep(x) => x.tick_delta_time(),
    }
  }
}

pub struct FixedStep {
  frame_step: f32,
  delta_time: f32,

  previous_time: Instant,
  accumulator: f32,
}

impl FixedStep {
  pub fn new(frame_step: f32) -> FixedStep {
    FixedStep {
      frame_step,
      delta_time: 0.0,
      previous_time: Instant::now(),
      accumulator: 0.0,
    }
  }

  pub fn should_tick(&self) -> bool {
    // Save the last tick for the next frame
    if self.accumulator >= 10e-7 {
      return true;
    }
    return false;
  }

  pub fn tick(&mut self) {
    if self.accumulator < self.frame_step {
      self.delta_time = self.accumulator;
      self.accumulator = 0.0;
    } else {
      self.delta_time = self.frame_step;
      self.accumulator -= self.frame_step;
    }
  }

  pub fn prepare_tick(&mut self) {
    self.accumulator =
      (Instant::now().duration_since(self.previous_time).subsec_nanos() as f64 / 1000000000.0f64) as f32;
    self.previous_time = Instant::now();
  }

  pub fn tick_delta_time(&self) -> f32 {
    self.delta_time
  }
}
