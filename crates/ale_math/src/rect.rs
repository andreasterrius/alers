use cgmath::Vector2;

#[derive(Debug, Clone)]
pub struct Rect {
  pub position: Vector2<i32>,
  pub size: Vector2<u32>,
}

impl Rect {
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
