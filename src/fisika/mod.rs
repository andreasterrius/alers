use std::time::Instant;
use cgmath::Vector2;

pub trait FisikaObject {
    fn fisika_tick(&mut self, fixed_dt : f32);

    fn on_collision(&mut self, fixed_dt : f32, other : &FisikaObject);
}

pub trait BoxCollider2D {
    fn get_world_position(&self) -> Vector2<f32>;

    fn get_size(&self) -> Vector2<f32>;
}

pub trait CircleCollider2D {
    fn get_world_position(&self) -> Vector2<f32>;

    fn get_radius(&self) -> f32;
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

    pub fn fisika_tick<F>(&mut self,
                       on_fixed_tick : &mut F) -> f32
        where F : FnMut(f32)
    {

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

fn aabb_collission_box_box(box_collider1: &T, box_collider2: &T) -> bool
    where T : BoxCollider2D
{

}

fn aabb_collission_box_circle(box_collider1: &T, circle_collider: &U) -> bool
    where T : BoxCollider2D, U : CircleCollider2D
{

}