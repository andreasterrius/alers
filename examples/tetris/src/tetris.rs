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
use ale_world::components::{Inputable, Spawnable, Tickable};
use ale_world::event::world::WorldCommand;
use ale_world::world::World;
use Block::Placed;

use crate::template::{BlockTypeId, Templates};
use crate::tetris::Block::{NotFilled, Ongoing};

const TICK_TIME: f32 = 0.5;
const HIDDEN_ROW_GRID_SIZE: usize = 4;
const ROW_GRID_SIZE: usize = 28;
const COLUMN_GRID_SIZE: usize = 10;
const BLOCK_SIZE: Vector2<usize> = Vector2::new(20, 20);
const INPUT_STICKY_TIME: f32 = 0.1;

#[derive(Clone)]
pub enum Block {
  Ongoing(Color),
  NotFilled,
  Placed(Color),
}

pub struct TetrisInfo {
  pub block_type: BlockTypeId,
  pub rotation_type: usize,
  pub position: Vector2<isize>,
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
  pub move_left_timer: Timer,
  pub right_is_pressed: bool,
  pub rotate_is_pressed: bool,
  pub speed_is_pressed: bool,

  pub move_down_timer: Timer,
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
      move_down_timer: Timer::new(TICK_TIME, Recurrence::Forever),
      move_left_timer: Timer::new_paused(INPUT_STICKY_TIME, Recurrence::Forever),
      right_is_pressed: false,
      rotate_is_pressed: false,
      speed_is_pressed: false,
    }
  }
}

struct Selected;

impl Selected {
  pub fn run(
    tetris_info: &mut TetrisInfo,
    templates: &mut Templates,
    arena: &mut Vec<Vec<Block>>,
    mut f: Vec<impl FnMut(usize, usize, i8, &mut Vec<Vec<Block>>, &mut TetrisInfo)>,
  ) {
    let blocks = templates
      .blocks
      .get(&tetris_info.block_type)
      .expect("unexpected block type id")
      .get(tetris_info.rotation_type)
      .expect("unexpected rotation type");

    for row in 0..blocks.len() {
      for column in 0..blocks[row].len() {
        // de-paint old grid
        let x = tetris_info.position.x + row as isize;
        let y = tetris_info.position.y + column as isize;
        let is_filled = blocks[column][row];

        for func in &mut f {
          if x < 0 || y < 0 { return; }
          func(x as usize, y as usize, is_filled, arena, tetris_info);
        }
      }
    }
  }

  pub fn depaint(x: usize, y: usize, is_filled: i8, arena: &mut Vec<Vec<Block>>, tetris_info: &mut TetrisInfo) {
    match arena[y][x] {
      Ongoing(_) => arena[y][x] = NotFilled,
      _ => {}
    };
  }

  pub fn paint(x: usize, y: usize, is_filled: i8, arena: &mut Vec<Vec<Block>>, tetris_info: &mut TetrisInfo) {
    if is_filled == 0 {
      return;
    };
    arena[y][x] = Ongoing(tetris_info.color);
  }

  pub fn place(x: usize, y: usize, is_filled: i8, arena: &mut Vec<Vec<Block>>, tetris_info: &mut TetrisInfo) {
    if is_filled == 0 {
      return;
    };
    arena[y][x] = Placed(tetris_info.color);
  }
}

impl Tickable for GameCoordinator {
  fn fixed_tick(&mut self, delta_time: f32) {
    // do nothing
  }

  fn tick(&mut self, delta_time: f32) {
    if self.selected.is_none() {
      let random = self.templates.random_one_piece();
      self.selected = Some(TetrisInfo {
        block_type: random.block_type,
        rotation_type: random.rotation_type,
        position: Vector2::new(COLUMN_GRID_SIZE as isize / 2, 0),
        color: random.color,
      });
    }

    // try to move down + place accordingly
    let mut should_place = false;
    if self.move_down_timer.tick_and_check(delta_time) {
      match &mut self.selected {
        None => {}
        Some(tetris_info) => {
          Selected::run(
            tetris_info,
            &mut self.templates,
            &mut self.arena,
            vec![Selected::depaint],
          );
          Selected::run(
            tetris_info,
            &mut self.templates,
            &mut self.arena,
            vec![
              |x: usize, y: usize, is_filled: i8, arena: &mut Vec<Vec<Block>>, tetris_info: &mut TetrisInfo| {
                if is_filled == 0 || should_place {
                  return;
                }
                if y + 1 >= ROW_GRID_SIZE {
                  //hit the bottom
                  should_place = true;
                  return;
                } else {
                  match arena[y + 1][x] {
                    // otherwise check other pieces
                    Placed(color) => should_place = true,
                    _ => {}
                  }
                }
              },
            ],
          );
          if should_place {
            Selected::run(tetris_info, &mut self.templates, &mut self.arena, vec![Selected::place]);
          } else {
            tetris_info.position.y += 1;
            Selected::run(tetris_info, &mut self.templates, &mut self.arena, vec![Selected::paint]);
          }
        }
      }
    }
    if should_place {
      // was placed before
      self.selected = None;
    }

    // try to move left/right accordingly
    if self.move_left_timer.tick_and_check(delta_time) {
      match &mut self.selected {
        None => {}
        Some(tetris_info) => {
          let mut valid_move = true;
          Selected::run(
            tetris_info,
            &mut self.templates,
            &mut self.arena,
            vec![
              |x: usize, y: usize, is_filled: i8, arena: &mut Vec<Vec<Block>>, tetris_info: &mut TetrisInfo| {
                if is_filled == 0 || !valid_move {
                  return;
                }
                if x == 0 {
                  valid_move = false;
                } else {
                  match arena[y][x - 1] {
                    Placed(color) => valid_move = false,
                    _ => {}
                  }
                }
              },
            ],
          );
          if(valid_move) {
            Selected::run(
              tetris_info,
              &mut self.templates,
              &mut self.arena,
              vec![Selected::depaint]
            );
            tetris_info.position.x -= 1;
            Selected::run(tetris_info, &mut self.templates, &mut self.arena, vec![Selected::paint]);
          }
        }
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
  fn input(&mut self, inputs: &Vec<Input>) {
    for input in inputs {
      match input {
        Input::Key(Left, _, Press, _) => {
          self.move_left_timer.reset_all();
          self.move_left_timer.set_paused(false);
          self.move_left_timer.force_one_tick();
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
          self.move_left_timer.reset_all();
          self.move_left_timer.set_paused(true);
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
