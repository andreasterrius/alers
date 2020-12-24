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

pub fn ale_camera_new(position: Vector3<f32>, viewport_size: Vector2<u32>, fov: f32) -> Camera {
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

pub fn ale_camera_translate(camera: &mut Camera, translation: Vector3<f32>) {
  if translation.is_zero() {
    return;
  }

  let forward_dir = ale_camera_forward_dir_get(camera);
  let right_dir = ale_camera_right_dir_get(camera);
  camera
    .transform
    .translate(forward_dir * translation.z + right_dir * translation.x);
  camera.view_mat = None;
}

pub fn ale_camera_set_rotation(camera: &mut Camera, rotation: Quaternion<f32>) {
  if rotation.is_zero() {
    return;
  }

  camera.transform.set_rotation(rotation);
  camera.view_mat = None;
}

pub fn ale_camera_forward_dir_get(camera: &Camera) -> Vector3<f32> {
  camera
    .transform
    .lcl_rotation
    .rotate_vector(Vector3::unit_z())
    .normalize()
}

pub fn ale_camera_right_dir_get(camera: &Camera) -> Vector3<f32> {
  return ale_camera_forward_dir_get(camera).cross(Vector3::unit_y()).normalize();
}

fn ale_camera_recalculate_matrices(camera: &mut Camera) {
  camera.orthographic_mat = None;
  camera.projection_mat = None;
  camera.view_mat = None;
}

fn ale_camera_view_mat_calculate(camera: &mut Camera) -> Matrix4<f32> {
  match camera.view_mat {
    None => {
      camera.view_mat = Some(
        Matrix4::from(camera.transform.lcl_rotation.invert()) * Matrix4::from_translation(camera.transform.position),
      );
    }
    Some(_) => (),
  }

  camera.view_mat.unwrap()
}

fn ale_camera_projection_mat_calculate(camera: &mut Camera) -> Matrix4<f32> {
  match camera.projection_mat {
    None => camera.projection_mat = Some(perspective(Deg(camera.fov), camera.aspect_ratio, 0.1f32, 100.0f32)),
    Some(_) => (),
  }
  camera.projection_mat.unwrap()
}

fn ale_camera_orthographic_mat_calculate(camera: &mut Camera) -> Matrix4<f32> {
  match camera.orthographic_mat {
    None => {
      camera.orthographic_mat = Some(ortho(
        0.0f32,
        camera.viewport_size.x as f32,
        camera.viewport_size.y as f32,
        0.0,
        -1.0,
        1.0f32,
      ))
    }
    Some(_) => (),
  }
  camera.orthographic_mat.unwrap()
}

pub fn ale_camera_render_info_calculate(camera: &mut Camera) -> CameraRenderInfo {
  CameraRenderInfo {
    view: ale_camera_view_mat_calculate(camera),
    projection: ale_camera_projection_mat_calculate(camera),
    orthographic: ale_camera_orthographic_mat_calculate(camera),
    position: camera.transform.position,
  }
}

#[derive(Debug, Clone)]
pub struct CameraRenderInfo {
  pub view: Matrix4<f32>,
  pub projection: Matrix4<f32>,
  pub orthographic: Matrix4<f32>,
  pub position: Vector3<f32>,
}
