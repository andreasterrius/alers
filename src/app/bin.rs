#[macro_use]
extern crate log;
extern crate alers;

mod example;
use alers::*;

pub fn main() {
  // Initialize File Logging
  alers::log::init();

  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(800, 600);

  // Initialize resources
  let mut fbx = resource::fbx::load("resources/test/geom/basic_blender.fbx").unwrap();
  let mut meshes = resource::fbx_convert::to_simple_statich_meshes(fbx);

  // Iniitalize renderer
  let mut pipeline = renderer::opengl::Context::new();

  // Initialize the windowing system
  while !window.is_closing() {
    engine.poll_inputs();
    window.swap_buffers();
  }
}