use ale_data::channel::Sender;
use ale_data::entity::Entity;
use ale_data::indexmap::Id;
use ale_data::timer::{Recurrence, Timer};
use ale_data::wire_component;
use ale_input::Action::{Press, Release};
use ale_input::Input;
use ale_input::Key::{Down, Left, Right, Space};
use ale_math::color::Color;
use ale_math::Vector2;
use ale_opengl::renderer::task::{RenderTask, Sprite};
use ale_render::component::Renderable;
use ale_resources::texture::LoadError::FileNotFound;
use ale_world::components::{Inputable, Spawnable, Tickable};
use ale_world::event::world::WorldCommand;
use ale_world::world::World;
use Block::Placed;

use crate::template::{BlockTypeId, Templates};
use crate::tetris::Block::{NotFilled, Ongoing};

const TICK_TIME: f32 = 0.2;
const HIDDEN_ROW_GRID_SIZE: usize = 4;
const ROW_GRID_SIZE: usize = 28;
const COLUMN_GRID_SIZE: usize = 10;
const BLOCK_SIZE: Vector2<usize> = Vector2::new(20, 20);

#[derive(Clone)]
pub enum Block {
  Ongoing(Color),
  NotFilled,
  Placed(Color),
}

pub struct TetrisInfo {
  pub block_type: BlockTypeId,
  pub rotation_type: usize,
  pub position: Vector2<usize>,
  pub color: Color,
}

pub struct GameCoordinator {
  pub id: Id<Entity>,
  pub templates: Templates,
  pub wc_sender: Sender<WorldCommand>,

  // Arena state
  pub arena: Vec<Vec<Block>>,
  pub selected: Option<TetrisInfo>,

  // Input states
  pub left_is_pressed: bool,
  pub right_is_pressed: bool,
  pub rotate_is_pressed: bool,
  pub speed_is_pressed: bool,

  pub tetris_timer: Timer,
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
    let arena = vec![vec![NotFilled; COLUMN_GRID_SIZE]; ROW_GRID_SIZE + HIDDEN_ROW_GRID_SIZE];
    let mut templates = Templates::new();
    templates.add_all();

    GameCoordinator {
      id: Id::new(),
      templates,
      wc_sender,
      arena,
      selected: None,
      tetris_timer: Timer::new(TICK_TIME, Recurrence::Forever),
      left_is_pressed: false,
      right_is_pressed: false,
      rotate_is_pressed: false,
      speed_is_pressed: false,
    }
  }
}

impl Tickable for GameCoordinator {
  fn fixed_tick(&mut self, delta_time: f32) {
    // do nothing
  }

  fn tick(&mut self, delta_time: f32) {
    //
    if self.selected.is_none() {
      let random = self.templates.random_one_piece();
      self.selected = Some(TetrisInfo {
        block_type: random.block_type,
        rotation_type: random.rotation_type,
        position: Vector2::new(COLUMN_GRID_SIZE / 2, 0),
        color: random.color,
      });
    }

    if self.tetris_timer.tick_and_check(delta_time) {
      let mut should_place = false;
      match &mut self.selected {
        None => {}
        Some(tetris_info) => {
          let blocks = self
            .templates
            .blocks
            .get(&tetris_info.block_type)
            .expect("unexpected block type id")
            .get(tetris_info.rotation_type)
            .expect("unexpected rotation type");

          // de-paint the grid
          // check whether block should be placed or not
          for row in 0..blocks.len() {
            for column in 0..blocks[row].len() {
              // de-paint old grid
              let x = tetris_info.position.x + row;
              let y = tetris_info.position.y + column;
              match self.arena[y][x] {
                Ongoing(_) => self.arena[y][x] = NotFilled,
                _ => {}
              };

              // check block below this
              if blocks[column][row] == 0 {
                continue;
              }
              if y + 1 >= ROW_GRID_SIZE {
                should_place = true;
                continue;
              }
              match self.arena[y + 1][x] {
                Placed(_) => should_place = true,
                _ => {}
              }
            }
          }

          // place
          if (!should_place) {
            tetris_info.position.y += 1;
          }

          //re-paint the grid
          for row in 0..blocks.len() {
            for column in 0..blocks[row].len() {
              if blocks[column][row] == 0 {
                continue;
              }
              let x = tetris_info.position.x + row;
              let y = tetris_info.position.y + column;
              if should_place {
                self.arena[y][x] = Placed(tetris_info.color);
              } else {
                self.arena[y][x] = Ongoing(tetris_info.color);
              }
            }
          }

          // destroy lines
        }
      }

      if should_place {
        // this has been placed before
        self.selected = None;
      }
    }
  }
}

impl Spawnable for GameCoordinator {
  fn on_spawn(&mut self) {}

  fn on_kill(&mut self) {}

  fn id(&self) -> Id<Entity> {
    self.id
  }
}

impl Inputable for GameCoordinator {
  fn input(&mut self, input: Input) {
    match input {
      Input::Key(Left, _, Press, _) => {
        self.left_is_pressed = true;
      }
      Input::Key(Right, _, Press, _) => {
        self.right_is_pressed = true;
      }
      Input::Key(Space, _, Press, _) => {
        self.rotate_is_pressed = true;
      }
      Input::Key(Down, _, Press, _) => {
        self.speed_is_pressed = true;
      }
      Input::Key(Left, _, Release, _) => {
        self.left_is_pressed = false;
      }
      Input::Key(Right, _, Release, _) => {
        self.right_is_pressed = false;
      }
      Input::Key(Space, _, Release, _) => {
        self.rotate_is_pressed = false;
      }
      Input::Key(Down, _, Release, _) => {
        self.speed_is_pressed = false;
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
          Ongoing(color) | Placed(color) => {
            renderables.push(RenderTask::Sprite(Sprite {
              texture_sprite: None,
              color: *color,
              position: Vector2::new((columnIndex * BLOCK_SIZE.x) as f32, (rowIndex * BLOCK_SIZE.y) as f32),
              size: Vector2::new(BLOCK_SIZE.x as f32, BLOCK_SIZE.y as f32),
            }));
          }
          NotFilled => {} //just skip
        }
      }
    }

    renderables
  }
}
