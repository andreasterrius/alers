use std::fs;

use cgmath::Vector3;

use alers::{camera, resource};
use alers::camera::{Camera, CameraRenderInfo};
use alers::camera::flycamera::FlyCamera;
use alers::data::display_info::DisplayInfo;
use alers::entity::camera::CameraEntity;
use alers::entity::pawn::PawnEntity;
use alers::entity::skybox::SkyboxEntity;
use alers::entity::world::World;
use alers::input::Input;
use alers::math::transform::Transform;
use alers::renderer::opengl::{Context, RenderTasks, SimpleRenderTasks};
use alers::renderer::opengl::shader::{ShaderVariable, ShaderVariableType};
use alers::resource::cubemap::Cubemap;
use alers::resource::shader::ShaderFile;
use alers::resource::static_mesh::StaticMesh;
use alers::resource::texture::Texture;
use alers::window::Window;

use crate::alers::data::id::Identifiable;

pub struct Game {
  world: World,
}

impl Game {
  pub fn init_window() -> DisplayInfo {
    DisplayInfo::new(800, 600)
  }

  pub fn load(context: &mut Context, window: &Window) -> Game {
    let base_path = "/home/alether/Codes/Graphics/alers";

    // Load meshes
    //let mesh = resource::fbx_convert::to_static_meshes(
    //  resource::fbx::load(&format!("{}/{}", base_path, "resources/test/cube.fbx")).unwrap()).unwrap().remove(0);
    let mesh = resource::static_mesh::create_cube();

    // Load shaders
    let lambert = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", base_path, "shaders/lambert.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", base_path, "shaders/lambert.frag")).unwrap(),
    );    // Load shaders
    let equirect = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", base_path, "shaders/equirect.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", base_path, "shaders/equirect.frag")).unwrap(),
    );
    let skybox = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", base_path, "shaders/skybox.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", base_path, "shaders/skybox.frag")).unwrap(),
    );

    // Load textures
    let texture = resource::texture::Texture::load(
      &format!("{}/{}", base_path, "resources/hdr/Newport_Loft_Ref.hdr")).unwrap();

    let cubemap = resource::cubemap::Cubemap::new();

    let fly_camera = FlyCamera::new(
      Camera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32), 90.0f32, 800f32 / 600f32));

    let mut world = World::new();

    world.add_pawn(PawnEntity {
      transform: Transform::new(),
      static_mesh_id: mesh.uid(),
      shader_id: lambert.uid(),
      textures: vec![texture.uid()],
    });

    world.set_skybox(SkyboxEntity {
      static_mesh_id: mesh.uid(),
      shader_id: skybox.uid(),
      cubemap_id: cubemap.uid(),
    });

    world.set_camera(CameraEntity::FlyCamera(fly_camera));

    context.cubemap(&cubemap).unwrap();
    context.static_mesh(&mesh).unwrap();
    context.shader(&lambert).unwrap();
    context.shader(&equirect).unwrap();
    context.shader(&skybox).unwrap();
    context.texture(&texture).unwrap();
    context.setup();

    // Conduct a render pass here for our equirect projection
    let mut render_tasks = SimpleRenderTasks::new();
    render_tasks.queue_cubemap_projection(
      equirect.uid(),
      mesh.uid(),
      texture.uid(),
      cubemap.uid(),
      window.get_display_info(),
      vec!(),
    );
    render_tasks.render(context);

    Game {
      world,
    }
  }

  pub fn input(&mut self, inputs: Vec<Input>) {
    self.world.input(&inputs);
  }

  pub fn tick(&mut self, delta_time: f32) {
    self.world.tick(delta_time);
  }

  pub fn render<T: RenderTasks>(&mut self, render_tasks: &mut T) {
    self.world.render::<T>(render_tasks);
  }
}
