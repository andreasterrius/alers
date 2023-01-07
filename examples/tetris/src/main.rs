use ale_app::app::{App, Genesis};
use ale_app::AppError;
use ale_app::engine::{Engine};
use ale_math::rect::Rect;
use ale_math::{Vector2, Vector3, Zero};
use ale_render::target::{RenderTarget, RenderTargetType};
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_world::components::Spawnable;
use ale_world::event::world::{SpawnCommand, WorldCommand};
use ale_world::world::World;
use crate::camera_2d::Camera2D;
use crate::piece::Piece;
use crate::tetris::Game;

mod tetris;
mod template;
mod piece;
mod camera_2d;

struct Tetris;

pub enum TetrisEvent{}

impl Genesis for Tetris {

  fn register_components(&self, world: &mut World) {
    Camera2D::register_components(world);
    Game::register_components(world);
    Piece::register_components(world);
  }

  fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), AppError> {

    // Create camera, window, and add to render target
    let window = engine.windows.add(DisplaySetting {
      dimension: Rect {
        position: Vector2::new(0, 0),
        size: Vector2::new(800, 600),
      },
      initial_target: TargetMonitor::PRIMARY,
      is_hidden: false,
    });

    let camera_2d = Camera2D::new(Vector3::new(0.0, 0.0, 5.0), Rect{
      position: Vector2::zero(),
      size: Vector2::new(800, 600)
    });
    let main_render_target = engine.render_targets.push(RenderTarget{
      camera: camera_2d.id(),
      render_target_type: RenderTargetType::Window(window)
    });

    // world command
    let wc_sender = world.get_world_command_sender();

    // create world
    let tetris = tetris::Game::new(wc_sender.clone(), main_render_target);

    // send spawn commands to world
    wc_sender.send(WorldCommand::Spawn(SpawnCommand::new(camera_2d)));
    wc_sender.send(WorldCommand::Spawn(SpawnCommand::new(tetris)));

    Ok(())
  }
}

fn main() {
  App::new(Tetris).run();
}
