use crate::display::{DisplaySetting, TargetMonitor};
use crate::input_translator::{
  translate_action, translate_key, translate_modifier, translate_mousebutton, translate_scancode,
};
use ale_input::Input;
use ale_math::{Vector2, Zero};
use ale_ui::element::Panel;
use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, RwLock};
use ale_data::alevec;
use ale_math::rect::Rect;

pub struct Window {
  pub glfw_window: glfw::Window,
  glfw_events: Receiver<(f64, WindowEvent)>,

  pub is_hidden : bool, // hidden window, for context
  pub display_setting: DisplaySetting,

  mouse_position: Option<(f64, f64)>,
  pub panel_key: Option<alevec::Key<Panel>>,
}

impl Window {
  pub fn new(
    glfw_window: glfw::Window,
    glfw_events: Receiver<(f64, WindowEvent)>,
    display_setting: DisplaySetting,
    is_hidden: bool,
  ) -> Window {
    Window {
      glfw_window,
      glfw_events,
      is_hidden,
      display_setting,
      mouse_position: None,
      panel_key: None
    }
  }

  pub fn is_closing(&self) -> bool {
    self.glfw_window.should_close()
  }

  pub fn close(&mut self) {
    self.glfw_window.set_should_close(true);
  }

  pub fn swap_buffers(&mut self) {
    self.glfw_window.swap_buffers();
  }

  pub fn make_current(&mut self) {
    self.glfw_window.make_current();
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
              rel_x: (x - mouse_position.0) as f32 / self.display_setting.get_dimension().size.x as f32,
              rel_y: (y - mouse_position.1) as f32 / self.display_setting.get_dimension().size.y as f32,
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
        glfw::WindowEvent::MouseButton(mbtn, action, modifier) => inputs.push(Input::MouseButton(
          translate_mousebutton(mbtn),
          translate_action(action),
          translate_modifier(modifier),
        )),
        _ => {}
      }
    }
    inputs
  }

  pub fn get_display_info(&self) -> &DisplaySetting {
    &self.display_setting
  }

  pub fn get_screen_size(&self) -> Vector2<u32> {
    Vector2::new(
      self.display_setting.dimension.size.x,
      self.display_setting.dimension.size.y,
    )
  }

  pub fn attach_panel(&mut self, panel : alevec::Key<Panel>) {
    self.panel_key = Some(panel);
  }

  pub fn remove_panel(&mut self, panel: alevec::Key<Panel>) {
    self.panel_key = None;
  }
}
