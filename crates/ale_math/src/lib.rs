pub mod color;
pub mod rect;
pub mod transform;
use cgmath::num_traits::clamp;
pub use cgmath::*;

pub const RED: Vector3<f32> = Vector3::new(1.0, 0.0, 0.0);
pub const GREEN: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
pub const BLUE: Vector3<f32> = Vector3::new(0.0, 0.0, 1.0);

pub fn clamp_vec3(p: Vector3<f32>, min: Vector3<f32>, max: Vector3<f32>) -> Vector3<f32> {
  Vector3::new(
    clamp(p.x, min.x, max.x),
    clamp(p.y, min.y, max.y),
    clamp(p.z, min.z, max.z),
  )
}

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

// return original point if the point is already inside the box
pub fn ale_bounding_box_closest_point(point: Vector3<f32>, bounding_box: (Vector3<f32>, Vector3<f32>)) -> Vector3<f32> {
  let (min, max) = bounding_box;

  return if point.x >= min.x
    && point.x <= max.x
    && point.y >= min.y
    && point.y <= max.y
    && point.z >= min.z
    && point.z <= max.z
  {
    point
  } else {
    clamp_vec3(point, min, max)
  };
}
