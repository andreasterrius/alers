use std::time::Instant;

pub struct FixedStepTick {
    frame_step : f32,
    delta_time : f32,

    previous_time: Instant
}

impl FixedStepTick {

    pub fn new(frame_step : f32) -> FixedStepTick {
        FixedStepTick {
            frame_step,
            delta_time: 0.0,
            previous_time: Instant::now(),
        }
    }

    pub fn tick<F>(&mut self,
                   on_fixed_tick : &mut F) -> f32
        where F : FnMut(f32, bool)
    {

        self.delta_time = (Instant::now().duration_since(
            self.previous_time).subsec_nanos() as f64
            / 1000000000.0f64) as f32;

        self.previous_time = Instant::now();

        let mut is_last_tick = false;
        let mut accumulator = self.delta_time;
        while accumulator >= self.frame_step {

            if accumulator >= self.frame_step &&
                accumulator <= self.frame_step * 2.0
            {
                is_last_tick = true;
            }

            on_fixed_tick(self.frame_step, is_last_tick);
            accumulator -= self.frame_step;

        }

        accumulator
    }
}