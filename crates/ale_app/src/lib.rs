use crate::fixed_tick::FixedTick;
use ale_camera::fly_camera::FlyCamera;
use ale_ecs::World;
use ale_glfw::{Backend, Window};
use ale_input::Input;
use ale_math::Vector2;
use ale_opengl::console::ale_opengl_console_render;
use ale_opengl::OpenGLRenderer;
use ale_resource::ResourcePile;
use std::time::{SystemTime, UNIX_EPOCH};

mod before_tick;
pub mod fixed_tick;
mod tick;

pub struct App {
  backend: Backend,
  window: Window,
  fixed_tick: FixedTick,
  renderer: OpenGLRenderer,
  resource_pile: ResourcePile,
}

impl App {
  pub fn new(window_size: Vector2<u32>) -> App {
    let mut backend = Backend::new();
    let mut window = Window::new(&mut backend, window_size);

    // Load the function pointers for opengl
    ale_opengl::raw::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let fixed_tick = FixedTick::new();
    let mut ogl_renderer = OpenGLRenderer::new();
    let mut resource_pile = ResourcePile::new();

    ogl_renderer.attach_resource_loader(&mut resource_pile);

    App {
      backend,
      window,
      fixed_tick,
      renderer: ogl_renderer,
      resource_pile,
    }
  }

  pub fn run(&mut self, mut world: World, camera: &mut FlyCamera) {
    self.fixed_tick.prepare_tick();

    while !self.window.should_close() {
      self.fixed_tick.prepare_tick();

      before_tick::before_tick(&mut world);

      while self.fixed_tick.frame_time_exists() {
        let delta_time = self.fixed_tick.get_delta_time();

        tick::tick(&mut world, delta_time);

        self.fixed_tick.consume_frame_time();
      }

      self.renderer.render(&mut world, camera);
    }
  }

  pub fn get_mut_resource_pile(&mut self) -> &mut ResourcePile {
    &mut self.resource_pile
  }
}
