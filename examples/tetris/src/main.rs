#![feature(const_type_id)]

use ale_app::app::{App, Genesis};
use ale_app::AppError;
use ale_app::engine::Engine;
use ale_data::queue::fast::FastQueue;
use ale_math::rect::Rect;
use ale_math::Vector2;
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_world::events::Events;
use ale_world::event::EventBuffer;
use ale_world::world::{Event, World};

mod tetris;

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

    let entity_events: Event<TetrisEvent> = world.get_entity_event_stream(1000);
    let world_events = world.get_world_event_stream();

    let tetris = tetris::Game::new(world.gen_entity_key(), entity_events);
    world.spawn(tetris);

    Ok(())
  }
}

fn main() {
  App::new(Tetris).run();
}
