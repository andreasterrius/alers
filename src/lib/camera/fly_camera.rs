use cgmath::{Deg, Matrix4, One, Point3, Quaternion, Rotation, Rotation3, Vector2, Vector3};
use cgmath::prelude::*;

use camera::CameraRenderInfo;
use math::transform::Transform;

pub struct Camera {
  transform: Transform,
  fov: f32,
  aspect_ratio: f32,
  view_dir: Vector3<f32>,

  projection_mat: Option<Matrix4<f32>>,
  view_mat: Option<Matrix4<f32>>
}

impl Camera {
  pub fn new(position: Vector3<f32>, view_dir: Vector3<f32>, fov: f32, aspect_ratio: f32) -> Camera {
    Camera {
      transform: Transform::from_position_rotation(position, Quaternion::one()),
      fov,
      view_dir,
      aspect_ratio,
      projection_mat: None,
      view_mat: None
    }
  }

  pub fn translate(&mut self, translation: Vector3<f32>) {
    let forward_dir = self.calculate_forward_dir();
    let right_dir = self.calculate_right_dir();
    self.transform.translate(forward_dir * translation.z + right_dir * translation.x);
    self.view_mat = None;
  }

  pub fn yaw_and_pitch(&mut self, theta_by_axis: Vector2<f32>) {
    self.transform.rotate_by_axis(Vector3::new(theta_by_axis.x, theta_by_axis.y, 0.0f32));
    self.view_mat = None;
  }

  pub fn calculate_forward_dir(&self) -> Vector3<f32> {
    return self.transform.lcl_rotation.rotate_vector(self.view_dir).normalize();
  }

  pub fn calculate_right_dir(&self) -> Vector3<f32> {
    return self.calculate_forward_dir().cross(Vector3::unit_y()).normalize();
  }

  fn calculate_view(&mut self) -> Matrix4<f32> {
    self.recalculate_view_mat();
    self.view_mat.unwrap()
  }

  fn calculate_projection(&mut self) -> Matrix4<f32> {
    self.recalculate_projection_mat();
    self.projection_mat.unwrap()
  }

  pub fn recalculate_projection_mat(&mut self) {
    match self.projection_mat {
      None => self.projection_mat = Some(cgmath::perspective(Deg(self.fov), self.aspect_ratio, 0.1f32, 100.0f32)),
      Some(_) => ()
    }
  }

  pub fn recalculate_view_mat(&mut self) {
    match self.view_mat {
      None => {
        self.view_mat = Some(
          Matrix4::from_translation(self.transform.position) *
            Matrix4::look_at(Point3::from_vec(self.transform.position),
              Point3::from_vec(self.transform.position + self.calculate_forward_dir()), Vector3::unit_y()));
      },
      Some(_) => ()
    }
  }

  pub fn camera_render_info(&mut self) -> CameraRenderInfo {
    CameraRenderInfo {
      view: self.calculate_view(),
      projection: self.calculate_projection()
    }
  }
}

