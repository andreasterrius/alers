use std::collections::HashMap;
use glfw::{Context, CursorMode};
use ale_data::alevec::{AleVec, AleVecIter, AleVecIterMut, Key};
use ale_ui::panels::Panels;
use crate::display::DisplaySetting;
use crate::window::{Window};

pub struct Windows {
  glfw: glfw::Glfw,
  windows : AleVec<Window>,
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

    if display_setting.is_hidden {
      self.glfw.window_hint(glfw::WindowHint::Visible(false));
    }
    else {
      self.glfw.window_hint(glfw::WindowHint::Visible(true));
    }
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
      false,
    ))
  }

  pub fn poll_inputs(&mut self) {
    self.glfw.poll_events();

    for window in self.windows.iter_mut() {
      window.make_current();
      window.input();
    }
  }

  pub fn cleanup(&mut self){
    let mut to_be_removed = vec!();
    let window_keys : Vec<Key<Window>> = self.windows.keys_iter().collect();

    for key in window_keys {
      let window = self.windows.get_mut(key);
      match window {
        None => {}
        Some(window) => {
          if window.is_closing() {
            to_be_removed.push(key);
          }
        }
      }
    }

    for rem in to_be_removed {
      self.windows.remove_drop(rem);
    }
  }

  pub fn get(&self, key : Key<Window>) -> Option<&Window> {
    return self.windows.get(key)
  }

  pub fn get_mut(&mut self, key : Key<Window>) -> Option<&mut Window> {
    return self.windows.get_mut(key)
  }

  pub fn len(&self) -> usize { self.windows.len() }

  pub fn iter(&self) -> AleVecIter<Window> {
    return self.windows.iter()
  }

  pub fn iter_mut(&mut self) -> AleVecIterMut<Window> {
    return self.windows.iter_mut()
  }
}
