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
    transform : Transform::position(Vector3::from_value(0.0)),
  };

  // Initialize renderer context
  let mut context = renderer::opengl::Context::new();
  context.shader(&pile.lambert_shader).unwrap();
  context.static_mesh(&pile.cube_mesh).unwrap();

  // Create camera here
  let mut camera = camera::fly_camera::FlyCamera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32),
    Vector3::unit_z(), 90.0f32, 800f32/600f32);

  // Main Game Loop
  while !window.is_closing() {

    // Initialize render queue & assign render tasks
    let mut render_tasks = SimpleRenderTasks::new();
    render_tasks.queue_static_mesh(cube.shader, cube.mesh, cube.transform.calculate_matrix());
    render_tasks.render(&context, &mut camera);

    window.swap_buffers();
    engine.poll_inputs();

    // Add handle events on windows to prevent crash
    window.handle_events();
  }
}