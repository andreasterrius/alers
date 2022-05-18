use ale_input::Input;
use ale_math::color::Color;
use ale_math::{Matrix4, Vector2};
use ale_opengl::renderer::sprite::SpriteRenderer;
use crate::element::RenderResources;

pub struct Button {
  position: Vector2<f32>,
  size: Vector2<f32>,

  idle_color: Color,
  enter_color: Color,
  click_color: Color,

  is_disable: bool,
}

impl Button {
  pub fn new(
    position : Vector2<f32>,
    size: Vector2<f32>,
    idle_color: Color,
    enter_color: Color,
    click_color: Color) -> Button {
    return Button {
      position,
      size,
      idle_color,
      enter_color,
      click_color,
      is_disable: false,
    };
  }

  pub fn input(&self, input: &Input) {
    match input {
      Input::MouseMotion{rel_x, rel_y, abs_x, abs_y} => {
        println!("{} {}", abs_x, abs_y);
      }
      _ => {}
    }
  }

  pub fn render_with(&self, rr : &mut RenderResources) {
    rr.sprite_renderer.render_flat_box_border(
      self.position,
      self.size,
      self.idle_color,
      2.0f32,
      self.enter_color,
      rr.camera_render_info.orthographic,
    )
  }
}
