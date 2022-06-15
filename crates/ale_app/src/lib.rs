use std::path::Path;

use thiserror::Error;

use ale_input::Input;
use ale_opengl::old::opengl::{RenderResources, SimpleRenderTasks};

use crate::display::DisplaySetting;
use crate::tick::{FixedStep, WorldTick};
use crate::window::Window;

pub use anyhow::Error as AppError;
use ale_opengl::viewport::Viewport;
use ale_ui::element;
use ale_world::world;
use ale_world::world::World;

pub mod display;
pub mod engine;
pub mod input_translator;
pub mod log;
pub mod tick;
pub mod window;
pub mod app;

// TODO: Break this to 1 function per trait
pub trait App<S> {
  fn load(&mut self, window: &Window) -> Result<S, anyhow::Error>;

  fn input(&mut self, s: &mut S, inputs: Vec<Input>);

  fn fixed_tick(&mut self, s: &mut S, delta_time: f32);

  fn tick(&mut self, s: &mut S);

  fn render(&mut self, s: &mut S);
}

pub fn ale_app_run<S, T: App<S>>(mut app: T, display_info: DisplaySetting) {
  let err = ale_app_run_internal(app, display_info);
  match err {
    Err(err) => {
      println!("{}", err);
    }
    _ => {}
  }
}

pub fn ale_app_run_internal<S, T: App<S>>(mut app: T, display_info: DisplaySetting) -> anyhow::Result<()> {
  // Initialize File Logging
  //alers::log::init_term();

  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(display_info);

  let mut state = app.load(&window)?;

  let mut tick = WorldTick::FixedStep(FixedStep::new(0.01f32));

  // Main Game Loop
  while !window.is_closing() {
    engine.poll_inputs();
    app.input(&mut state, window.input());

    tick.prepare_tick();
    while tick.should_tick() {
      tick.tick();
      app.fixed_tick(&mut state, tick.delta_time());
    }

    app.tick(&mut state);
    app.render(&mut state);

    window.swap_buffers();
  }

  Ok(())
}

pub fn ale_app_resource_path(path: &str) -> String {
  let p = Path::new(env!("CARGO_MANIFEST_DIR"))
    .parent()
    .unwrap()
    .parent()
    .unwrap()
    .join("resources")
    .join(path);
  p.to_str().unwrap().to_owned()
}
