use cgmath::{Matrix4, Vector3, Rotation, Deg, Quaternion};
use math::transform::Transform;
use camera::Camera;

pub struct FlyCamera {
  transform: Transform,
  fov: f32,
  aspect_ratio: f32,

  projection_mat: Option<Matrix4<f32>>,
  view_mat: Option<Matrix4<f32>>
}

impl FlyCamera {
  pub fn new(position: Vector3<f32>, view_dir : Vector3<f32>, fov: f32, aspect_ratio: f32) -> FlyCamera {
    FlyCamera {
      transform : Transform::position_rotation(position, Quaternion::look_at(view_dir, Vector3::unit_y())),
      fov,
      aspect_ratio,
      projection_mat: None,
      view_mat: None
    }
  }

  pub fn calculate_forward_dir(&self) -> Vector3<f32> {
    return self.transform.lcl_rotation.rotate_vector(Vector3::unit_z());
  }

  pub fn calculate_right_dir(&self) -> Vector3<f32> {
    return self.transform.lcl_rotation.rotate_vector(Vector3::unit_x());
  }

  pub fn recalculate_projection_mat(&mut self) {
    match self.projection_mat {
      None => self.projection_mat = Some(cgmath::perspective(Deg(self.fov), self.aspect_ratio, 0.1f32, 100.0f32)),
      Some(_) => ()
    }
  }
}

impl Camera for FlyCamera {
  fn calculate_view(&mut self) -> Matrix4<f32> {
    self.transform.calculate_matrix()
  }

  fn calculate_projection(&mut self) -> Matrix4<f32> {
    self.recalculate_projection_mat();
    self.projection_mat.unwrap()
  }
}
