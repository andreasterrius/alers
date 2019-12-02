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
use std::borrow::BorrowMut;

mod example;
mod game;

pub fn main() {
  // Initialize File Logging
  alers::log::init();

  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(800, 600);

  let mut context = renderer::opengl::Context::new();
  let mut game = game::Game::load(&mut context);

  // Main Game Loop
  while !window.is_closing() {

    game.tick();

    // Initialize render queue & assign render tasks
    let mut render_tasks = SimpleRenderTasks::new();
    game.render(&mut render_tasks);
    render_tasks.render(&context, game.camera().borrow_mut());

    window.swap_buffers();
    engine.poll_inputs();

    // Add handle events on windows to prevent crash
    game.input(window.input());
  }
}