use std::convert::TryInto;
use cgmath::Vector2;

#[derive(Debug, Clone)]
pub enum OriginPoint {
  UpperLeft,
  UpperRight,
  LowerLeft,
  LowerRight,
  Center,
}

#[derive(Debug, Clone)]
pub struct Rect {
  x: i32,
  y: i32,
  width: u32,
  height: u32,
  origin_point: OriginPoint,
}

impl Rect {
  pub fn new(width: u32, height: u32) -> Rect {
    Rect {
      x: 0,
      y: 0,
      width,
      height,
      origin_point: OriginPoint::UpperLeft,
    }
  }

  pub fn from_xy(x: i32, y: i32, width: u32, height: u32) -> Rect {
    Rect {
      x,
      y,
      width,
      height,
      origin_point: OriginPoint::UpperLeft,
    }
  }

  pub fn get_x(&self) -> i32 {
    self.x
  }

  pub fn get_y(&self) -> i32 {
    self.y
  }

  pub fn get_width(&self) -> u32 {
    self.width
  }

  pub fn get_height(&self) -> u32 {
    self.height
  }

  pub fn is_inside(&self, x : i32, y : i32) -> bool {
    let xleft = self.x;
    let xright = self.x + self.width as i32;
    if x < xleft || x > xright {
      return false;
    }

    let ytop = self.y;
    let ybot = self.y + self.height as i32;
    if y < ytop || y > ybot {
      return false;
    }

    return true;
  }
}
