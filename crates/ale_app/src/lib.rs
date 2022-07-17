use std::fs::File;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use thiserror::Error;

use ale_input::Input;
use ale_opengl::old::opengl::{RenderResources, SimpleRenderTasks};

use ale_window::display::DisplaySetting;
use ale_window::tick::{FixedStep, WorldTick};
use ale_window::window::Window;

pub use anyhow::Error as AppError;
use log::{info, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, SimpleLogger, TerminalMode, TermLogger, WriteLogger};
use ale_opengl::viewport::Viewport;
use ale_ui::element;
use ale_window::backend;
use ale_window::backend::Windows;
use ale_world::world;
use ale_world::world::World;

pub mod app;
pub mod engine;
pub mod visitor;

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
  init_term();

  // Initialize the engine
  let mut windows = Windows::new();
  let mut window_key = windows.add(display_info);

  let window = windows.get(window_key).unwrap();
  let mut state = app.load(window)?;

  let mut tick = WorldTick::FixedStep(FixedStep::new(0.01f32));

  // Main Game Loop
  while windows.len() != 0 {
    windows.poll_inputs();

    let window = windows.get_mut(window_key).unwrap();
    app.input(&mut state, window.input());

    tick.prepare_tick();
    while tick.should_tick() {
      tick.tick();
      app.fixed_tick(&mut state, tick.delta_time());
    }

    app.tick(&mut state);
    app.render(&mut state);

    window.swap_buffers();
    windows.cleanup();
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

pub fn init() {
  let now_ms = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards")
      .as_millis();
  CombinedLogger::init(vec![
    TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
    WriteLogger::new(
      LevelFilter::Info,
      Config::default(),
      File::create(format!("alers-{}.log", now_ms)).unwrap(),
    ),
  ])
      .unwrap();
}

pub fn init_term() {
  TermLogger::init(LevelFilter::Debug,
                   Config::default(),
                   TerminalMode::Mixed,
                   ColorChoice::Auto).unwrap();
}
