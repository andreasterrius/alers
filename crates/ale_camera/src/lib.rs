use ale_math::prelude::*;
use ale_math::rect::Rect;
use ale_math::transform::Transform;
use ale_math::{ortho, perspective, Deg, Matrix4, Quaternion, Vector2, Vector3};
use std::sync::Arc;

pub mod fly_camera;

pub struct Camera {
  transform: Transform,
  fov: f32,
  aspect_ratio: f32,
  viewport_size: Vector2<u32>,

  projection_mat: Option<Matrix4<f32>>,
  orthographic_mat: Option<Matrix4<f32>>,
  view_mat: Option<Matrix4<f32>>,
}

impl Camera {
  pub fn new(position: Vector3<f32>, viewport_size: Vector2<u32>, fov: f32) -> Camera {
    let aspect_ratio = viewport_size.x as f32 / viewport_size.y as f32;

    Camera {
      transform: Transform::from_position_rotation(position, Quaternion::one()),
      fov,
      aspect_ratio,
      viewport_size,
      projection_mat: None,
      orthographic_mat: None,
      view_mat: None,
    }
  }

  pub fn translate(&mut self, translation: Vector3<f32>) {
    if translation.is_zero() {
      return;
    }

    let forward_dir = self.get_forward_dir();
    let right_dir = self.get_right_dir();
    self
      .transform
      .translate(forward_dir * translation.z + right_dir * translation.x);
    self.view_mat = None;
  }

  pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
    if rotation.is_zero() {
      return;
    }

    self.transform.set_rotation(rotation);
    self.view_mat = None;
  }

  pub fn get_forward_dir(&mut self) -> Vector3<f32> {
    self.transform.lcl_rotation.rotate_vector(Vector3::unit_z()).normalize()
  }

  pub fn get_right_dir(&mut self) -> Vector3<f32> {
    return self.get_forward_dir().cross(Vector3::unit_y()).normalize();
  }

  fn recalculate_matrices(&mut self) {
    self.orthographic_mat = None;
    self.projection_mat = None;
    self.view_mat = None;
  }

  fn calculate_view_mat(&mut self) -> Matrix4<f32> {
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

  fn calculate_projection_mat(&mut self) -> Matrix4<f32> {
    match self.projection_mat {
      None => self.projection_mat = Some(perspective(Deg(self.fov), self.aspect_ratio, 0.1f32, 100.0f32)),
      Some(_) => (),
    }
    self.projection_mat.unwrap()
  }

  fn calculate_orthographic_mat(&mut self) -> Matrix4<f32> {
    match self.orthographic_mat {
      None => {
        self.orthographic_mat = Some(ortho(
          0.0f32,
          self.viewport_size.x as f32,
          self.viewport_size.y as f32,
          0.0,
          -1.0,
          1.0f32,
        ))
      }
      Some(_) => (),
    }
    self.orthographic_mat.unwrap()
  }

  pub fn calculate_render_info(&mut self) -> CameraRenderInfo {
    CameraRenderInfo {
      view: self.calculate_view_mat(),
      projection: self.calculate_projection_mat(),
      orthographic: self.calculate_orthographic_mat(),
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
