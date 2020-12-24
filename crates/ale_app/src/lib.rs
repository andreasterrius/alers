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

pub fn ale_app_new(window_size: Vector2<u32>) -> App {
  // Initialize the engine
  let mut backend = Backend::new();
  let mut window = Window::new(&mut backend, window_size);
  let fixed_tick = FixedTick::new();

  // Load the function pointers for opengl
  ale_opengl::raw::load_with(|symbol| window.get_proc_address(&mut window, symbol) as *const _);

  App {
    backend,
    window,
    fixed_tick,
  }
}

pub fn ale_app_run(app: &mut App) {
  while !app.window.should_close() {
    app.window.poll_inputs(&mut app.backend);

    input_tick(window.get_inputs());
    fixed_tick.tick(&mut app.fixed_tick, physics_tick);

    OpenGL::clear_buffer();

    app.window.swap_buffers();
  }
}

pub fn ale_app_running(app: &mut App) -> bool {
  ale_glfw_window_buffers_swap(&mut app.window);
  ale_glfw_backend_poll_inputs(&mut app.backend);
  ale_glfw_window_inputs_get(&mut app.window);

  return !ale_glfw_window_should_close(&app.window);
}
