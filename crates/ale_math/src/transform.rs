use cgmath::prelude::*;
use cgmath::{Matrix4, One, Quaternion, Vector3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AleTransform {
  pub position: Vector3<f32>,
  pub lcl_rotation: Quaternion<f32>,
  pub scale: Vector3<f32>,
  matrix: Option<Matrix4<f32>>,
}

impl AleTransform {
  pub fn new() -> AleTransform {
    AleTransform {
      position: Vector3::zero(),
      lcl_rotation: Quaternion::one(),
      scale: Vector3::from_value(1.0f32),
      matrix: None,
    }
  }

  pub fn from_position(position: Vector3<f32>) -> AleTransform {
    AleTransform {
      position,
      scale: Vector3::from_value(1.0f32),
      lcl_rotation: Quaternion::one(),
      matrix: None,
    }
  }

  pub fn from_position_rotation(position: Vector3<f32>, lcl_rotation: Quaternion<f32>) -> AleTransform {
    AleTransform {
      position,
      scale: Vector3::from_value(1.0f32),
      lcl_rotation,
      matrix: None,
    }
  }

  pub fn from_position_scale(position: Vector3<f32>, scale: Vector3<f32>) -> AleTransform {
    AleTransform {
      position,
      scale,
      lcl_rotation: Quaternion::one(),
      matrix: None,
    }
  }

  pub fn from_scale(scale: Vector3<f32>) -> AleTransform {
    AleTransform {
      position: Vector3::zero(),
      scale,
      lcl_rotation: Quaternion::one(),
      matrix: None,
    }
  }

  pub fn from_all(position: Vector3<f32>, lcl_rotation: Quaternion<f32>, scale: Vector3<f32>) -> AleTransform {
    AleTransform {
      position,
      scale,
      lcl_rotation,
      matrix: None,
    }
  }

  pub fn translate(&mut self, unit: Vector3<f32>) {
    self.position += unit;
    self.matrix = None; // Destroy matrix cache
  }

  pub fn set_position(&mut self, unit: Vector3<f32>){
    self.position = unit;
    self.matrix = None;
  }

  pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
    self.lcl_rotation = rotation;
    self.matrix = None; // Destroy matrix cache
  }

  pub fn matrix(&mut self) -> Matrix4<f32> {
    match self.matrix {
      None => {
        let m = Matrix4::from(self.lcl_rotation)
          * Matrix4::from_translation(self.position)
          * Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
        self.matrix = Some(m);
      }
      Some(_) => (),
    }
    self.matrix.unwrap()
  }
}
