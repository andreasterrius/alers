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

  while !window.is_closing() {
    engine.poll_inputs();
    window.swap_buffers();
  }
}