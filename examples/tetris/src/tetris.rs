use ale_data::channel::Sender;
use std::collections::HashMap;

use ale_data::indexmap::Key;
use ale_world::components::{Spawnable, Tickable};
use ale_world::event::world::WorldCommand;
use ale_world::wire_component;
use ale_world::world::{Entity, World};
use crate::template::Templates;

use crate::tetris::Block::NotFilled;
use crate::TetrisEvent;

#[derive(Clone)]
pub enum Block {
  Filled(Key<Entity>),
  NotFilled,
}

pub struct Game {
  pub key: Key<Entity>,

  pub templates : Templates,

  pub world_cmd_chan: Sender<WorldCommand>,
  pub block_chans: HashMap<Key<Entity>, Sender<TetrisEvent>>,
}

impl Game {
  pub fn register_components(world: &mut World) {
    world.register_components(&[
      wire_component!(dyn Spawnable, Game),
      wire_component!(dyn Tickable, Game),
    ]);
  }

  pub fn new(key: Key<Entity>, world_cmd_chan: Sender<WorldCommand>) -> Game {
    let width = 10;
    let height = 24;

    let blocks = vec![vec![NotFilled; width]; height];
    let mut templates = Templates::new();
    templates.add_all();

    Game {
      key,
      templates ,
      world_cmd_chan,
      block_chans: HashMap::new(),
    }
  }

}

impl Tickable for Game {
  fn fixed_tick(&mut self, delta_time: f32) {
    // do nothing
  }

  fn tick(&mut self, delta_time: f32) {
    // do nothing
  }
}

impl Spawnable for Game {
  fn on_spawn(&mut self) {}

  fn on_kill(&mut self) {}

  fn get_key(&self) -> Key<Entity> {
    self.key
  }
}
