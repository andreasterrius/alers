use cgmath::Matrix4;

pub mod fly_camera;

pub trait Camera {
  fn calculate_view(&mut self) -> Matrix4<f32>;

  fn calculate_projection(&mut self) -> Matrix4<f32>;
}