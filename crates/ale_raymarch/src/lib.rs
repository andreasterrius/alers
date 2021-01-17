use ale_camera::Camera;
use ale_math::InnerSpace;
use ale_math::Vector3;
use ale_mesh::sdf::MeshSDF;
use ale_texture::Texture;

pub struct Ray {
  origin: Vector3<f32>,
  direction: Vector3<f32>,
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

pub fn ale_raymarch_sdf_single(camera: &Camera, sdf: &MeshSDF) -> Vec<(Vector3<f32>, Vector3<f32>)> {
  let aspect_ratio = camera.aspect_ratio();
  let (x, y) = camera.viewport_size();
  //let (x, y) = (5, 5);
  let position = camera.position();

  let initial =
    camera.position() + camera.forward_dir() - ((camera.right_dir() * aspect_ratio) / 2.0) - (camera.up_dir() / 2.0);
  let right_step = (camera.right_dir() / x as f32 * aspect_ratio);
  let down_step = camera.up_dir() / y as f32;

  let mut rays = vec![];
  for i in 0..x {
    for j in 0..y {
      let direction = ((initial + right_step * i as f32 + down_step * j as f32) - position).normalize();
      rays.push((position.clone(), (position + direction).clone()));

      let ray = ale_ray_new(position, direction);
    }
  }

  rays
}
