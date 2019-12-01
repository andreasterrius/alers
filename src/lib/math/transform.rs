use cgmath::{Vector3, Matrix, Matrix4, Quaternion, One};
use cgmath::prelude::*;
use rusttype::vector;

#[derive(Debug, Clone)]
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

  pub fn position(position: Vector3<f32>) -> Transform {
    Transform {
      position,
      scale : Vector3::from_value(1.0f32),
      lcl_rotation: Quaternion::one(),
      matrix: None,
    }
  }

  pub fn position_rotation(position : Vector3<f32>, lcl_rotation: Quaternion<f32>) -> Transform {
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
        self.matrix = Some(m) ;
      },
      Some(_) => (),
    }
  }

  pub fn calculate_matrix(&mut self) -> Matrix4<f32> {
    self.recalculate();
    self.matrix.unwrap()
  }
}
