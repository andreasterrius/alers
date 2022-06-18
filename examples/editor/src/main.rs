use ale_app::app::{App, Init};
use ale_app::display::{DisplaySetting, TargetMonitor};
use ale_math::{Vector2, Zero};
use ale_math::rect::Rect;
use ale_resources::resources::Resources;
use ale_world::world::World;
use crate::ui::main_frame::MainFrame;

mod ui;
mod scene;

struct Editor;

enum Windows {
  Main = 0,
  Test = 1
}

impl Init for Editor {
  fn request_initial_windows(&self) -> Vec<DisplaySetting> {
    vec![
      DisplaySetting {
        id: Windows::Main as u32,
        dimension: Rect {
          position: Vector2::zero(),
          size: Vector2::new(800, 600),
        },
        target: TargetMonitor::PRIMARY,
      },
      DisplaySetting {
        id: Windows::Test as u32,
        dimension: Rect {
          position: Vector2::zero(),
          size: Vector2::new(400, 300),
        },
        target: TargetMonitor::PRIMARY,
      }
    ]
  }

  fn register_components(&self, world: &mut World) {
    MainFrame::register_components(world);
  }

  fn init(&self, resources: &mut Resources, world: &mut World) -> Result<(), ale_app::AppError> {
    world.spawn(MainFrame::new());

    Ok(())
  }
}

fn main() {
  App::run(&mut Editor);
}
