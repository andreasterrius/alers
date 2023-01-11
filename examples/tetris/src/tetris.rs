use ale_data::alevec::Key;
use ale_data::channel::{Channel, Sender};
use ale_data::entity::entry::ComponentEntry;
use ale_data::entity::Entity;
use log::info;
use std::collections::HashMap;

use crate::piece::{Piece, PieceEvent};
use crate::template::{BlockTypeId, Templates};
use ale_data::indexmap::Id;
use ale_data::timer::{Recurrence, Timer};
use ale_data::wire_component;
use ale_math::{Vector2, Vector3, Zero};
use ale_render::target::RenderTarget;
use ale_world::components::{Spawnable, Tickable};
use ale_world::event::world::{SpawnCommand, WorldCommand};
use ale_world::world::World;

use crate::tetris::Block::NotFilled;
use crate::TetrisEvent;

pub enum GameEvent {}

#[derive(Clone)]
pub enum Block {
  Filled(Id<Entity>),
  NotFilled,
}

pub struct Game {
  pub id: Id<Entity>,
  pub piece_templates: Templates,
  pub wc_sender: Sender<WorldCommand>,

  pub render_target: Key<RenderTarget>,

  // Arena state
  pub arena: Vec<Vec<Block>>,

  // Current selection
  pub curr_piece_event: Option<Sender<PieceEvent>>,

  pub tetris_timer: Timer,
  pub game_events: Channel<GameEvent>,
}

impl Game {
  pub fn components() -> Vec<ComponentEntry> {
    vec![
      wire_component!(dyn Spawnable, Game),
      wire_component!(dyn Tickable, Game),
    ]
  }

  pub fn new(wc_sender: Sender<WorldCommand>, render_target: Key<RenderTarget>) -> Game {
    let width = 10;
    let height = 24;

    let arena = vec![vec![NotFilled; width]; height];
    let mut templates = Templates::new();
    templates.add_all();

    Game {
      id: Id::new(),
      piece_templates: templates,
      wc_sender,
      render_target,
      arena,
      curr_piece_event: None,
      tetris_timer: Timer::new(0.2, Recurrence::Forever),
      game_events: Channel::new(),
    }
  }

  pub fn try_select_random(&mut self) {
    if self.curr_piece_event.is_none() {
      let random_tetris_info = self.piece_templates.random_one_piece();
      let piece = Piece::new(
        random_tetris_info.block_type,
        random_tetris_info.rotation_type,
        random_tetris_info.blocks_template,
        self.render_target,
        self.game_events.sender.clone(),
      );

      self.curr_piece_event = Some(piece.piece_events.sender.clone());
      self.wc_sender.send(WorldCommand::Spawn(SpawnCommand::new(piece)));
    }
  }

  pub fn spawn_blocks(&mut self) {}

  pub fn move_pieces_down(&mut self) {}
}

impl Tickable for Game {
  fn fixed_tick(&mut self, delta_time: f32) {
    // do nothing
  }

  fn tick(&mut self, delta_time: f32) {
    match &mut self.curr_piece_event {
      None => {
        self.tetris_timer.reset_current_time();
        self.try_select_random();
      }
      Some(selected_piece) => {
        if self.tetris_timer.tick_and_check(delta_time) {
          //selected_piece.move_down();
        }
      }
    }

    if self.curr_piece_event.is_none() {}

    if self.tetris_timer.tick_and_check(delta_time) {
      self.move_pieces_down();
    }
  }
}

impl Spawnable for Game {
  fn on_spawn(&mut self) {
    info!("on spawn enter");
    self.try_select_random();
  }

  fn on_kill(&mut self) {}

  fn id(&self) -> Id<Entity> {
    self.id
  }
}
