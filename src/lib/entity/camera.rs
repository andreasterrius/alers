use crate::camera::flycamera::FlyCamera;
use crate::camera::{Camera, CameraRenderInfo};
use crate::input::Input;

pub enum CameraEntity {
  FlyCamera(FlyCamera),
  Camera(Camera),
  None,
}

impl CameraEntity {
  pub fn input(&mut self, inputs: &Vec<Input>) {
    match self {
      CameraEntity::FlyCamera(c) => c.input(inputs),
      CameraEntity::Camera(_) => { /* No Input */ }
      CameraEntity::None => { /* No Input */ }
    }
  }

  pub fn tick(&mut self, delta_time: f32) {
    match self {
      CameraEntity::FlyCamera(c) => c.tick(delta_time),
      CameraEntity::Camera(_) => {}
      CameraEntity::None => {}
    }
  }

  pub fn get_camera_render_info(&mut self) -> Option<CameraRenderInfo> {
    match self {
      CameraEntity::FlyCamera(c) => Some(c.camera_mut().camera_render_info()),
      CameraEntity::Camera(c) => Some(c.camera_render_info()),
      CameraEntity::None => None,
    }
  }
}
