#[derive(Debug, Clone)]
pub struct Rect2d {
  x: i32,
  y: i32,
  width: u32,
  height: u32,
}

impl Rect2d {
  pub fn new(width: u32, height: u32) -> Rect2d {
    Rect2d {
      x: 0,
      y: 0,
      width,
      height
    }
  }

  pub fn new_with_xy(x: i32, y: i32, width: u32, height: u32) -> Rect2d {
    Rect2d {
      x,
      y,
      width,
      height
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
}

