pub struct Timer {
  elapsed_time: f32,
  recurred_time: i32, //how many times this timer has been triggered

  is_paused: bool,

  target_time: f32,
  target_recurrence: Recurrence,
}

pub enum Recurrence {
  Once,
  Recurring { target_recurrence: i32 },
  Forever,
}

impl Timer {
  pub fn new(target_time: f32, recurrence: Recurrence) -> Timer {
    Timer {
      elapsed_time: 0.0,
      recurred_time: 0,
      is_paused: false,
      target_time,
      target_recurrence: recurrence,
    }
  }

  pub fn new_paused(target_time: f32, recurrence: Recurrence) -> Timer {
    Timer {
      elapsed_time: 0.0,
      recurred_time: 0,
      is_paused: true,
      target_time,
      target_recurrence: recurrence,
    }
  }

  pub fn tick(&mut self, delta_time: f32) {
    let _ = self.tick_and_check(delta_time);
  }

  pub fn tick_and_check(&mut self, delta_time: f32) -> bool {
    if self.is_paused {
      return false;
    }

    match self.target_recurrence {
      Recurrence::Once => {
        if self.recurred_time >= 1 {
          return false;
        }
      }
      Recurrence::Recurring { target_recurrence } => {
        if self.recurred_time >= target_recurrence {
          return false;
        }
      }
      Recurrence::Forever => {}
    }

    self.elapsed_time += delta_time;
    if self.elapsed_time > self.target_time {
      self.elapsed_time = self.target_time - self.elapsed_time;
      self.recurred_time += 1;
      return true;
    }
    return false;
  }

  pub fn set_target_recurrence(&mut self, recurrence: Recurrence) {
    self.target_recurrence = recurrence;
  }

  pub fn reset_current_time(&mut self) {
    self.elapsed_time = 0.0;
  }

  pub fn reset_current_recurrence(&mut self) {
    self.recurred_time = 0;
  }

  pub fn set_paused(&mut self, is_paused: bool) {
    self.is_paused = is_paused;
  }

  pub fn is_paused(&self) -> bool {
    return self.is_paused;
  }

  pub fn is_elapsed_time_zero(&self) -> bool {
    return self.elapsed_time == 0.0;
  }

  pub fn set_target_time(&mut self, target_time: f32) {
    self.target_time = target_time
  }

  pub fn force_one_tick(&mut self) {
    self.elapsed_time = self.target_time + 0.01; //force 1 tick
  }

  pub fn reset_all(&mut self) {
    self.reset_current_time();
    self.reset_current_recurrence();
  }
}
