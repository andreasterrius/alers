use ale_data::indexmap::Id;
use ale_opengl::renderer::task::Task;
use ale_world::components::Renderable;
use ale_world::world::Entity;
use crate::template::BlockTypeId;

pub struct Piece {
  entities: Vec<Id<Entity>>,
  rotation_type: usize,
  block_type: BlockTypeId,
}

impl Piece {
  pub fn new(block_type: BlockTypeId, rotation_type: usize) -> Piece {
    Piece {
      entities: vec![],
      rotation_type,
      block_type,
    }
  }
}