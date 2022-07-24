mod tetris;

use ale_app::app::{App, Genesis};
use ale_app::engine::Engine;
use ale_app::AppError;
use ale_math::rect::Rect;
use ale_math::Vector2;
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_world::world::World;

struct Tetris;

impl Genesis for Tetris {
  fn register_components(&self, world: &mut World) {
    tetris::Game::register_components(world);
  }

  fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), AppError> {
    let main_window_key = engine.windows.add(DisplaySetting {
      dimension: Rect {
        position: Vector2::new(0, 0),
        size: Vector2::new(800, 600),
      },
      initial_target: TargetMonitor::PRIMARY,
      is_hidden: false,
    });

    let tetris_game_key = world.spawn(tetris::Game::new());

    Ok(())
  }
}


fn main() {
  App::new(Tetris).run();
}
