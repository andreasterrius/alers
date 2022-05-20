use ale_math::{Vector2, Zero};
use std::collections::HashMap;


pub enum LayoutType {
  NoLayout,
  TableLayout(TableLayoutType),
}

impl LayoutType {
  pub fn arrange(&mut self, childs : Vec<&mut Layout>) {
    match self {
      LayoutType::NoLayout => {}
      LayoutType::TableLayout(tl) => tl.arrange(childs)
    }
  }
}


pub struct TableLayoutType {
  divider: Vec<Vec<f32>>,
  lookup: Vec<(usize, usize)>,
}

impl TableLayoutType {
  pub fn new(divider: Vec<Vec<f32>>) -> TableLayoutType {
    let mut layout = TableLayoutType { divider, lookup: vec![] };
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

  pub fn arrange(&mut self, childs: Vec<&mut Layout>) {

  }
}


pub struct Layout {
  pub position: Vector2<i32>,
  pub size: Vector2<u32>,

  pub global_position : Vector2<i32>,
  pub global_size : Vector2<u32>
}

impl Layout {
  pub fn new() -> Layout {
    Layout {
      position: Vector2::zero(),
      size: Vector2::zero(),
      global_position: Vector2::zero(),
      global_size: Vector2::zero(),
    }
  }

  pub fn new_local(local_position : Vector2<i32>, local_size : Vector2<u32>) -> Layout {
    Layout {
      position: local_position,
      size: local_size,
      global_position: local_position,
      global_size: local_size,
    }
  }

  pub fn is_inside(&self, x: i32, y: i32) -> bool {
    let xleft = self.position.x;
    let xright = self.position.x + self.size.x as i32;
    if x < xleft || x > xright {
      return false;
    }

    let ytop = self.position.y;
    let ybot = self.position.y + self.size.y as i32;
    if y < ytop || y > ybot {
      return false;
    }

    return true;
  }
}