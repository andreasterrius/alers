use std::time::Instant;

pub trait FisikaObject {
    fn fisika_tick(&mut self, fixed_dt : f32);

    fn on_collision(&mut self, fixed_dt : f32, other : &FisikaObject);
}

pub struct FixedFisikaTicker {
    frame_step : f32,
    delta_time : f32,

    previous_time: Instant
}

impl FixedFisikaTicker {

    pub fn new(frame_step : f32) -> FixedFisikaTicker {
        FixedFisikaTicker {
            frame_step,
            delta_time: 0.0,
            previous_time: Instant::now(),
        }
    }

    pub fn fisika_tick(&mut self,
                       on_fixed_tick : &Fn(f32)) -> f32 {

        self.delta_time = (Instant::now().duration_since(
            self.previous_time).subsec_nanos() as f64
            / 1000000000.0f64) as f32;

        self.previous_time = Instant::now();

        let mut accumulator = self.delta_time;
        while accumulator >= self.frame_step {
            on_fixed_tick(self.frame_step);
            accumulator -= self.frame_step;
        }

        accumulator
    }
}