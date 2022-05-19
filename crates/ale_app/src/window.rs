use crate::display_info::DisplayInfo;
use crate::input_translator::{translate_action, translate_key, translate_modifier, translate_mousebutton, translate_scancode};
use ale_input::Input;
use ale_math::Vector2;
use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};

pub struct WindowCreator<'a> {
  glfw: &'a mut glfw::Glfw,
}

impl<'a> WindowCreator<'a> {
  pub fn new_creator(glfw: &'a mut glfw::Glfw) -> WindowCreator<'a> {
    WindowCreator { glfw }
  }

  pub fn new(self, display_info: DisplayInfo) -> Window {
    self.glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    self
      .glfw
      .window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    // glfw window creation
    // --------------------
    let (mut glfw_window, glfw_events) = self
      .glfw
      .create_window(
        display_info.get_dimension().size.x,
        display_info.get_dimension().size.y,
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
          if key == glfw::Key::I {
            self.glfw_window.set_cursor_mode(CursorMode::Normal);
          }
          if key == glfw::Key::O {
            self.glfw_window.set_cursor_mode(CursorMode::Disabled);
          }
          inputs.push(input);
        }
        glfw::WindowEvent::CursorPos(x, y) => inputs.push(match self.mouse_position {
          None => {
            self.mouse_position = Some((x, y));
            Input::MouseMotion {
              rel_x: 0.0f32,
              rel_y: 0.0f32,
              abs_x: x as f32,
              abs_y: y as f32,
            }
          }
          Some(mouse_position) => {
            let result = Input::MouseMotion {
              rel_x: (x - mouse_position.0) as f32 / self.display_info.get_dimension().size.x as f32,
              rel_y: (y - mouse_position.1) as f32 / self.display_info.get_dimension().size.y as f32,
              abs_x: x as f32,
              abs_y: y as f32,
            };
            self.mouse_position = Some((x, y));
            result
          }
        }),
        glfw::WindowEvent::Char(char) => {
          inputs.push(Input::Char(char));
        }
        glfw::WindowEvent::MouseButton(mbtn, action, modifier) => {
          inputs.push(Input::MouseButton(translate_mousebutton(mbtn), translate_action(action), translate_modifier(modifier)))
        }
        _ => {}
      }
    }
    inputs
  }

  pub fn get_display_info(&self) -> &DisplayInfo {
    &self.display_info
  }

  pub fn get_screen_size(&self) -> Vector2<u32> {
    Vector2::new(
      self.display_info.dimension.size.x,
      self.display_info.dimension.size.y,
    )
  }
}
