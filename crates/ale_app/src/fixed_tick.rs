use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct FixedTick {
  dt: f64,
  current_time: u128,

  frame_time: f64,
  delta_time: f64,
}

impl FixedTick {
  pub fn new() -> FixedTick {
    FixedTick {
      current_time: intern_get_milis(),
      dt: 1.0 / 60.0,
      delta_time: 0.0,
      frame_time: 0.0,
    }
  }

  pub fn prepare_tick(&mut self) {
    let new_time = intern_get_milis();
    self.frame_time = (new_time - self.current_time) as f64 / 1_000_000.0;
    self.current_time = new_time;
  }

  pub fn frame_time_exists(&self) -> bool {
    self.frame_time >= 1.0e-7
  }

  pub fn consume_frame_time(&mut self) {
    let delta_time = f64::min(self.frame_time, self.dt);
    self.frame_time -= delta_time;
  }

  pub fn get_delta_time(&self) -> f64 {
    self.delta_time
  }
}

fn intern_get_milis() -> u128 {
  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()
}
