use ale_app::app::{App, Genesis};
use ale_app::engine::Engine;
use ale_app::AppError;
use ale_math::rect::Rect;
use ale_math::{Vector2, Vector3, Zero};
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_world::components::Spawnable;
use ale_world::event::world::{SpawnCommand, WorldCommand};
use ale_world::world::World;

use crate::camera_2d::Camera2D;
use crate::piece::Piece;
use crate::tetris::GameCoordinator;

mod camera_2d;
mod piece;
mod template;
mod tetris;

struct TetrisGame;

pub enum TetrisEvent {}

impl Genesis for TetrisGame {
  fn register_components(&self, world: &mut World) {
    Camera2D::register_components(world);
    GameCoordinator::register_components(world);
    Piece::register_components(world);
  }

  fn window(&self) -> DisplaySetting {
    DisplaySetting {
      dimension: Rect {
        position: Vector2::new(0, 0),
        size: Vector2::new(800, 600),
      },
      initial_target: TargetMonitor::PRIMARY,
      is_hidden: false,
    }
  }

  fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), AppError> {
    let camera_2d = Camera2D::new(
      Vector3::new(0.0, 0.0, 5.0),
      Rect {
        position: Vector2::zero(),
        size: Vector2::new(800, 600),
      },
    );

    // world command
    let wc_sender = world.get_world_command_sender();

    // create world
    let tetris = GameCoordinator::new(wc_sender.clone());

    // send spawn commands to world
    wc_sender
      .send(WorldCommand::Spawn(SpawnCommand::new(camera_2d)))
      .unwrap();
    wc_sender.send(WorldCommand::Spawn(SpawnCommand::new(tetris))).unwrap();

    Ok(())
  }
}

fn main() {
  App::new(TetrisGame).run();
}
