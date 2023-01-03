use ale_data::channel::Sender;
use std::collections::HashMap;

use ale_data::indexmap::Id;
use ale_data::timer::{Recurrence, Timer};
use ale_math::{Vector2, Vector3, Zero};
use ale_world::components::{Spawnable, Tickable};
use ale_world::event::world::WorldCommand;
use ale_world::wire_component;
use ale_world::world::{Entity, World};
use crate::piece::Piece;
use crate::template::{BlockTypeId, Templates};

use crate::tetris::Block::NotFilled;
use crate::TetrisEvent;

#[derive(Clone)]
pub enum Block {
  Filled(Id<Entity>),
  NotFilled,
}

pub struct Game {
  pub id: Id<Entity>,
  pub piece_templates: Templates,
  pub wc_sender: Sender<WorldCommand>,

  // Arena state
  pub arena : Vec<Vec<Block>>,

  // Current selection
  pub curr_selection : Option<Piece>,
  pub curr_selection_location : Vector2<i32>,

  pub tetris_timer : Timer,
}

impl Game {
  pub fn register_components(world: &mut World) {
    world.register_components(&[
      wire_component!(dyn Spawnable, Game),
      wire_component!(dyn Tickable, Game),
    ]);
  }

  pub fn new(wc_sender: Sender<WorldCommand>) -> Game {
    let width = 10;
    let height = 24;

    let arena = vec![vec![NotFilled; width]; height];
    let mut templates = Templates::new();
    templates.add_all();

    Game {
      id: Id::new(),
      piece_templates: templates,
      wc_sender,
      arena,
      curr_selection: None,
      curr_selection_location: Vector2::zero(),
      tetris_timer: Timer::new(0.2, Recurrence::Forever),
    }
  }

  pub fn try_select_random(&mut self) {
    if self.curr_selection.is_none() {
      let piece = self.piece_templates.get_one_random();
    }
  }

  pub fn move_pieces_down(&mut self) {

  }
}

impl Tickable for Game {
  fn fixed_tick(&mut self, delta_time: f32) {
    // do nothing
  }

  fn tick(&mut self, delta_time: f32) {
    if self.curr_selection.is_none() {
      self.try_select_random();
    }

    if self.tetris_timer.tick_and_check(delta_time) {
      self.move_pieces_down();
    }
  }
}

impl Spawnable for Game {
  fn on_spawn(&mut self) {
  }

  fn on_kill(&mut self) {}

  fn id(&self) -> Id<Entity> {
    self.id
  }
}
