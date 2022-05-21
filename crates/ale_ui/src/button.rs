use ale_input::{Action, Input, MouseButton};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::{Matrix4, Vector2};
use ale_opengl::renderer::sprite::SpriteRenderer;

use crate::element::RenderResources;
use crate::layout::Layout;

pub struct Button {
  pub (crate) layout: Layout,
  idle_color: Color,
  hover_color: Color,
  press_color: Color,

  is_pressed: bool,
  is_hover: bool,
  is_disable: bool,
}

impl Button {
  pub fn new(
    position: Vector2<i32>,
    size: Vector2<u32>,
    idle_color: Color,
    enter_color: Color,
    click_color: Color,
  ) -> Button {
    return Button {
      layout: Layout::new_local(position, size),
      idle_color,
      hover_color: enter_color,
      press_color: click_color,
      is_pressed: false,
      is_hover: false,
      is_disable: false,
    };
  }

  pub fn new_basic(color : Color) -> Button {
    return Button {
      layout: Layout::new(),
      idle_color: color,
      hover_color: color,
      press_color: color,
      is_pressed: false,
      is_hover: false,
      is_disable: false
    }
  }

  pub fn before_tick(&mut self) {}

  pub fn input(&mut self, input: &Input) {
    match input {
      Input::MouseMotion {
        rel_x,
        rel_y,
        abs_x,
        abs_y,
      } => {
        let x = *abs_x as i32;
        let y = *abs_y as i32;
        if self.layout.is_inside(x, y) {
          self.is_hover = true;
        } else {
          self.is_hover = false;
        }
      }
      Input::MouseButton(mbtn, action, modifier) => {
        if self.is_hover && mbtn == &MouseButton::ButtonLeft && action == &Action::Press {
          self.is_pressed = true;
        } else {
          self.is_pressed = false;
        }
      }
      _ => {}
    }
  }

  pub fn render_with(&self, rr: &mut RenderResources) {
    let mut button_color = self.idle_color;
    if self.is_hover {
      button_color = self.hover_color;
    }
    if self.is_pressed {
      button_color = self.press_color;
    }

    rr.sprite_renderer.render_flat_box(
      Vector2::new(self.layout.global_position.x as f32, self.layout.global_position.y as f32),
      Vector2::new(self.layout.global_size.x as f32, self.layout.global_size.y as f32),
      button_color,
      rr.camera_render_info.orthographic,
    )
  }
}
