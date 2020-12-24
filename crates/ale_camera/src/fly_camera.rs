use crate::{Camera, CameraRenderInfo};
use ale_input::{Action, Input, Key};
use ale_math::prelude::*;
use ale_math::{Deg, Quaternion, Vector2, Vector3, Zero};

pub const SMALL_NUMBERF32: f32 = 1e-12;

pub struct FlyCamera {
  camera: Camera,

  delta_rotate_input: Vector2<f32>,
  rotate_input: Vector2<f32>,
  move_input: Vector3<f32>,

  camera_speed: f32,
  camera_rotate_speed: f32,

  disable_input: bool,
}

impl FlyCamera {
  pub fn new(position: Vector3<f32>, viewport_size: Vector2<u32>, fov: f32) -> FlyCamera {
    FlyCamera {
      camera: Camera::new(position, viewport_size, fov),
      delta_rotate_input: Vector2::zero(),
      rotate_input: Vector2::zero(),
      move_input: Vector3::zero(),
      camera_speed: 10.0,
      camera_rotate_speed: 100.0,
      disable_input: false,
    }
  }

  pub fn inputs(&mut self, inputs: &Vec<Input>) {
    self.delta_rotate_input = Vector2::zero();
    for input in inputs {
      self.handle_input(&input);
    }

    if !self.delta_rotate_input.is_zero() {
      self.camera.set_rotation(
        Quaternion::from_angle_y(-Deg(self.rotate_input.x * self.camera_rotate_speed))
          * Quaternion::from_angle_x(-Deg(self.rotate_input.y * self.camera_rotate_speed)),
      );
    }
  }

  pub fn tick(&mut self, delta_time: f64) {
    self
      .camera
      .translate(self.move_input * self.camera_speed * (delta_time as f32));
  }

  pub fn calculate_render_info(&mut self) -> CameraRenderInfo {
    self.calculate_render_info()
  }

  fn handle_input(&mut self, input: &Input) {
    if self.disable_input {
      return;
    }

    match input {
      // Handle movement
      Input::Key(Key::A, _, Action::Press, _) => self.move_input.x += -10.0f32,
      Input::Key(Key::D, _, Action::Press, _) => self.move_input.x += 10.0f32,
      Input::Key(Key::W, _, Action::Press, _) => self.move_input.z += 10.0f32,
      Input::Key(Key::S, _, Action::Press, _) => self.move_input.z += -10.0f32,
      Input::Key(Key::A, _, Action::Release, _) => self.move_input.x += 10.0f32,
      Input::Key(Key::D, _, Action::Release, _) => self.move_input.x += -10.0f32,
      Input::Key(Key::W, _, Action::Release, _) => self.move_input.z += -10.0f32,
      Input::Key(Key::S, _, Action::Release, _) => self.move_input.z += 10.0f32,
      Input::Key(Key::Z, _, Action::Press, _) => self.disable_input = !self.disable_input,

      Input::MouseMotion(x, y) => {
        self.rotate_input.x += *x;
        self.rotate_input.y += *y;
        self.delta_rotate_input.x = *x;
        self.delta_rotate_input.y = *y;
      }
      _ => {}
    }
  }
}
