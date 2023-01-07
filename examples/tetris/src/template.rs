use std::collections::HashMap;
use rand::Rng;
use crate::piece::Piece;
use enumn::N;
use ale_data::channel::Sender;
use ale_world::event::world::WorldCommand;
use crate::tetris::GameEvent;

#[derive(Eq, PartialEq, Hash, N)]
#[repr(usize)]
pub enum BlockTypeId {
  ZLeft = 0,
  ZRight = 1,
  I = 2,
}

pub struct RandomTetrisInfo {
  pub block_type: BlockTypeId,
  pub rotation_type: usize,
  pub blocks_template: Vec<Vec<Vec<i8>>>,
}

pub struct Templates {
  blocks: HashMap<BlockTypeId, Vec<Vec<Vec<i8>>>>,
}

impl Templates {
  pub fn new() -> Templates {
    Templates { blocks: HashMap::new() }
  }

  pub fn add_all(&mut self) {
    self.add_zleft();
    self.add_zright();
  }

  pub fn add_zleft(&mut self) {
    self.blocks.insert(
      BlockTypeId::ZLeft,
      vec![
        // Z rotation 0
        vec![
          vec![1, 1, 0], //
          vec![0, 1, 1], //
          vec![0, 0, 0],
        ],
        // Z rotation 1
        vec![
          vec![0, 0, 1], //
          vec![0, 1, 1], //
          vec![0, 1, 0],
        ],
      ],
    );
  }

  pub fn add_zright(&mut self) {
    self.blocks.insert(
      BlockTypeId::ZRight,
      vec![
        // Z rotation 0
        vec![
          vec![0, 1, 1], //
          vec![1, 1, 0], //
          vec![0, 0, 0],
        ],
        // Z rotation 1
        vec![
          vec![1, 0, 0], //
          vec![1, 1, 0], //
          vec![0, 1, 0],
        ],
      ],
    );
  }

  pub fn random_one_piece(&self) -> RandomTetrisInfo {
    let mut rand = rand::thread_rng();
    let block_type = BlockTypeId::n(rand.gen_range(0..self.blocks.len())).unwrap();

    let blocks_template = self.blocks.get(&block_type).unwrap().clone();
    let rotation_type = rand.gen_range(0..blocks_template.len());

    RandomTetrisInfo {
      blocks_template,
      rotation_type,
      block_type
    }
  }
}
