#[macro_use]
extern crate log;
extern crate alers;
extern crate cgmath;

mod example;

use alers::*;
use std::fs;
use alers::renderer::opengl::{SimpleRenderTasks, RenderTasks};
use pile::Pile;
use alers::data::object::Object;
use alers::math::transform::Transform;
use cgmath::{Matrix4, Vector3};
use cgmath::prelude::*;

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
  let mut cube = Object {
    mesh: &pile.cube_mesh,
    shader: &pile.lambert_shader,
    transform : Transform::position(Vector3::from_value(1.0)),
  };

  // Initialize renderer context
  let mut context = renderer::opengl::Context::new();
  context.shader(&pile.lambert_shader).unwrap();
  context.static_mesh(&pile.cube_mesh).unwrap();

  // Initialize the windowing system
  while !window.is_closing() {

    // Initialize render queue & assign render tasks
    let mut render_tasks = SimpleRenderTasks::new();
    render_tasks.queue_static_mesh(cube.shader, cube.mesh, cube.transform.get_matrix());
    render_tasks.render(&context);

    window.swap_buffers();
    engine.poll_inputs();

    // Add handle events on windows to prevent crash
    window.handle_events();
  }
}