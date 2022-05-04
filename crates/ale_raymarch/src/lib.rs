use ale_camera::Camera;
use ale_math::num_traits::clamp;
use ale_math::transform::AleTransform;
use ale_math::{clamp_vec3, InnerSpace};
use ale_math::{Vector3, GREEN, RED};
use ale_resources::mesh::sdf::{ale_mesh_sdf_distance, MeshSDF};

pub struct Ray {
  pub origin: Vector3<f32>,
  pub direction: Vector3<f32>,
  t_min: f32,
  t_max: f32,
}

pub fn ale_ray_new(origin: Vector3<f32>, direction: Vector3<f32>) -> Ray {
  Ray {
    origin,
    direction,
    t_min: 0.0,
    t_max: f32::MAX,
  }
}

pub fn ale_ray_position_get(ray: &Ray, t: f32) -> Vector3<f32> {
  let tcl = clamp(t, ray.t_min, ray.t_max);
  ray.origin + tcl * ray.direction
}
pub fn ale_raymarch_sdf_single(
  camera: &Camera,
  sdfs: Vec<(&mut AleTransform, &MeshSDF)>,
) -> Vec<(Vector3<f32>, Vector3<f32>)> {
  let aspect_ratio = camera.aspect_ratio();
  //let (x, y) = camera.viewport_size();
  let (x, y) = (5, 5);
  let position = camera.position();

  let initial =
    camera.position() + camera.forward_dir() - ((camera.right_dir() * aspect_ratio) / 2.0) - (camera.up_dir() / 2.0);
  let right_step = (camera.right_dir() / x as f32 * aspect_ratio);
  let down_step = camera.up_dir() / y as f32;

  let mut color = RED;

  let mut debug = vec![];
  for i in 0..x {
    for j in 0..y {
      let direction = ((initial + right_step * i as f32 + down_step * j as f32) - position).normalize();
      color = if color == RED { GREEN } else { RED };

      let ray = ale_ray_new(position, direction);
      for iter in 0..5 {
        let mut min_dist = f32::MAX;
        let mut curr_pos = ray.origin;
        for (transform, sdf) in &sdfs {
          let mut t = **transform.clone();
          let dist = ale_mesh_sdf_distance(sdf, curr_pos, &mut t);
          min_dist = f32::min(dist, min_dist);
        }
        curr_pos = ale_ray_position_get(&ray, min_dist);
      }
    }
  }

  debug
}
