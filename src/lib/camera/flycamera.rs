use camera::Camera;
use input::{Input, Key, Action};
use cgmath::{Vector3, Vector2};
use cgmath::prelude::*;

pub struct FlyCamera {
  camera: Camera,

  // Uses polar coordinate to keep track of angles
  // r = 1
  theta: f32,
  phi: f32,

  should_rotate : Vector2<f32>,
  should_move: Vector3<f32>,

  camera_speed: f32,
  camera_rotate_speed: f32,
}

impl FlyCamera {
  pub fn new(camera: Camera) -> FlyCamera {
    FlyCamera {
      camera,
      theta: 0.0,
      phi: 0.0,
      should_rotate: Vector2::zero(),
      should_move: Vector3::zero(),
      camera_speed: 10.0,
      camera_rotate_speed: 5000.0,
    }
  }

  pub fn camera(&self) -> &Camera {
    &self.camera
  }

  pub fn camera_mut(&mut self) -> &mut Camera{
    &mut self.camera
  }

  pub fn input(&mut self, inputs : &Vec<Input>) {
    //reset rotation every frame
    self.should_rotate = Vector2::zero();
    for input in inputs {
      self.camera_input(&input);
    }
  }

  pub fn tick(&mut self, delta_time : f32) {
    self.camera.translate(self.should_move * self.camera_speed * delta_time);
    self.camera.yaw_and_pitch(-self.should_rotate * self.camera_rotate_speed * delta_time);
  }

  fn camera_input(&mut self, input: &Input) {
    match input {

      // Handle movement
      Input::Key(Key::A, _, Action::Press, _) => self.should_move.x = -1.0f32,
      Input::Key(Key::D, _, Action::Press, _) => self.should_move.x = 1.0f32,
      Input::Key(Key::W, _, Action::Press, _) => self.should_move.z = 1.0f32,
      Input::Key(Key::S, _, Action::Press, _) => self.should_move.z = -1.0f32,
      Input::Key(Key::A, _, Action::Release, _) => {self.should_move.x = 0.0f32},
      Input::Key(Key::D, _, Action::Release, _) => self.should_move.x = 0.0f32,
      Input::Key(Key::W, _, Action::Release, _ ) => self.should_move.z = 0.0f32,
      Input::Key(Key::S, _, Action::Release, _) => self.should_move.z = 0.0f32,

      Input::MouseMotion(x, y) => {
        self.should_rotate.x += *x;
        self.should_rotate.y -= *y;
      }

      _ => {}
    }
  }
}