use ale_app::{ale_app_resource_path, ale_app_run, App};

use std::fs;

use ale_camera::flycamera::FlyCamera;
use ale_camera::Camera;
use ale_font::{ale_font_layout, ale_font_load, Font};
use ale_gltf::ale_gltf_load;
use ale_input::Input;
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_mesh::{ale_mesh_cube_new, ale_mesh_plane_new, Mesh};
use ale_opengl::mesh::{ale_opengl_mesh_context_new, OpenGLMeshContext};
use ale_opengl::shader::{ale_opengl_shader_context_new, OpenGLShaderContext};
use ale_opengl::texture::{ale_opengl_texture_context_new, OpenGLTextureContext};

use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_console::{
  ale_console_input, ale_console_new, ale_console_variable_has_event, ale_console_variable_register, Console,
};
use ale_math::transform::AleTransform;
use ale_math::{Vector2, Vector3};
use ale_opengl::console::ale_opengl_console_render;
use ale_opengl::old::cubemap::Cubemap;
use ale_opengl::old::entity::camera::CameraEntity;
use ale_opengl::old::entity::pawn::PawnEntity;
use ale_opengl::old::entity::skybox::SkyboxEntity;
use ale_opengl::old::entity::world::World;
use ale_opengl::old::opengl::{ProjectionTarget, RenderContext, RenderTasks, SimpleRenderTasks};
use ale_opengl::raw::enable_depth_test;
use ale_opengl::render_frame::{
  ale_opengl_render_frame_capture, ale_opengl_render_frame_new, ale_opengl_render_frame_render,
  OpenGLRenderFrameContext,
};
use ale_opengl::text::{ale_opengl_text_font_context_new, ale_opengl_text_render, OpenGLTextFontContext};
use ale_opengl::wire::{
  ale_opengl_wire_boundingbox_render, ale_opengl_wire_console_variable_refresh,
  ale_opengl_wire_console_variable_register, ale_opengl_wire_context_new, OpenGLWireContext,
};
use ale_opengl::{ale_opengl_blend_enable, ale_opengl_clear_render, ale_opengl_depth_test_enable};
use ale_opengl_fxaa::{
  ale_opengl_fxaa_console_variable_refresh, ale_opengl_fxaa_console_variable_register, ale_opengl_fxaa_context_new,
  ale_opengl_fxaa_render, OpenGLFXAAContext,
};
use ale_shader::{ale_shader_new, Shader};
use ale_texture::ale_texture_load;
use ale_variable::Variable;

pub struct State {
  world: World,

  screen_size: Vector2<i32>,
  inconsolata_font: Font,

  console: Console,

  opengl_main_render_frame_context: OpenGLRenderFrameContext,
  opengl_font_context: OpenGLTextFontContext,
  opengl_fxaa_context: OpenGLFXAAContext,
  opengl_wire_context: OpenGLWireContext,

  meshes: Vec<(AleTransform, Mesh)>,
}

struct CompleteExample;
impl App<State> for CompleteExample {
  fn load(&mut self, context: &mut RenderContext, window: &Window) -> State {
    let opengl_main_render_frame_context = ale_opengl_render_frame_new(window.get_screen_size());
    let opengl_fxaa_context = ale_opengl_fxaa_context_new();
    let opengl_wire_context = ale_opengl_wire_context_new();
    let opengl_font_context = ale_opengl_text_font_context_new();

    let screen_size = Vector2::new(800, 600);

    let resource_base_path = "/home/alether/Codes/Graphics/alers/resources";
    let shader_base_path = "/home/alether/Codes/Graphics/alers/resources/shaders";

    // let meshes = ale_gltf_load(&format!("{}/{}", resource_base_path, "test/sceneg.gltf"));
    let meshes = ale_gltf_load(&ale_app_resource_path("gltf/sphere.gltf"));

    let cube_mesh = ale_mesh_cube_new();
    let plane_mesh = ale_mesh_plane_new();

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
    let texture = ale_texture_load(&format!("{}/{}", resource_base_path, "hdr/GravelPlaza_Env.hdr")).unwrap();

    // Load cubemap
    let cubemap = Cubemap::new(Rect::new(512, 512));
    let convoluted_cubemap = Cubemap::new(Rect::new(32, 32));

    // Load fonts
    let mut inconsolata_font = ale_font_load(&format!("{}/{}", resource_base_path, "font/Inconsolata-Regular.ttf"));

    // Load camera
    let fly_camera = FlyCamera::new(Camera::new(
      Vector3::new(0.0f32, 0.0f32, -10.0f32),
      window.get_display_info().dimension.clone(),
      90.0f32,
    ));

    let mut world = World::new();

    for (transform, mesh) in &meshes {
      world.add_pawn(PawnEntity {
        transform: *transform,
        static_mesh_id: mesh.uid(),
        shader_id: pbr_shader.uid(),
        textures: vec![],
        shader_variables: vec![
          Variable::F32_3("albedo".to_owned(), Vector3::new(0.7f32, 0.7, 0.7)),
          Variable::F32_1("metallic".to_owned(), 0.0f32),
          Variable::F32_1("roughness".to_owned(), 0.5f32),
          Variable::F32_1("ao".to_owned(), 0.5f32),
        ],
      });
      context.static_mesh(&mesh).unwrap();
    }

    world.set_skybox(SkyboxEntity {
      static_mesh_id: cube_mesh.uid(),
      shader_id: skybox_shader.uid(),
      rendered_cubemap_id: convoluted_cubemap.uid(),
      irradiance_cubemap_id: convoluted_cubemap.uid(),
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

    let mut console = ale_console_new(100);
    ale_opengl_fxaa_console_variable_register(&opengl_fxaa_context, &mut console);
    ale_opengl_wire_console_variable_register(&opengl_wire_context, &mut console);

    // Setup the opengl renderer;
    ale_opengl_blend_enable();
    ale_opengl_depth_test_enable();

    State {
      world,
      inconsolata_font,
      console,
      screen_size,
      opengl_main_render_frame_context,
      opengl_font_context,
      opengl_fxaa_context,
      opengl_wire_context,
      meshes,
    }
  }

  fn input(&mut self, state: &mut State, inputs: Vec<Input>) {
    if !state.console.has_focus {
      state.world.input(&inputs);
    }

    for input in &inputs {
      ale_console_input(&mut state.console, input);
    }

    if ale_console_variable_has_event(&state.console) {
      ale_opengl_fxaa_console_variable_refresh(&mut state.opengl_fxaa_context, &mut state.console);
      ale_opengl_wire_console_variable_refresh(&mut state.opengl_wire_context, &mut state.console);
    }
  }

  fn tick(&mut self, s: &mut State, delta_time: f32) {
    s.world.tick(delta_time);
  }

  fn render(&mut self, s: &mut State, mut render_tasks: SimpleRenderTasks, context: &mut RenderContext) {
    let world = &mut s.world;
    let camera_render_info = world.get_camera_render_info();

    let opengl_wire_context = &mut s.opengl_wire_context;
    let meshes = &mut s.meshes;

    // Capture the scene render to a render frame
    ale_opengl_render_frame_capture(&s.opengl_main_render_frame_context, || {
      ale_opengl_clear_render();

      world.render::<SimpleRenderTasks>(&mut render_tasks);
      render_tasks.render(&context).unwrap();

      ale_opengl_wire_boundingbox_render(opengl_wire_context, meshes, &camera_render_info);
    });

    // Render the frame with fxaa
    ale_opengl_fxaa_render(&s.opengl_fxaa_context, &s.opengl_main_render_frame_context);

    // Render the console after postprocessing
    ale_opengl_console_render(
      &mut s.opengl_font_context,
      &s.world.get_camera_render_info(),
      &s.console,
      s.screen_size,
      &mut s.inconsolata_font,
    );
  }
}

fn main() {
  let app = CompleteExample;

  ale_app_run(app, DisplayInfo::new(Rect::new(800, 600)));
}
