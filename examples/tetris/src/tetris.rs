
use std::any::{Any, TypeId};
use std::collections::HashMap;
use log::info;
use ale_data::indexmap::Key;

use ale_data::queue::fast::Sender;
use ale_world::components::{Error, EventListener, Renderable, Spawnable, Tick};
use ale_world::typecast::entry::Traitcast;
use ale_world::wire_component;
use ale_world::world::{Entity, EntityEvent, Event, World};

use crate::{Tetris, TetrisEvent};
use crate::tetris::Block::NotFilled;

#[derive(Clone)]
pub enum Block {
  Filled,
  NotFilled,
}

pub struct Game {
  pub key : Key<Entity>,
  pub blocks: Vec<Vec<Block>>,
  pub current: Option<Vec<Vec<Block>>>,

  pub elapsed_time: f32,
  pub move_down_time: f32,

  pub score: i32,
}

impl Game {
  pub fn register_components(world: &mut World) {
    world.register_components(&[
      wire_component!(dyn Tick, Game),
    ]);
  }

  pub fn new(key : Key<Entity>) -> Game {
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
    }
  }
}

impl Tick for Game {
  fn fixed_tick(&mut self, delta_time: f32) {
    // do nothing
  }

  fn tick(&mut self, delta_time: f32) {
    self.elapsed_time += delta_time;
    if self.elapsed_time > self.move_down_time {
      // send move down event to all tetris blocks here
      self.elapsed_time = 0.0;
      // self.sender.send(EntityEvent::broadcast(MoveDownEvent {
      //   counter: self.elapsed_time
      // })).unwrap();
    }
    // check if blocks has 1 line, then we remove and add to score
  }
}