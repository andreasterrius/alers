use std::collections::HashMap;
use log::info;
use ale_data::alevec::Key;
use ale_data::channel::{Channel, Sender};
use ale_data::entity::Entity;
use ale_data::indexmap::Id;
use ale_data::wire_component;
use ale_math::color::Color;
use ale_math::{Vector2, Zero};
use ale_opengl::renderer::task::{RenderTask, Sprite};
use ale_render::component::Renderable;
use ale_render::target::RenderTarget;
use ale_world::components::{Spawnable};
use ale_world::world::{World};

use crate::template::BlockTypeId;
use crate::tetris::GameEvent;

pub enum PieceEvent {}

pub struct Piece {
  id: Id<Entity>,
  rotation_type: usize,
  block_type: BlockTypeId,
  blocks_template: Vec<Vec<Vec<i8>>>,

  render_target : Key<RenderTarget>,

  game_events: Sender<GameEvent>,
  pub piece_events: Channel<PieceEvent>,
}

impl Piece {
  pub fn register_components(world: &mut World) {
    world.register_components(&[
      wire_component!(dyn Spawnable, Piece),
      wire_component!(dyn Renderable, Piece),
    ]);
  }

  pub fn new(
    block_type: BlockTypeId,
    rotation_type: usize,
    blocks_template: Vec<Vec<Vec<i8>>>,
    render_target : Key<RenderTarget>,
    game_events: Sender<GameEvent>,
  ) -> Piece {
    Piece {
      id: Id::new(),
      rotation_type,
      block_type,
      blocks_template,
      render_target,
      game_events,
      piece_events: Channel::new(),
    }
  }
}

impl Spawnable for Piece {
  fn on_spawn(&mut self) {
    info!("piece on spawn called");
  }

  fn on_kill(&mut self) {}

  fn id(&self) -> Id<Entity> {
    self.id
  }
}

impl Renderable for Piece {
  fn get_render_task(&mut self) -> HashMap<Key<RenderTarget>, Vec<RenderTask>> {

    let mut renderables = vec![];
    for i in 0..4 {
      let coord = 0.0 + (i as f32)*20.0;
      renderables.push(RenderTask::Sprite(Sprite{
        texture_sprite: None,
        color: Color::red(),
        position: Vector2::new(coord, coord),
        size: Vector2::new(1000.0, 2000.0)
      }));
    }

    let mut map = HashMap::new();
    map.insert(self.render_target, renderables);
    map
  }
}