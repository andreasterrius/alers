extern crate alers;
extern crate cgmath;
#[macro_use]
extern crate log;

use std::fs;

use cgmath::{Matrix4, Vector3};
use cgmath::prelude::*;

use alers::*;
use alers::math::transform::Transform;
use alers::renderer::opengl::{RenderTasks, SimpleRenderTasks};

mod example;
mod game;

pub fn main() {
  // Initialize File Logging
  alers::log::init();

  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(800, 600);

  let mut context = renderer::opengl::Context::new();

  // Create camera here
  let mut camera = camera::fly_camera::FlyCamera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32),
    Vector3::unit_z(), 90.0f32, 800f32 / 600f32);

  let mut game = game::Game::new();
  game.load(&mut context);

  // Main Game Loop
  while !window.is_closing() {

    // Initialize render queue & assign render tasks
    let mut render_tasks = SimpleRenderTasks::new();
    render_tasks.render(&context, &mut camera);

    window.swap_buffers();
    engine.poll_inputs();

    // Add handle events on windows to prevent crash
    window.handle_events();
  }
}