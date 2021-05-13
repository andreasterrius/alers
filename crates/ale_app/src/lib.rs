use crate::display_info::DisplayInfo;
use crate::tick::{FixedStep, WorldTick};
use crate::window::Window;
use ale_input::Input;
use ale_opengl::old::opengl::{RenderContext, SimpleRenderTasks};
use std::path::Path;

pub mod display_info;
pub mod engine;
pub mod input_translator;
pub mod log;
pub mod tick;
pub mod window;

pub trait App<S> {
  fn load(&mut self, context: &mut RenderContext, window: &Window) -> S;

  fn input(&mut self, s: &mut S, inputs: Vec<Input>);

  fn fixed_tick(&mut self, s: &mut S, delta_time: f32);

  fn tick(&mut self, s: &mut S);

  fn render(&mut self, s: &mut S, render_tasks: SimpleRenderTasks, render_context: &mut RenderContext);
}

pub fn ale_app_run<S, T: App<S>>(mut app: T, display_info: DisplayInfo) {
  // Initialize File Logging
  //alers::log::init_term();

  // Initialize the engine
  let mut engine = engine::Engine::new();
  let mut window = engine.windows().new(display_info);

  let mut context = RenderContext::new();
  let mut state = app.load(&mut context, &window);

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

    // Initialize render queue & assign render tasks
    let mut render_tasks = SimpleRenderTasks::new();
    app.render(&mut state, render_tasks, &mut context);

    window.swap_buffers();
  }
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
