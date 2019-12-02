use std::fs;

use cgmath::Vector3;

use alers::{camera, resource};
use alers::camera::fly_camera::FlyCamera;
use alers::data::display_info::DisplayInfo;
use alers::data::id::Identifiable;
use alers::math::transform::Transform;
use alers::renderer::opengl::{Context, RenderTasks};
use alers::resource::ResourceEventObserver;
use alers::resource::shader::ShaderFile;
use alers::resource::static_mesh::StaticMesh;
use alers::input::Input;
use alers::camera::CameraRenderInfo;

pub struct Game {
  camera: FlyCamera,
  mesh: StaticMesh,
  lambert: ShaderFile,
  transform : Transform,
}

impl Game {

  pub fn load(context : &mut Context) -> Game {
    // Load meshes
    let mut mesh = resource::fbx_convert::to_static_meshes(
      resource::fbx::load("resources/test/geom/triangle.fbx").unwrap()).remove(0);

    // Load shaders
    let mut lambert = resource::shader::ShaderFile::new(
      fs::read_to_string("shaders/lambert.vs").unwrap(),
      fs::read_to_string("shaders/lambert.fs").unwrap()
    );

    context.static_mesh(&mesh);
    context.shader(&lambert);

    let camera = camera::fly_camera::FlyCamera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32),
      Vector3::unit_z(), 90.0f32, 800f32 / 600f32);

    Game {
      camera,
      mesh,
      lambert,
      transform: Transform::new()
    }
  }

  pub fn tick(&mut self) {}

  pub fn render<T: RenderTasks>(&mut self, render_tasks: &mut T) {
    render_tasks.queue_static_mesh(&self.lambert, &self.mesh, self.transform.calculate_matrix())
  }

  pub fn input(&mut self, input: Vec<Input>){

  }

  pub fn camera_render_info(&mut self) -> CameraRenderInfo {
    self.camera.camera_render_info()
  }
}