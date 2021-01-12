pub mod color;
pub mod rect;
pub mod transform;
pub use cgmath::*;

pub fn ale_quaternion_look_at(
  source: Vector3<f32>,
  dest: Vector3<f32>,
  front: Vector3<f32>,
  up: Vector3<f32>,
) -> Quaternion<f32> {
  let to_vector = (dest - source).normalize();

  //compute rotation axis
  let mut rot_axis = front.cross(to_vector).normalize();
  if rot_axis.magnitude2() == 0.0 {
    rot_axis = up;
  }

  //find the angle around rotation axis
  let dot = front.dot(to_vector);
  let ang = f32::acos(dot);

  //convert axis angle to quaternion
  return Quaternion::from_axis_angle(rot_axis, Deg(ang));
}

pub fn ale_bounding_box_size(bounding_box: (Vector3<f32>, Vector3<f32>)) -> Vector3<f32> {
  let (min, max) = bounding_box;
  max - min
}
