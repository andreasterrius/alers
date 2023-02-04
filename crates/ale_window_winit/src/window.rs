use ale_data::alevec;
use ale_ui::element::Panel;
use raw_window_handle::{
  HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};

use crate::display::DisplaySetting;

pub struct Window {
  pub winit_window: winit::window::Window,
  pub display_setting: DisplaySetting,
  mouse_position: Option<(f64, f64)>,
  pub panel_key: Option<alevec::Key<Panel>>,
}

impl Window {
  pub fn new(winit_window: winit::window::Window, display_setting: DisplaySetting) -> Window {
    Window {
      winit_window,
      display_setting,
      mouse_position: None,
      panel_key: None,
    }
  }
}

unsafe impl HasRawWindowHandle for Window {
  fn raw_window_handle(&self) -> RawWindowHandle {
    self.winit_window.raw_window_handle()
  }
}

unsafe impl HasRawDisplayHandle for Window {
  fn raw_display_handle(&self) -> RawDisplayHandle {
    self.winit_window.raw_display_handle()
  }
}
