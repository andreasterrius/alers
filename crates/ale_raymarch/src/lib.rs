use ale_camera::Camera;
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

pub fn ale_raymarch_sdf_single(camera: Camera, sdf: &MeshSDF) -> Texture {
  let aspect_ratio = camera.aspect_ratio();
}
