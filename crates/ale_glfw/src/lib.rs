use crate::input_translator::{translate_action, translate_key, translate_modifier, translate_scancode};
use ale_input::Input;
use ale_math::Vector2;
use glfw::{Action, Context, CursorMode, GLProc, Key, WindowEvent};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};

mod input_translator;

pub struct Backend {
  pub(crate) glfw: glfw::Glfw,
}

impl Backend {
  pub fn new() -> Backend {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    Backend { glfw }
  }

  pub fn poll_inputs(&mut self) {
    return self.glfw.poll_events();
  }
}

pub struct Window {
  pub(crate) glfw_window: glfw::Window,
  pub(crate) glfw_events: Receiver<(f64, WindowEvent)>,

  pub(crate) window_size: Vector2<u32>,
  pub(crate) mouse_position: Option<(f64, f64)>,
}

impl Window {
  pub fn new(backend: &mut Backend, window_size: Vector2<u32>) -> Window {
    let glfw = &mut backend.glfw;
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    // glfw window creation
    // --------------------
    let (mut glfw_window, glfw_events) = glfw
      .create_window(
        window_size.x as u32,
        window_size.y as u32,
        "Alers",
        glfw::WindowMode::Windowed,
      )
      .expect("Failed to create GLFW window");

    glfw_window.make_current();
    glfw_window.set_char_polling(true);
    glfw_window.set_key_polling(true);
    glfw_window.set_cursor_pos_polling(true);
    glfw_window.set_framebuffer_size_polling(true);
    //glfw_self.set_cursor_mode(CursorMode::Disabled);

    Window {
      glfw_window,
      glfw_events,
      window_size,
      mouse_position: None,
    }
  }

  pub fn should_close(&mut self) -> bool {
    self.glfw_window.should_close()
  }

  pub fn swap_buffers(&mut self) {
    self.glfw_window.swap_buffers();
  }

  pub fn get_inputs(&mut self) -> Vec<Input> {
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
              (x - mouse_position.0) as f32 / self.window_size.x as f32,
              (y - mouse_position.1) as f32 / self.window_size.y as f32,
            );
            self.mouse_position = Some((x, y));
            result
          }
        }),
        glfw::WindowEvent::Char(char) => {
          inputs.push(Input::Char(char));
        }
        _ => {}
      }
    }
    inputs
  }

  pub fn get_screen_size(&self) -> Vector2<u32> {
    self.window_size
  }

  pub fn get_proc_address(&mut self, procname: &str) -> GLProc {
    self.glfw_window.get_proc_address(procname)
  }
}
