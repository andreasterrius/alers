use std::time::{Instant, SystemTime, UNIX_EPOCH};

pub struct FixedTick {
  dt: f64,
  current_time: u128,
}

impl FixedTick {
  pub fn new() -> FixedTick {
    FixedTick {
      current_time: intern_get_milis(),
      dt: 1.0 / 60.0,
    }
  }

  pub fn tick<F>(&mut self, f: &mut F)
  where
    F: FnMut(f64),
  {
    let new_time = intern_get_milis();
    let mut frame_time: f64 = (new_time - self.current_time) as f64 / 1_000_000.0;
    self.current_time = new_time;

    while frame_time >= 1.0e-7 {
      let delta_time = f64::min(frame_time, self.dt);
      f(delta_time);
      frame_time -= delta_time;
    }
  }
}

fn intern_get_milis() -> u128 {
  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()
}
