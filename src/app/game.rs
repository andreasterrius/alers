use std::fs;

use cgmath::Vector3;

use alers::{camera, resource};
use alers::camera::CameraRenderInfo;
use alers::camera::flycamera::FlyCamera;
use alers::input::Input;
use alers::math::transform::Transform;
use alers::renderer::opengl::{Context, RenderTasks, ShaderVariable, ShaderVariableType};
use alers::resource::shader::ShaderFile;
use alers::resource::static_mesh::StaticMesh;
use alers::resource::texture::Texture;

pub struct Game {
  fly_camera: FlyCamera,
  mesh: StaticMesh,
  lambert: ShaderFile,
  texture: Texture,
  transform: Transform,
}

impl Game {
  pub fn load(context: &mut Context) -> Game {

    let base_path = "/home/alether/Codes/Graphics/alers";

    // Load meshes
    //let mesh = resource::fbx_convert::to_static_meshes(
    //  resource::fbx::load(&format!("{}/{}", base_path, "resources/test/cube.fbx")).unwrap()).unwrap().remove(0);
    let mesh = resource::static_mesh::create_cube();

    // Load shaders
    let lambert = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", base_path, "shaders/lambert.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", base_path, "shaders/lambert.frag")).unwrap()
    );

    // Load textures
    let texture = resource::texture::Texture::load(
      &format!("{}/{}", base_path, "resources/test/container.jpg")).unwrap();

    context.static_mesh(&mesh).unwrap();
    context.shader(&lambert).unwrap();
    context.texture(&texture).unwrap();
    context.setup();

    let camera = camera::Camera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32), 90.0f32, 800f32 / 600f32);
    let fly_camera = camera::flycamera::FlyCamera::new(camera);

    Game {
      fly_camera,
      mesh,
      texture,
      lambert,
      transform: Transform::new(),
    }
  }

  pub fn input(&mut self, inputs: Vec<Input>) {
    self.fly_camera.input(&inputs);
  }

  pub fn tick(&mut self, delta_time: f32) {
    self.fly_camera.tick(delta_time);
  }

  pub fn render<T: RenderTasks>(&mut self, render_tasks: &mut T) {
    // Let there be light
    let light_position = ShaderVariable::new("light_position".to_owned(), ShaderVariableType::F32_3(Vector3::new(5.0, 5.0, 5.0)));
    let light_color = ShaderVariable::new("light_color".to_owned(), ShaderVariableType::F32_3(Vector3::new(0.0, 0.0, 1.0)));

    render_tasks.queue_static_mesh(&self.lambert, &self.mesh, vec!(&self.texture),
      self.transform.matrix(), vec![light_position, light_color]);
  }

  pub fn camera_render_info(&mut self) -> CameraRenderInfo {
    self.fly_camera.camera_mut().camera_render_info()
  }
}
