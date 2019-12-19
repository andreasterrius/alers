use crate::camera::Camera;
use crate::input::{Input, Key, Action};
use cgmath::{Vector3, Vector2, Deg, Quaternion};
use cgmath::prelude::*;

pub struct FlyCamera {
  camera: Camera,

  rotate_input: Vector2<f32>,
  move_input: Vector3<f32>,

  camera_speed: f32,
  camera_rotate_speed: f32,
}

impl FlyCamera {
  pub fn new(camera: Camera) -> FlyCamera {
    FlyCamera {
      camera,
      rotate_input: Vector2::zero(),
      move_input: Vector3::zero(),
      camera_speed: 10.0,
      camera_rotate_speed: 100.0,
    }
  }

  pub fn camera(&self) -> &Camera {
    &self.camera
  }

  pub fn camera_mut(&mut self) -> &mut Camera{
    &mut self.camera
  }

  pub fn input(&mut self, inputs : &Vec<Input>) {
    for input in inputs {
      self.camera_input(&input);
    }

    self.camera.set_rotation(Quaternion::from_angle_y(-Deg(self.rotate_input.x * self.camera_rotate_speed))
      * Quaternion::from_angle_x(-Deg(self.rotate_input.y * self.camera_rotate_speed)));
  }

  pub fn tick(&mut self, delta_time : f32) {
    self.camera.translate(self.move_input * self.camera_speed * delta_time);
  }

  fn camera_input(&mut self, input: &Input) {
    match input {
      // Handle movement
      Input::Key(Key::A, _, Action::Press, _) => self.move_input.x += -1.0f32,
      Input::Key(Key::D, _, Action::Press, _) => self.move_input.x += 1.0f32,
      Input::Key(Key::W, _, Action::Press, _) => self.move_input.z += 1.0f32,
      Input::Key(Key::S, _, Action::Press, _) => self.move_input.z += -1.0f32,
      Input::Key(Key::A, _, Action::Release, _) => self.move_input.x += 1.0f32,
      Input::Key(Key::D, _, Action::Release, _) => self.move_input.x += -1.0f32,
      Input::Key(Key::W, _, Action::Release, _ ) => self.move_input.z += -1.0f32,
      Input::Key(Key::S, _, Action::Release, _) => self.move_input.z += 1.0f32,

      Input::MouseMotion(x, y) => {
        self.rotate_input.x += *x;
        self.rotate_input.y += *y;
      }
      _ => {}
    }
  }
}