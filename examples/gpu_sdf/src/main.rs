use ale_app::window::Window;
use ale_app::{ale_app_run, App};
use ale_input::Input;
use ale_opengl::old::opengl::{RenderContext, SimpleRenderTasks};

fn main() {
  ale_app_run()
}

struct GPUSDFDemo;

struct State {}

impl App<State> for GPUSDFDemo {
  fn load(&mut self, window: &Window) -> State {}

  fn input(&mut self, s: &mut State, inputs: Vec<Input>) {}

  fn fixed_tick(&mut self, s: &mut State, delta_time: f32) {}

  fn tick(&mut self, s: &mut State) {}

  fn render(&mut self, s: &mut State) {}
}
