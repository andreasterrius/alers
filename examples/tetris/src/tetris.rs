use log::info;

use ale_data::channel::{Channel, Sender};
use ale_data::entity::Entity;
use ale_data::indexmap::Id;
use ale_data::timer::{Recurrence, Timer};
use ale_data::wire_component;
use ale_input::Action::Press;
use ale_input::Input;
use ale_input::Key::{Left, Right};
use ale_math::color::Color;
use ale_math::Vector2;
use ale_opengl::renderer::task::{RenderTask, Sprite};
use ale_render::component::Renderable;
use ale_world::components::{Inputable, Spawnable, Tickable};
use ale_world::event::world::{SpawnCommand, WorldCommand};
use ale_world::world::World;

use crate::piece::{Piece, PieceEvent};
use crate::template::Templates;
use crate::tetris::Block::NotFilled;

const TICK_TIME: f32 = 0.2;
const ROW_GRID_SIZE: usize = 20;
const COLUMN_GRID_SIZE: usize = 10;
const BLOCK_SIZE: Vector2<usize> = Vector2::new(5, 5);

pub enum GameEvent {}

#[derive(Clone)]
pub enum Block {
  Filled(Color),
  NotFilled,
}

pub struct GameCoordinator {
  pub id: Id<Entity>,
  pub piece_templates: Templates,
  pub wc_sender: Sender<WorldCommand>,

  // Arena state
  pub arena: Vec<Vec<Block>>,

  // Current selection
  pub curr_piece_event: Option<Sender<PieceEvent>>,

  pub tetris_timer: Timer,
  pub game_events: Channel<GameEvent>,
}

impl GameCoordinator {
  pub fn register_components(world: &mut World) {
    world.register_components(&[
      wire_component!(dyn Spawnable, GameCoordinator),
      wire_component!(dyn Tickable, GameCoordinator),
      wire_component!(dyn Inputable, GameCoordinator),
      wire_component!(dyn Renderable, GameCoordinator),
    ]);
  }

  pub fn new(wc_sender: Sender<WorldCommand>) -> GameCoordinator {
    let width = 10;
    let height = 24;

    let arena = vec![vec![NotFilled; COLUMN_GRID_SIZE]; ROW_GRID_SIZE];
    let mut templates = Templates::new();
    templates.add_all();

    GameCoordinator {
      id: Id::new(),
      piece_templates: templates,
      wc_sender,
      arena,
      curr_piece_event: None,
      tetris_timer: Timer::new(TICK_TIME, Recurrence::Forever),
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
        self.game_events.sender.clone(),
      );

      self.curr_piece_event = Some(piece.piece_events.sender.clone());
      self.wc_sender.send(WorldCommand::Spawn(SpawnCommand::new(piece)));
    }
  }

  pub fn spawn_blocks(&mut self) {}

  pub fn move_pieces_down(&mut self) {}
}

impl Tickable for GameCoordinator {
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

impl Spawnable for GameCoordinator {
  fn on_spawn(&mut self) {
    info!("on spawn enter");
    self.try_select_random();

    // just test
    self.arena[5][5] = Block::Filled(Color::green());
  }

  fn on_kill(&mut self) {}

  fn id(&self) -> Id<Entity> {
    self.id
  }
}

impl Inputable for GameCoordinator {
  fn input(&mut self, input: Input) {
    match input {
      Input::Key(Left, _, Press, _) => {
        println!("left is pressed");
      }
      Input::Key(Right, _, Press, _) => {
        println!("right is pressed");
      }
      _ => {}
    }
  }
}

impl Renderable for GameCoordinator {
  fn get_render_tasks(&mut self) -> Vec<RenderTask> {
    let mut renderables = vec![];
    for (rowIndex, row) in self.arena.iter().enumerate() {
      for (columnIndex, block) in row.iter().enumerate() {
        match block {
          Block::Filled(color) => {
            renderables.push(RenderTask::Sprite(Sprite {
              texture_sprite: None,
              color: *color,
              position: Vector2::new((columnIndex * BLOCK_SIZE.x) as f32, (rowIndex * BLOCK_SIZE.y) as f32),
              size: Vector2::new(400 as f32, 400 as f32),
            }));
          }
          NotFilled => {} //just skip
        }
      }
    }

    renderables.push(RenderTask::Sprite(Sprite {
      texture_sprite: None,
      color: Color::red(),
      position: Vector2::new(0.0, 0.0),
      size: Vector2::new(400 as f32, 400 as f32),
    }));

    renderables
  }
}
