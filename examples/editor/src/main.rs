use ale_app::app::{App, Init};
use ale_app::display::{DisplaySetting, TargetMonitor};
use ale_math::rect::Rect;
use ale_math::{Vector2, Zero};
use ale_resources::resources::Resources;
use ale_world::registry::{EntryBuilder};
use std::rc::Rc;
use ale_world::wire_component;
use ale_world::world::{Input, Tick, World};

struct Editor;

impl Init for Editor {
  fn request_windows(&self) -> Vec<DisplaySetting> {
    vec!(
      DisplaySetting {
        dimension: Rect { position: Vector2::zero(), size: Vector2::new(800, 600) },
        target: TargetMonitor::PRIMARY,
      }
    )
  }

  fn register(&self, world: &mut World) {
    PongPaddle::register(world);
  }

  fn init(&self,
          resources: &mut Resources,
          world: &mut World) -> Result<(), ale_app::AppError> {

    world.spawn(PongPaddle);

    Ok(())
  }
}

struct PongPaddle;

impl PongPaddle {
  pub fn register(world : &mut World) {
    world.enable_many(&[
      wire_component!(dyn Tick, PongPaddle),
      wire_component!(dyn Input, PongPaddle),
    ])
  }
}

impl Tick for PongPaddle {
  fn tick(&mut self, delta_time: f32) {
    println!("some tick");
  }
}

impl Input for PongPaddle {
  fn input(&mut self, input: ale_input::Input) {

  }
}

// impl Render for PongPaddle {
//   fn render(&mut self) -> RenderComponent {
//     // RenderComponent should be an enum of ways to render
//   }
// }
//

fn main() {
  App::run(&mut Editor);
}

