use cgmath::Matrix4;

pub mod camera;

pub struct CameraRenderInfo {
  pub view : Matrix4<f32>,
  pub projection : Matrix4<f32>,
}
