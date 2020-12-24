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
    //glfw_window.set_cursor_mode(CursorMode::Disabled);

    Window {
      glfw_window,
      glfw_events,
      window_size,
      mouse_position: None,
    }
  }

  pub fn should_close(window: &Window) -> bool {
    window.glfw_window.should_close()
  }

  pub fn swap_buffers(window: &mut Window) {
    window.glfw_window.swap_buffers();
  }

  pub fn get_inputs(window: &mut Window) -> Vec<Input> {
    let mut inputs = vec![];
    for (_, event) in glfw::flush_messages(&window.glfw_events) {
      match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.glfw_window.set_should_close(true),
        glfw::WindowEvent::Key(key, scancode, action, modifier) => {
          let input = Input::Key(
            translate_key(key),
            translate_scancode(scancode),
            translate_action(action),
            translate_modifier(modifier),
          );
          inputs.push(input);
        }
        glfw::WindowEvent::CursorPos(x, y) => inputs.push(match window.mouse_position {
          None => {
            window.mouse_position = Some((x, y));
            Input::MouseMotion(0.0f32, 0.0f32)
          }
          Some(mouse_position) => {
            let result = Input::MouseMotion(
              (x - mouse_position.0) as f32 / window.window_size.x as f32,
              (y - mouse_position.1) as f32 / window.window_size.y as f32,
            );
            window.mouse_position = Some((x, y));
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

  pub fn get_screen_size(window: &Window) -> Vector2<u32> {
    window.window_size
  }

  pub fn get_proc_address(window: &mut Window, procname: &str) -> GLProc {
    window.glfw_window.get_proc_address(procname)
  }
}
