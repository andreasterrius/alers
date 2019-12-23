
#[derive(Clone)]
pub struct DisplayInfo {
  pub width: u32,
  pub height: u32,
}

impl DisplayInfo {
  pub fn new(width: u32, height: u32) -> DisplayInfo {
    DisplayInfo {
      width,
      height
    }
  }

  pub fn resize(&mut self, width: u32, height: u32) {
    self.width = width;
    self.height = height;
  }
}