#[macro_use]
extern crate log;
extern crate alers;

mod example;

use alers::*;
use std::fs;
use alers::renderer::opengl::{SimpleRenderTasks, RenderTasks};

pub fn main() {
  // Initialize File Logging
  alers::log::init();

  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(800, 600);

  // Initialize resources
  let mut fbx = resource::fbx::load("resources/test/geom/basic_blender.fbx").unwrap();
  let mut meshes = resource::fbx_convert::to_simple_statich_meshes(fbx);

  // Load shaders
  let mut lambert = resource::shader::ShaderFile::new(
    fs::read_to_string("shaders/test.vs").unwrap(),
    fs::read_to_string("shaders/test.fs").unwrap()
  );

  // Initialize render queue & assign render tasks
  let mut render_queue = SimpleRenderTasks::new();


  // Initialize renderer
  let mut pipeline = renderer::opengl::Context::new();

  // Initialize the windowing system
  while !window.is_closing() {
    engine.poll_inputs();

    pipeline.render(render_queue);

    window.swap_buffers();
  }
}