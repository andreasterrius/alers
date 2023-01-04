use ale_app::app::{App, Genesis};
use ale_app::AppError;
use ale_app::engine::Engine;
use ale_math::rect::Rect;
use ale_math::Vector2;
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_world::event::world::{SpawnCommand, WorldCommand};
use ale_world::typecast::entry::Traitcast;
use ale_world::world::World;

mod tetris;
mod template;
mod piece;

struct Tetris;

pub enum TetrisEvent{}

impl Genesis for Tetris {

  fn register_components(&self, world: &mut World) {
    tetris::Game::register_components(world);
  }

  fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), AppError> {
    engine.windows.add(DisplaySetting {
      dimension: Rect {
        position: Vector2::new(0, 0),
        size: Vector2::new(800, 600),
      },
      initial_target: TargetMonitor::PRIMARY,
      is_hidden: false,
    });

    let wc_sender = world.get_world_command_sender();

    let tetris = tetris::Game::new(wc_sender.clone());
    wc_sender.send(WorldCommand::Spawn(SpawnCommand::new(tetris)));

    Ok(())
  }
}

fn main() {
  App::new(Tetris).run();
}
