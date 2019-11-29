use cgmath::{Vector3, Matrix, Matrix4, Quaternion, One};
use cgmath::prelude::*;
use rusttype::vector;

pub struct Transform {
  position: Vector3<f32>,
  lcl_rotation : Quaternion<f32>,
  scale : Vector3<f32>,
  matrix : Option<Matrix4<f32>>,
}

impl Transform {
  pub fn position(position: Vector3<f32>) -> Transform {
    Transform {
      position,
      scale : Vector3::from_value(1.0f32),
      lcl_rotation: Quaternion::one(),
      matrix: None,
    }
  }

  pub fn recalculate(&mut self) {
    match self.matrix {
      None => {
        let mut m = Matrix4::from_translation(self.position);
        self.matrix = Some(m) ;
      },
      Some(_) => { /* Do Nothing*/ },
    }
  }

  pub fn get_matrix(&mut self) -> Matrix4<f32> {
    self.recalculate();
    self.matrix.unwrap()
  }
}
