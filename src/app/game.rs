use std::fs;

use ale_font::{ale_font_load, Font};
use ale_gltf::ale_gltf_load;
use ale_mesh::{ale_mesh_new_cube, ale_mesh_new_plane};
use ale_opengl::ale_opengl_text_render;
use alers::camera::flycamera::FlyCamera;
use alers::camera::Camera;
use alers::data::color::Color;
use alers::data::display_info::DisplayInfo;
use alers::entity::camera::CameraEntity;
use alers::entity::pawn::PawnEntity;
use alers::entity::skybox::SkyboxEntity;
use alers::entity::ui::UIEntity;
use alers::entity::world::World;
use alers::input::Input;
use alers::math::rect::Rect;
use alers::renderer::opengl::shader::{ShaderVariable, ShaderVariableType};
use alers::renderer::opengl::{ProjectionTarget, RenderContext, RenderTasks, SimpleRenderTasks};
use alers::resource;
use alers::ui::panel::Panel;
use alers::ui::UI;
use alers::window::Window;
use cgmath::{Vector2, Vector3};
use log::info;

pub struct Game {
  world: World,

  inconsolata_font: Font,
}

impl Game {
  pub fn init_window() -> DisplayInfo {
    DisplayInfo::new(Rect::new(800, 600))
  }

  pub fn load(context: &mut RenderContext, window: &Window) -> Game {
    let resource_base_path = "E:\\Codes\\Repos\\alers\\resources";
    let shader_base_path = "E:\\Codes\\Repos\\alers\\shaders";

    // Load meshes
    // let meshes = resource::fbx_convert::to_static_meshes(
    //   resource::fbx::load(&format!("{}/{}", resource_base_path, "test/scene.fbx")).unwrap(),
    // )
    // .unwrap();

    let meshes = ale_gltf_load(&format!("{}/{}", resource_base_path, "test/sceneg.gltf"));

    let cube_mesh = ale_mesh_new_cube();
    let plane_mesh = ale_mesh_new_plane();

    // Load skeletal meshes
    // let _skeletal_meshes = resource::fbx_convert::to_skeletal_meshes(
    //   resource::fbx::load(&format!("{}/{}", resource_base_path, "test/anim_begin.fbx")).unwrap(),
    // )
    // .unwrap();

    // Load shaders
    let pbr_shader = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "pbr.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "pbr.frag")).unwrap(),
    );
    let equirect_shader = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "cubemap.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "equirect.frag")).unwrap(),
    );
    let irradiance_shader = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "cubemap.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "irradiance.frag")).unwrap(),
    );
    let skybox_shader = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "skybox.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "skybox.frag")).unwrap(),
    );
    let ui_shader = resource::shader::ShaderFile::new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "ui.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "ui.frag")).unwrap(),
    );

    // Load textures
    let texture =
      resource::texture::Texture::load(&format!("{}/{}", resource_base_path, "test/hdr/GravelPlaza_Env.hdr")).unwrap();

    // Load cubemap
    let cubemap = resource::cubemap::Cubemap::new(Rect::new(512, 512));
    let convoluted_cubemap = resource::cubemap::Cubemap::new(Rect::new(32, 32));

    // Load fonts
    let inconsolata_font = ale_font_load("resources/font/Inconsolata/static/Inconsolata-Regular.ttf");

    // Load camera
    let fly_camera = FlyCamera::new(Camera::new(
      Vector3::new(0.0f32, 0.0f32, -10.0f32),
      window.get_display_info().clone(),
      90.0f32,
    ));

    let mut world = World::new();

    for mesh in meshes {
      world.add_pawn(PawnEntity {
        transform: mesh.0,
        static_mesh_id: mesh.1.uid(),
        shader_id: pbr_shader.uid(),
        textures: vec![],
        shader_variables: vec![
          ShaderVariable::new(
            "albedo".to_owned(),
            ShaderVariableType::F32_3(Vector3::new(0.7f32, 0.7, 0.7)),
          ),
          ShaderVariable::new("metallic".to_owned(), ShaderVariableType::F32_1(0.0f32)),
          ShaderVariable::new("roughness".to_owned(), ShaderVariableType::F32_1(0.5f32)),
          ShaderVariable::new("ao".to_owned(), ShaderVariableType::F32_1(0.5f32)),
        ],
      });
      context.static_mesh(&mesh.1).unwrap();
    }

    world.set_skybox(SkyboxEntity {
      static_mesh_id: cube_mesh.uid(),
      shader_id: skybox_shader.uid(),
      rendered_cubemap_id: convoluted_cubemap.uid(),
      irradiance_cubemap_id: convoluted_cubemap.uid(),
    });

    world.add_ui(UIEntity {
      ui: UI::Panel(Panel::new(
        Rect::from_xy(400, 300, 200, 150),
        Color::from_rgb(1.0, 1.0, 1.0),
      )),
      mesh_id: plane_mesh.uid(),
      shader_id: ui_shader.uid(),
    });

    world.set_camera(CameraEntity::FlyCamera(fly_camera));

    // Cube map registration
    context.cubemap(&cubemap).unwrap();
    context.cubemap(&convoluted_cubemap).unwrap();

    // Static mesh registration
    context.static_mesh(&cube_mesh).unwrap();
    context.static_mesh(&plane_mesh).unwrap();

    // Shader registration
    context.shader(&pbr_shader).unwrap();
    context.shader(&equirect_shader).unwrap();
    context.shader(&irradiance_shader).unwrap();
    context.shader(&skybox_shader).unwrap();
    context.shader(&ui_shader).unwrap();

    // Texture registration
    context.texture(&texture).unwrap();
    context.setup();

    // Conduct a render pass here for our equirect projection
    let mut render_tasks = SimpleRenderTasks::new();
    render_tasks.queue_cubemap_projection(
      equirect_shader.uid(),
      cube_mesh.uid(),
      ProjectionTarget::Texture2d(texture.uid()),
      cubemap.uid(),
      cubemap.get_dimension().clone(),
      window.get_display_info().get_dimension().clone(),
      vec![],
    );
    render_tasks.render(context).unwrap();

    // Do a projection again, this time convoluting the cubemap
    let mut render_tasks = SimpleRenderTasks::new();
    render_tasks.queue_cubemap_projection(
      irradiance_shader.uid(),
      cube_mesh.uid(),
      ProjectionTarget::Cubemap(cubemap.uid()),
      convoluted_cubemap.uid(),
      convoluted_cubemap.get_dimension().clone(),
      window.get_display_info().get_dimension().clone(),
      vec![],
    );
    render_tasks.render(context).unwrap();

    ale_opengl_text_render(&inconsolata_font, 24, Vector2::new(0.0, 0.0), "some string");

    Game {
      world,
      inconsolata_font,
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
