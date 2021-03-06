#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Color {
  r: f32,
  g: f32,
  b: f32,
  a: f32,
}

impl Color {
  pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b, a: 1.0 }
  }

  pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
    Color { r, g, b, a }
  }

  pub fn get_rgb(&self) -> (f32, f32, f32) {
    (self.r, self.g, self.b)
  }

  pub fn get_rgba(&self) -> (f32, f32, f32, f32) {
    (self.r, self.g, self.b, self.a)
  }
}
