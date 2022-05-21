use crate::layout::TableLayoutTypeError::RowNotFound;
use ale_math::{Array, Vector2, Zero};
use std::collections::HashMap;
use thiserror::Error;

pub enum LayoutType {
  NoLayout,
  TableLayout(TableLayoutType),
}

impl LayoutType {
  pub fn arrange(&mut self, parent: &Layout, mut childs: Vec<&mut Layout>) -> Result<(), LayoutError> {
    match self {
      LayoutType::NoLayout => Ok(()),
      LayoutType::TableLayout(tl) => Ok(tl.arrange(parent, childs)?),
    }
  }
}

#[derive(Error, Debug)]
pub enum LayoutError {
  #[error(transparent)]
  TableLayoutTypeError(#[from] TableLayoutTypeError),
}

pub struct TableLayoutType {
  column_dividers: Vec<Vec<f32>>,
  row_divider: Vec<f32>,
}

impl TableLayoutType {
  pub fn new() -> TableLayoutType {
    TableLayoutType {
      row_divider: vec![],
      column_dividers: vec![],
    }
  }

  pub fn new_divider(column_divider: Vec<Vec<f32>>, row_divider: Vec<f32>) -> TableLayoutType {
    TableLayoutType {
      row_divider,
      column_dividers: column_divider,
    }
  }

  pub fn add_row(&mut self, divider: f32) {
    self.row_divider.push(divider);
    self.column_dividers.push(vec![]);
  }

  pub fn add_column(&mut self, row_index: usize, divider: f32) -> Result<(), TableLayoutTypeError> {
    if row_index >= self.row_divider.len() {
      return Err(RowNotFound(row_index, self.row_divider.len()));
    }

    self.column_dividers[row_index].push(divider);
    Ok(())
  }

  pub fn arrange(&mut self, parent: &Layout, mut childs: Vec<&mut Layout>) -> Result<(), TableLayoutTypeError> {
    let mut row_percentage_total = 0.0f32;
    for row in &self.row_divider {
      row_percentage_total += row;
    }

    let mut child_index = 0;
    let mut upper_left_pos = Vector2::from_value(0);
    for (row_index, column_divider) in self.column_dividers.iter().enumerate() {
      let mut column_percentage_total = 0.0f32;
      for column in column_divider {
        column_percentage_total += column;
      }

      let row = match self.row_divider.get(row_index) {
        None => return Err(RowNotFound(row_index, self.row_divider.len())),
        Some(r) => r,
      };

      upper_left_pos.x = 0;
      for column in column_divider {
        let child = match childs.get_mut(child_index) {
          None => break,
          Some(ch) => ch,
        };

        // resize with percentage
        child.position = upper_left_pos;
        child.global_position = upper_left_pos + parent.global_position;
        child.size.x = ((column / column_percentage_total) * parent.size.x as f32) as u32;
        child.size.y = ((row / row_percentage_total) * parent.size.y as f32) as u32;
        child.global_size = child.size;

        upper_left_pos.x += child.size.x as i32;
        child_index += 1;
      }
      upper_left_pos.y += ((row / row_percentage_total) * parent.size.y as f32) as i32;
    }

    Ok(())
  }
}

#[derive(Error, Debug)]
pub enum TableLayoutTypeError {
  #[error("(TableLayoutTypeError::RowNotFound) Index:{}, Len:{}", .0, .1)]
  RowNotFound(usize, usize),
}

#[derive(Debug)]
pub struct Layout {
  pub position: Vector2<i32>,
  pub size: Vector2<u32>,

  pub global_position: Vector2<i32>,
  pub global_size: Vector2<u32>,
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

  pub fn new_local(local_position: Vector2<i32>, local_size: Vector2<u32>) -> Layout {
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
