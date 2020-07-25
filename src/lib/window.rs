use crate::data::display_info::DisplayInfo;
use crate::input::Input;
use crate::window::input_translator::{translate_action, translate_key, translate_modifier, translate_scancode};
use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use std::sync::mpsc::Receiver;

pub mod input_translator;

pub struct WindowCreator<'a> {
  glfw: &'a mut glfw::Glfw,
}

impl<'a> WindowCreator<'a> {
  pub fn new_creator(glfw: &'a mut glfw::Glfw) -> WindowCreator<'a> {
    WindowCreator { glfw }
  }

  pub fn new(self, display_info: DisplayInfo) -> Window {
    self.glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    self.glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    // glfw window creation
    // --------------------
    let (mut glfw_window, glfw_events) = self
      .glfw
      .create_window(
        display_info.get_dimension().get_width(),
        display_info.get_dimension().get_height(),
        "Alers",
        glfw::WindowMode::Windowed,
      )
      .expect("Failed to create GLFW window");

    glfw_window.make_current();
    glfw_window.set_key_polling(true);
    glfw_window.set_cursor_pos_polling(true);
    glfw_window.set_framebuffer_size_polling(true);
    glfw_window.set_cursor_mode(CursorMode::Disabled);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    gl::load_with(|symbol| glfw_window.get_proc_address(symbol) as *const _);

    Window {
      glfw_window,
      glfw_events,
      display_info,
      mouse_position: None,
    }
  }
}

pub struct Window {
  glfw_window: glfw::Window,
  glfw_events: Receiver<(f64, WindowEvent)>,

  display_info: DisplayInfo,

  mouse_position: Option<(f64, f64)>,
}

impl Window {
  pub fn is_closing(&self) -> bool {
    self.glfw_window.should_close()
  }

  pub fn swap_buffers(&mut self) {
    self.glfw_window.swap_buffers();
  }

  pub fn input(&mut self) -> Vec<Input> {
    let mut inputs = vec![];
    for (_, event) in glfw::flush_messages(&self.glfw_events) {
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => self.glfw_window.set_should_close(true),
        glfw::WindowEvent::Key(key, scancode, action, modifier) => {
          let input = Input::Key(
            translate_key(key),
            translate_scancode(scancode),
            translate_action(action),
            translate_modifier(modifier),
          );
          inputs.push(input);
        }
        glfw::WindowEvent::CursorPos(x, y) => inputs.push(match self.mouse_position {
          None => {
            self.mouse_position = Some((x, y));
            Input::MouseMotion(0.0f32, 0.0f32)
          }
          Some(mouse_position) => {
            let result = Input::MouseMotion(
              (x - mouse_position.0) as f32 / self.display_info.get_dimension().get_width() as f32,
              (y - mouse_position.1) as f32 / self.display_info.get_dimension().get_height() as f32,
            );
            self.mouse_position = Some((x, y));
            result
          }
        }),
        _ => {}
      }
    }
    inputs
  }

  pub fn get_display_info(&self) -> &DisplayInfo {
    &self.display_info
  }
}
