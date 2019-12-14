extern crate alers;
extern crate cgmath;
extern crate log;

use alers::*;
use alers::data::display_info::DisplayInfo;
use alers::engine::tick::FixedStep;
use alers::renderer::opengl::{RenderTasks, SimpleRenderTasks};

mod example;
mod game;

pub fn main() {
  // Initialize File Logging
  alers::log::init();

  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(DisplayInfo::new(800, 600));

  let mut context = renderer::opengl::Context::new();
  let mut game = game::Game::load(&mut context);

  let mut tick = engine::tick::WorldTick::FixedStep(FixedStep::new(0.01f32));

  // Main Game Loop
  while !window.is_closing() {
    engine.poll_inputs();
    game.input(window.input());

    tick.prepare_tick();
    while tick.should_tick() {
      game.tick(tick.delta_time());

      tick.tick();
    }

    // Initialize render queue & assign render tasks
    let mut render_tasks = SimpleRenderTasks::new();
    game.render(&mut render_tasks);
    render_tasks.render(&context, &mut game.camera_render_info());

    window.swap_buffers();
  }
}