use ale_app::app::{App, Init};
use ale_app::display::{DisplaySetting, TargetMonitor};
use ale_math::rect::Rect;
use ale_math::{Vector2, Zero};
use ale_resources::resources::Resources;
use ale_world::world::{Entity, Input, Tick, World};

struct Editor;
impl Init for Editor {
  fn request_windows(&self) -> Vec<DisplaySetting> {
    vec!(
      DisplaySetting {
        dimension: Rect { position: Vector2::zero(), size: Vector2::new(800, 600) } ,
        target: TargetMonitor::PRIMARY
      }
    )
  }

  fn init(&self,
          resources: &mut Resources,
          world: &mut World) -> Result<(), ale_app::AppError> {

    /*
      world.register(PongPaddleTick, Position, Key<Mesh>, Key<Sound>);
      world.register(PongPaddleTick, Position, Key<Mesh>, Key<Sound>);
    */
    world.spawn(PongPaddle);

    Ok(())
  }
}

//#[Tick, Render, Input
struct PongPaddle;

impl Entity for PongPaddle {}

traitcast::traitcast!(struct PongPaddle: Entity, Tick);

#[Enable]
impl Tick for PongPaddle {
  fn tick(&mut self, delta_time : f32) {
    println!("abc");
  }
}

#[Enable]
impl Input for PongPaddle {
  fn input(&mut self, input : ale_input::Input) {

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

