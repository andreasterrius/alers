extern crate alers;

use alers::*;

pub fn main() {
  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(800, 600);

  let mut cube_fbx = resource::load_fbx("E:/Codes/Repos/alexyt/resources/basic_blender.fbx");

  while !window.is_closing() {
    engine.poll_inputs();
    window.swap_buffers();
  }
}