use cgmath::{Vector3, Matrix, Matrix4, Quaternion, One, Deg, Rad};
use cgmath::prelude::*;
use rusttype::vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
  pub position: Vector3<f32>,
  pub lcl_rotation : Quaternion<f32>,
  pub scale : Vector3<f32>,
  matrix : Option<Matrix4<f32>>,
}

impl Transform {

  pub fn new() -> Transform {
    Transform {
      position: Vector3::zero(),
      lcl_rotation: Quaternion::one(),
      scale: Vector3::from_value(1.0f32),
      matrix: None
    }
  }

  pub fn from_position(position: Vector3<f32>) -> Transform {
    Transform {
      position,
      scale : Vector3::from_value(1.0f32),
      lcl_rotation: Quaternion::one(),
      matrix: None,
    }
  }

  pub fn from_position_rotation(position : Vector3<f32>, lcl_rotation: Quaternion<f32>) -> Transform {
    Transform {
      position,
      scale : Vector3::from_value(1.0f32),
      lcl_rotation,
      matrix: None,
    }
  }

  pub fn recalculate(&mut self) {
    match self.matrix {
      None => {
        let mut m = Matrix4::from_translation(self.position);
        self.matrix = Some(m);
      },
      Some(_) => (),
    }
  }

  pub fn translate(&mut self, unit : Vector3<f32>) {
    self.position += unit;
    self.matrix = None; // Destroy matrix cache
  }

  pub fn rotate_by_axis(&mut self, theta_by_axis : Vector3<f32>){
    self.lcl_rotation = self.lcl_rotation * Quaternion::from_angle_y(Deg(theta_by_axis.x)); // yaw ?
//    self.lcl_rotation = self.lcl_rotation * Quaternion::from_angle_y(Deg(theta_by_axis.y));
//    self.lcl_rotation = self.lcl_rotation * Quaternion::from_angle_z(Deg(theta_by_axis.z));
    self.matrix = None; // Destroy matrix cache
  }

  pub fn calculate_matrix(&mut self) -> Matrix4<f32> {
    self.recalculate();
    self.matrix.unwrap()
  }
}
