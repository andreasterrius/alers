use cgmath::Matrix4;

pub mod fly_camera;

pub struct CameraRenderInfo {
  pub view : Matrix4<f32>,
  pub projection : Matrix4<f32>,
}
