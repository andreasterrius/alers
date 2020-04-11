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
use alers::renderer::opengl::{Context, RenderTasks, SimpleRenderTasks, ProjectionTarget};
use alers::renderer::opengl::shader::{ShaderVariable, ShaderVariableType};
use alers::resource::cubemap::Cubemap;
use alers::resource::shader::ShaderFile;
use alers::resource::static_mesh::StaticMesh;
use alers::resource::texture::Texture;
use alers::window::Window;
use log::info;

use crate::alers::data::id::Identifiable;
use alers::data::rect2d::Rect2d;

pub struct Game {
  world: World,
}

impl Game {
  pub fn init_window() -> DisplayInfo {
    DisplayInfo::new(Rect2d::new(800, 600))
  }

  pub fn load(context: &mut Context, window: &Window) -> Game {
    let resource_base_path = "E:\\Codes\\Repos\\alers\\resources";
    let shader_base_path = "E:\\Codes\\Repos\\alers\\shaders";

    // Load meshes
    let meshes = resource::fbx_convert::to_static_meshes(
      resource::fbx::load(&format!("{}/{}", resource_base_path, "test/cube.fbx")).unwrap()).unwrap();
    let cube_mesh = resource::static_mesh::create_cube();

    info!("loaded: {:?}", &meshes[0]);
    info!("cm: {:?}", cube_mesh);

    // Load shaders
    let pbr = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "pbr.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "pbr.frag")).unwrap(),
    );
    let equirect = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "cubemap.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "equirect.frag")).unwrap(),
    );
    let irradiance = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "cubemap.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "irradiance.frag")).unwrap(),
    );
    let skybox = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "skybox.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "skybox.frag")).unwrap(),
    );

    // Load textures
    let texture = resource::texture::Texture::load(
      &format!("{}/{}", resource_base_path, "test/hdr/GravelPlaza_Env.hdr")).unwrap();

    // Load cubemap
    let cubemap = resource::cubemap::Cubemap::new(Rect2d::new(512, 512));
    let convoluted_cubemap = resource::cubemap::Cubemap::new(Rect2d::new(32, 32));

    // Load camera
    let fly_camera = FlyCamera::new(
      Camera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32), 90.0f32, 800f32 / 600f32));

    let mut world = World::new();

    for mesh in meshes {
      world.add_pawn(PawnEntity {
        transform: mesh.0,
        static_mesh_id: mesh.1.uid(),
        shader_id: pbr.uid(),
        textures: vec![],
        shader_variables: vec![
          ShaderVariable::new("albedo".to_owned(), ShaderVariableType::F32_3(Vector3::new(1.0f32, 0.0, 0.0))),
          ShaderVariable::new("metallic".to_owned(), ShaderVariableType::F32_1(0.5f32)),
          ShaderVariable::new("roughness".to_owned(), ShaderVariableType::F32_1(0.5f32)),
          ShaderVariable::new("ao".to_owned(), ShaderVariableType::F32_1(0.5f32)),
        ]
      });
      context.static_mesh(&mesh.1).unwrap();
    }

    world.set_skybox(SkyboxEntity {
      static_mesh_id: cube_mesh.uid(),
      shader_id: skybox.uid(),
      rendered_cubemap_id: convoluted_cubemap.uid(),
      irradiance_cubemap_id: convoluted_cubemap.uid(),
    });

    world.set_camera(CameraEntity::FlyCamera(fly_camera));

    context.cubemap(&cubemap).unwrap();
    context.cubemap(&convoluted_cubemap).unwrap();
    context.static_mesh(&cube_mesh).unwrap();
    context.shader(&pbr).unwrap();
    context.shader(&equirect).unwrap();
    context.shader(&irradiance).unwrap();
    context.shader(&skybox).unwrap();
    context.texture(&texture).unwrap();
    context.setup();

    // Conduct a render pass here for our equirect projection
    let mut render_tasks = SimpleRenderTasks::new();
    render_tasks.queue_cubemap_projection(
      equirect.uid(),
      cube_mesh.uid(),
      ProjectionTarget::Texture2d(texture.uid()),
      cubemap.uid(),
      cubemap.get_dimension().clone(),
      window.get_display_info().get_dimension().clone(),
      vec!()
    );
    render_tasks.render(context);

    // Do a projection again, this time convoluting the cubemap
    let mut render_tasks = SimpleRenderTasks::new();
    render_tasks.queue_cubemap_projection(
      irradiance.uid(),
      cube_mesh.uid(),
      ProjectionTarget::Cubemap(cubemap.uid()),
      convoluted_cubemap.uid(),
      convoluted_cubemap.get_dimension().clone(),
      window.get_display_info().get_dimension().clone(),
      vec!()
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
