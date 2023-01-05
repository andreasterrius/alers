use ale_data::channel::Sender;
use ale_data::indexmap::Id;
use ale_opengl::renderer::task::Task;
use ale_world::components::Renderable;
use ale_world::event::world::WorldCommand;
use ale_world::world::Entity;
use crate::template::BlockTypeId;

pub struct Piece {
  entities: Vec<Id<Entity>>,
  rotation_type: usize,
  block_type: BlockTypeId,
  block_template : Vec<Vec<Vec<i8>>>
}

impl Piece {
  pub fn new(block_type: BlockTypeId, rotation_type: usize, block_template: Vec<Vec<Vec<i8>>>) -> Piece {
    Piece {
      entities: vec![],
      rotation_type,
      block_type,
      block_template
    }
  }

  pub fn spawn_blocks(&mut self, wc_sender: Sender<WorldCommand>) {
    let mut blocks = vec![];
    for row in self.block_template[self.rotation_type]{
      for cell in row {
        //let block = Block::new();
        //wc_sender.send(WorldCommand::Spawn())
      }
    }

  }
}