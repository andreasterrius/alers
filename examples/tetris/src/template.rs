use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
pub enum BlockTypeId {
  ZLeft = 0,
  ZRight = 1,
  I = 2,
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
}
