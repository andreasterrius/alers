use ale_math::Vector2;
use std::collections::HashMap;

pub enum Layout {
  NoLayout,
  TablePercentageLayout(TablePercentageLayout),
}

pub struct LayoutData {
  pub position: Vector2<i32>,
  pub size: Vector2<u32>,
}

pub struct TablePercentageLayout {
  divider: Vec<Vec<f32>>,
  lookup: Vec<(usize, usize)>,
}

impl TablePercentageLayout {
  pub fn new(divider: Vec<Vec<f32>>) -> TablePercentageLayout {
    let mut layout = TablePercentageLayout { divider, lookup: vec![] };
    layout.build_lookup();
    layout
  }

  pub fn build_lookup(&mut self){
    let mut index = 0;
    self.lookup.clear();
    for (i, row) in self.divider.iter().enumerate() {
      for (j, column) in row.iter().enumerate() {
        self.lookup.push((i, j));
      }
    }
  }

  // pub fn retrieve(&mut self, num: usize, LayoutData: usize) -> Option<LayoutData> {
  //
  // }
}
