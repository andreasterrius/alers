use ale_data::alevec;
use ale_ui::element::Panel;

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
