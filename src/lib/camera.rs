use cgmath::prelude::*;
use cgmath::{Deg, Matrix4, Quaternion, Rotation, Vector3};

use crate::math::transform::Transform;

pub mod flycamera;

pub struct Camera {
  transform: Transform,
  fov: f32,
  aspect_ratio: f32,

  projection_mat: Option<Matrix4<f32>>,
  orthographic_mat: Option<Matrix4<f32>>,
  view_mat: Option<Matrix4<f32>>,
}

impl Camera {
  pub fn new(position: Vector3<f32>, fov: f32, aspect_ratio: f32) -> Camera {
    Camera {
      transform: Transform::from_position_rotation(position, Quaternion::one()),
      fov,
      aspect_ratio,
      projection_mat: None,
      orthographic_mat: None,
      view_mat: None,
    }
  }

  pub fn translate(&mut self, translation: Vector3<f32>) {
    let forward_dir = self.forward_dir();
    let right_dir = self.right_dir();
    self
      .transform
      .translate(forward_dir * translation.z + right_dir * translation.x);
  }

  pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
    self.transform.set_rotation(rotation);
    self.view_mat = None;
  }

  pub fn forward_dir(&self) -> Vector3<f32> {
    self.transform.lcl_rotation.rotate_vector(Vector3::unit_z()).normalize()
  }

  pub fn right_dir(&self) -> Vector3<f32> {
    return self.forward_dir().cross(Vector3::unit_y()).normalize();
  }

  fn view_mat(&mut self) -> Matrix4<f32> {
    match self.view_mat {
      None => {
        self.view_mat = Some(
          Matrix4::from(self.transform.lcl_rotation.invert()) * Matrix4::from_translation(self.transform.position),
        );
      }
      Some(_) => (),
    }

    self.view_mat.unwrap()
  }

  fn projection_mat(&mut self) -> Matrix4<f32> {
    match self.projection_mat {
      None => self.projection_mat = Some(cgmath::perspective(Deg(self.fov), self.aspect_ratio, 0.1f32, 100.0f32)),
      Some(_) => (),
    }
    self.projection_mat.unwrap()
  }

  fn orthographic_mat(&mut self) -> Matrix4<f32> {
    match self.orthographic_mat {
      None => self.orthographic_mat = Some(cgmath::ortho(0.0f32, 800.0, 600.0, 0.0, -1.0, 1.0f32)),
      Some(_) => (),
    }
    self.orthographic_mat.unwrap()
  }

  pub fn camera_render_info(&mut self) -> CameraRenderInfo {
    CameraRenderInfo {
      view: self.view_mat(),
      projection: self.projection_mat(),
      orthographic: self.orthographic_mat(),
      position: self.transform.position,
    }
  }
}

#[derive(Debug, Clone)]
pub struct CameraRenderInfo {
  pub view: Matrix4<f32>,
  pub projection: Matrix4<f32>,
  pub orthographic: Matrix4<f32>,
  pub position: Vector3<f32>,
}
