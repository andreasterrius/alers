use std::fs;

use ale_camera::flycamera::FlyCamera;
use ale_camera::Camera;
use ale_font::{ale_font_layout, ale_font_load, Font};
use ale_gltf::ale_gltf_load;
use ale_input::Input;
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_mesh::{ale_mesh_cube_new, ale_mesh_plane_new};
use ale_opengl::mesh::{ale_opengl_mesh_context_new, OpenGLMeshContext};
use ale_opengl::shader::{
  ale_opengl_shader_context_new, ale_opengl_shader_variable_new, OpenGLShaderContext, OpenGLShaderVariable,
  OpenGLShaderVariableType,
};
use ale_opengl::texture::{ale_opengl_texture_context_new, OpenGLTextureContext};

use ale_console::{ale_console_input, ale_console_new, Console};
use ale_opengl::console::ale_opengl_console_render;
use ale_opengl::raw::enable_depth_test;
use ale_opengl::render_frame::{
  ale_opengl_render_frame_capture, ale_opengl_render_frame_new, ale_opengl_render_frame_render,
  OpenGLRenderFrameContext,
};
use ale_opengl::text::ale_opengl_text_render;
use ale_opengl::{ale_opengl_blend_enable, ale_opengl_clear_render, ale_opengl_depth_test_enable};
use ale_shader::{ale_shader_new, Shader};
use ale_texture::ale_texture_load;
use alers::data::display_info::DisplayInfo;
use alers::entity::camera::CameraEntity;
use alers::entity::pawn::PawnEntity;
use alers::entity::skybox::SkyboxEntity;
use alers::entity::ui::UIEntity;
use alers::entity::world::World;
use alers::renderer::opengl::{ProjectionTarget, RenderContext, RenderTasks, SimpleRenderTasks};
use alers::resource;
use alers::ui::panel::Panel;
use alers::ui::UI;
use alers::window::Window;
use cgmath::{Vector2, Vector3};
use log::info;

pub struct Game {
  world: World,

  screen_size: Vector2<i32>,
  inconsolata_font: Font,

  console: Console,

  opengl_texture_context: OpenGLTextureContext,
  opengl_mesh_context: OpenGLMeshContext,
  opengl_shader_context: OpenGLShaderContext,

  opengl_main_render_frame_context: OpenGLRenderFrameContext,
}

impl Game {
  pub fn init_window() -> DisplayInfo {
    DisplayInfo::new(Rect::new(800, 600))
  }

  pub fn load(context: &mut RenderContext, window: &Window) -> Game {
    let opengl_texture_context = ale_opengl_texture_context_new();
    let opengl_mesh_context = ale_opengl_mesh_context_new();
    let opengl_shader_context = ale_opengl_shader_context_new();
    let opengl_main_render_frame_context = ale_opengl_render_frame_new(window.get_screen_size());
    let screen_size = Vector2::new(800, 600);

    //let resource_base_path = "E:\\Codes\\Repos\\alers\\resources";
    //let shader_base_path = "E:\\Codes\\Repos\\alers\\shaders";
    let resource_base_path = "/home/alether/Codes/Graphics/alers/resources";
    let shader_base_path = "/home/alether/Codes/Graphics/alers/shaders";

    // Load meshes
    // let meshes = resource::fbx_convert::to_static_meshes(
    //   resource::fbx::load(&format!("{}/{}", resource_base_path, "test/scene.fbx")).unwrap(),
    // )
    // .unwrap();

    let meshes = ale_gltf_load(&format!("{}/{}", resource_base_path, "test/sceneg.gltf"));

    let cube_mesh = ale_mesh_cube_new();
    let plane_mesh = ale_mesh_plane_new();

    // Load skeletal meshes
    // let _skeletal_meshes = resource::fbx_convert::to_skeletal_meshes(
    //   resource::fbx::load(&format!("{}/{}", resource_base_path, "test/anim_begin.fbx")).unwrap(),
    // )
    // .unwrap();

    // Load shaders
    let pbr_shader = ale_shader_new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "pbr.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "pbr.frag")).unwrap(),
    );
    let equirect_shader = ale_shader_new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "cubemap.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "equirect.frag")).unwrap(),
    );
    let irradiance_shader = ale_shader_new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "cubemap.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "irradiance.frag")).unwrap(),
    );
    let skybox_shader = ale_shader_new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "skybox.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "skybox.frag")).unwrap(),
    );
    let ui_shader = ale_shader_new(
      fs::read_to_string(format!("{}/{}", shader_base_path, "ui.vert")).unwrap(),
      fs::read_to_string(format!("{}/{}", shader_base_path, "ui.frag")).unwrap(),
    );

    // Load textures
    let texture = ale_texture_load(&format!("{}/{}", resource_base_path, "test/hdr/GravelPlaza_Env.hdr")).unwrap();

    // Load cubemap
    let cubemap = resource::cubemap::Cubemap::new(Rect::new(512, 512));
    let convoluted_cubemap = resource::cubemap::Cubemap::new(Rect::new(32, 32));

    // Load fonts
    let mut inconsolata_font = ale_font_load(&format!(
      "{}/{}",
      resource_base_path, "font/Inconsolata/static/Inconsolata-Regular.ttf"
    ));

    // Load camera
    let fly_camera = FlyCamera::new(Camera::new(
      Vector3::new(0.0f32, 0.0f32, -10.0f32),
      window.get_display_info().dimension.clone(),
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
          ale_opengl_shader_variable_new(
            "albedo".to_owned(),
            OpenGLShaderVariableType::F32_3(Vector3::new(0.7f32, 0.7, 0.7)),
          ),
          ale_opengl_shader_variable_new("metallic".to_owned(), OpenGLShaderVariableType::F32_1(0.0f32)),
          ale_opengl_shader_variable_new("roughness".to_owned(), OpenGLShaderVariableType::F32_1(0.5f32)),
          ale_opengl_shader_variable_new("ao".to_owned(), OpenGLShaderVariableType::F32_1(0.5f32)),
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

    let console = ale_console_new(100);

    // Setup the opengl renderer;
    ale_opengl_blend_enable();
    //ale_opengl_depth_test_enable();

    Game {
      world,
      inconsolata_font,
      console,
      screen_size,
      opengl_texture_context,
      opengl_mesh_context,
      opengl_shader_context,
      opengl_main_render_frame_context,
    }
  }

  pub fn input(&mut self, inputs: Vec<Input>) {
    if !self.console.has_focus {
      self.world.input(&inputs);
    }

    for input in &inputs {
      ale_console_input(&mut self.console, input);
    }
  }

  pub fn tick(&mut self, delta_time: f32) {
    self.world.tick(delta_time);
  }

  pub fn render<T: RenderTasks>(&mut self, render_tasks: &mut T, context: &mut RenderContext) {
    let opengl_texture_context = &mut self.opengl_texture_context;
    let opengl_mesh_context = &self.opengl_mesh_context;
    let opengl_shader_context = &self.opengl_shader_context;
    let camera_render_info = self.world.get_camera_render_info();
    let inconsolata_font = &mut self.inconsolata_font;
    let console = &self.console;
    let world = &mut self.world;
    let screen_size = self.screen_size;

    ale_opengl_render_frame_capture(&self.opengl_main_render_frame_context, || {
      ale_opengl_clear_render();

      world.render::<T>(render_tasks);
      render_tasks.render(&context).unwrap();

      ale_opengl_console_render(
        opengl_texture_context,
        opengl_mesh_context,
        opengl_shader_context,
        &camera_render_info,
        console,
        screen_size,
        inconsolata_font,
      );
    });

    ale_opengl_render_frame_render(
      &self.opengl_main_render_frame_context,
      &self.opengl_shader_context,
      &self.opengl_mesh_context,
    );
  }
}
