#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32,
}

impl Color {
  pub fn red() -> Color {
    return Color::from_rgb(1.0, 0.0, 0.0);
  }

  pub fn green() -> Color {
    return Color::from_rgb(0.0, 1.0, 0.0);
  }

  pub fn yellow() -> Color {
    return Color::from_rgb(1.0, 1.0, 0.0);
  }

  pub fn blue() -> Color {
    return Color::from_rgb(0.0, 0.0, 1.0);
  }

  pub fn white() -> Color {
    return Color::from_rgb(1.0, 1.0, 1.0);
  }

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
