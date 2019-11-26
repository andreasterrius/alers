extern crate log;
extern crate alers;

mod example;

use alers::*;
use std::fs;
use alers::renderer::opengl::{SimpleRenderTasks, RenderTasks};
use pile::Pile;
use alers::data::object::Object;

mod pile;

pub fn main() {
  // Initialize File Logging
  alers::log::init();

  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(800, 600);

  // Initialize resources
  let pile = Pile::load_initial();

  // Test create an object
  let cube = Object {
    mesh: &pile.cube_mesh,
    shader: &pile.lambert_shader
  };

  // Initialize render queue & assign render tasks
  let mut render_tasks = SimpleRenderTasks::new();

  // Initialize renderer
  let mut context = renderer::opengl::Context::new();

  // Initialize the windowing system
  while !window.is_closing() {
    engine.poll_inputs();

    context.render(&mut render_tasks);
    window.swap_buffers();
  }
}