use crate::window::WindowCreator;

pub struct Engine {
  glfw: glfw::Glfw,
}

impl Engine {
  pub fn new() -> Engine {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    Engine { glfw }
  }

  pub fn windows(&mut self) -> WindowCreator {
    WindowCreator::new_creator(&mut self.glfw)
  }

  pub fn poll_inputs(&mut self) {
    self.glfw.poll_events();
  }
}
