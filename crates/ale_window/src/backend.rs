use glfw::{Context, CursorMode};
use ale_data::alevec::{AleVec, Key};
use crate::display::DisplaySetting;
use crate::window::{Window, WindowCreator};

pub struct Windows {
  glfw: glfw::Glfw,
  windows : AleVec<Window>
}

impl Windows {
  pub fn new() -> Windows {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    Windows {
      glfw,
      windows: AleVec::new()
    }
  }

  pub fn add(&mut self, display_setting: DisplaySetting) -> Key<Window> {
    self.glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    self.glfw
      .window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    // glfw window creation
    // --------------------
    let (mut glfw_window, glfw_events) = self
      .glfw
      .create_window(
        display_setting.get_dimension().size.x,
        display_setting.get_dimension().size.y,
        "Alers",
        glfw::WindowMode::Windowed,
      )
      .expect("Failed to create GLFW window");

    glfw_window.make_current();
    glfw_window.set_char_polling(true);
    glfw_window.set_key_polling(true);
    glfw_window.set_cursor_pos_polling(true);
    glfw_window.set_mouse_button_polling(true);
    glfw_window.set_framebuffer_size_polling(true);
    glfw_window.set_cursor_mode(CursorMode::Normal);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    ale_opengl::raw::load_with(|symbol| glfw_window.get_proc_address(symbol) as *const _);

    self.windows.push(Window::new(
      glfw_window,
      glfw_events,
      display_setting,
    ))
  }

  pub fn creator(&mut self) -> WindowCreator {
    WindowCreator::new_creator(&mut self.glfw)
  }

  pub fn poll_inputs(&mut self) {
    self.glfw.poll_events();
  }
}
