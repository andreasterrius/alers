extern crate alers;
extern crate cgmath;
extern crate log;

use alers::engine::tick::FixedStep;
use alers::renderer::opengl::{RenderTasks, SimpleRenderTasks};
use alers::{engine, renderer};

mod example;
mod game;

fn main() {
  // Initialize File Logging
  alers::log::init_term();

  // Initialize config
  let display_info = game::Game::init_window();

  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(display_info);

  let mut context = renderer::opengl::RenderContext::new();
  let mut game = game::Game::load(&mut context, &window);

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
    game.render(&mut render_tasks, &mut context);

    window.swap_buffers();
  }
}
