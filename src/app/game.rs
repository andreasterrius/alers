use std::fs;

use cgmath::{Vector3, Vector2};

use alers::{camera, resource};
use alers::camera::CameraRenderInfo;
use alers::camera::fly_camera::Camera;
use alers::data::display_info::DisplayInfo;
use alers::data::id::Identifiable;
use alers::input::Input;
use alers::input::Key::{Down, Left, Right, Up};
use alers::input::Action::{Release, Press, Repeat};
use alers::math::transform::Transform;
use alers::renderer::opengl::{Context, RenderTasks};
use alers::resource::ResourceEventObserver;
use alers::resource::shader::ShaderFile;
use alers::resource::static_mesh::StaticMesh;
use cgmath::Zero;

pub struct Game {
  camera: Camera,
  mesh: StaticMesh,
  lambert: ShaderFile,
  transform: Transform,

  // Camera
  camera_input: CameraInput,
  camera_speed: f32,
  camera_rotate_speed : f32,
}

impl Game {
  pub fn load(context: &mut Context) -> Game {
    // Load meshes
    let mut mesh = resource::fbx_convert::to_static_meshes(
      resource::fbx::load("resources/test/triangle.fbx").unwrap()).remove(0);

    // Load shaders
    let mut lambert = resource::shader::ShaderFile::new(
      fs::read_to_string("shaders/lambert.vs").unwrap(),
      fs::read_to_string("shaders/lambert.fs").unwrap()
    );

    context.static_mesh(&mesh);
    context.shader(&lambert);

    let camera = camera::fly_camera::Camera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32),
      Vector3::unit_z(), 90.0f32, 800f32 / 600f32);

    Game {
      camera,
      mesh,
      lambert,
      transform: Transform::new(),
      camera_input: CameraInput { should_move: Vector3::zero(), should_rotate: Vector2::zero() },
      camera_speed: 100.0,
      camera_rotate_speed: 10000.0,
    }
  }

  pub fn tick(&mut self, delta_time: f32) {
    self.camera_transform(delta_time);
  }

  pub fn render<T: RenderTasks>(&mut self, render_tasks: &mut T) {
    render_tasks.queue_static_mesh(&self.lambert, &self.mesh, self.transform.calculate_matrix())
  }

  pub fn input(&mut self, inputs: Vec<Input>) {
    //reset rotation every frame
    self.camera_input.should_rotate = Vector2::zero();
    for input in inputs {
      self.camera_input(&input);
    }
  }

  pub fn camera_render_info(&mut self) -> CameraRenderInfo {
    self.camera.camera_render_info()
  }

  fn camera_input(&mut self, input: &Input) {
    match input {

      // Handle movement
      Input::Key(Left, _, Press, _) => self.camera_input.should_move.x = 1.0f32,
      Input::Key(Right, _, Press, _) => self.camera_input.should_move.x = -1.0f32,
      Input::Key(Up, _, Press, _) => self.camera_input.should_move.z = 1.0f32,
      Input::Key(Down, _, Press, _) => self.camera_input.should_move.z = -1.0f32,
      Input::Key(Left, _, Release, _) => {self.camera_input.should_move.x = 0.0f32},
      Input::Key(Right, _, Release, _) => self.camera_input.should_move.x = 0.0f32,
      Input::Key(Up, _, Release, _ ) => self.camera_input.should_move.z = 0.0f32,
      Input::Key(Down, _, Release, _) => self.camera_input.should_move.z = 0.0f32,

      Input::MouseMotion(x, y) => {
        self.camera_input.should_rotate.x += *x;
        self.camera_input.should_rotate.y += *y;
      }

      _ => {}
    }
  }

  fn camera_transform(&mut self, delta_time : f32) {
    self.camera.translate(self.camera_input.should_move * self.camera_speed * delta_time);
    self.camera.yaw_and_pitch(-self.camera_input.should_rotate * self.camera_rotate_speed * delta_time);
  }
}

pub struct CameraInput {
  pub should_move: Vector3<f32>,
  pub should_rotate : Vector2<f32>,
}