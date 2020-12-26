use crate::fixed_tick::FixedTick;
use ale_glfw::{Backend, Window};
use ale_input::Input;
use ale_math::Vector2;
use ale_opengl::OpenGL;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod fixed_tick;

pub struct App {
  backend: Backend,
  window: Window,
  fixed_tick: FixedTick,
}

impl App {
  pub fn new(window_size: Vector2<u32>) -> App {
    // Initialize the engine
    let mut backend = Backend::new();
    let mut window = Window::new(&mut backend, window_size);
    let fixed_tick = FixedTick::new();

    // Load the function pointers for opengl
    ale_opengl::raw::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    App {
      backend,
      window,
      fixed_tick,
    }
  }

  pub fn run(&mut self) {
    while !app.window.should_close() {
      app.backend.poll_inputs();

      app.fixed_tick.tick(&mut |delta_time| {});

      OpenGL::clear_buffer();

      app.window.swap_buffers();
    }
  }
}
