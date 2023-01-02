use ale_data::channel::Sender;
use std::collections::HashMap;

use ale_data::indexmap::Key;
use ale_world::components::{Spawnable, Tickable};
use ale_world::event::world::WorldCommand;
use ale_world::wire_component;
use ale_world::world::{Entity, World};

use crate::tetris::Block::NotFilled;
use crate::TetrisEvent;

#[derive(Clone)]
pub enum Block {
  Filled,
  NotFilled,
}

pub struct Game {
  pub key: Key<Entity>,
  pub blocks: Vec<Vec<Block>>,
  pub current: Option<Vec<Vec<Block>>>,

  pub elapsed_time: f32,
  pub move_down_time: f32,

  pub score: i32,

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

    Game {
      key,
      blocks,
      current: None,
      elapsed_time: 0.0,
      move_down_time: 1.0,
      score: 0,
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
    self.elapsed_time += delta_time;
    if self.elapsed_time > self.move_down_time {
      // send move down event to all tetris blocks here
      self.elapsed_time = 0.0;
    }
    // check if blocks has 1 line, then we remove and add to score
  }
}

impl Spawnable for Game {
  fn on_spawn(&mut self) {}

  fn on_kill(&mut self) {}

  fn get_key(&self) -> Key<Entity> {
    self.key
  }
}
