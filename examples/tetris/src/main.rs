use ale_app::app::{App, Genesis};
use ale_app::event::EngineCommand::CreateWindow;
use ale_app::event::{CreateWindowCommand, EngineCommand};
use ale_app::AppError;
use ale_data::channel::Sender;
use ale_math::rect::Rect;
use ale_math::{Vector2, Vector3, Zero};
use ale_render::target::{RenderTarget, RenderTargetType};
use ale_window_winit::display::{DisplaySetting, TargetMonitor};
use ale_world::components::Spawnable;
use ale_world::event::world::{RegisterComponentCommand, SpawnCommand, WorldCommand};
use WorldCommand::RegisterComponent;

use crate::camera_2d::Camera2D;
use crate::piece::Piece;
use crate::tetris::Game;

mod camera_2d;
mod piece;
mod template;
mod tetris;

struct Tetris;

pub enum TetrisEvent {}

impl Genesis for Tetris {
  fn init(&self, engine_commands: Sender<EngineCommand>, world_commands: Sender<WorldCommand>) -> Result<(), AppError> {
    // register components
    world_commands.send(RegisterComponent(RegisterComponentCommand::new(Camera2D::components())));
    world_commands.send(RegisterComponent(RegisterComponentCommand::new(Game::components())));
    world_commands.send(RegisterComponent(RegisterComponentCommand::new(Piece::components())));

    let (window_id, create_window) = CreateWindowCommand::new(DisplaySetting {
      dimension: Rect {
        position: Vector2::new(0, 0),
        size: Vector2::new(800, 600),
      },
      initial_target: TargetMonitor::PRIMARY,
      is_hidden: false,
    });
    engine_commands.send(CreateWindow(create_window));

    let camera_2d = Camera2D::new(
      Vector3::new(0.0, 0.0, 5.0),
      Rect {
        position: Vector2::zero(),
        size: Vector2::new(800, 600),
      },
    );
    // let main_render_target = engine.render_targets.push(RenderTarget {
    //   camera: camera_2d.id(),
    //   render_target_type: RenderTargetType::Window(window),
    // });

    // create world
    //let tetris = Game::new(world_commands.clone(), main_render_target);

    // send spawn commands to world
    //world_commands.send(WorldCommand::Spawn(SpawnCommand::new(camera_2d)));
    //world_commands.send(WorldCommand::Spawn(SpawnCommand::new(tetris)));

    Ok(())
  }
}

fn main() {
  App::new(Tetris).run();
}
