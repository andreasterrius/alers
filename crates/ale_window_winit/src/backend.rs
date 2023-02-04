use winit::dpi::{LogicalSize, PhysicalSize};
pub use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowBuilder;

use ale_data::indexmap::{AleIndexMap, Id};

use crate::display::DisplaySetting;
use crate::window::Window;

pub struct Windows {
  windows: AleIndexMap<Window>,
}

impl Windows {
  pub fn new() -> Windows {
    return Windows {
      windows: AleIndexMap::new(),
    };
  }

  pub fn add(&mut self, id: Id<Window>, display_setting: DisplaySetting, event_loop: &EventLoopWindowTarget<()>) {
    let size = display_setting.dimension.size;
    let window = WindowBuilder::new()
      .with_inner_size(LogicalSize::new(size.x,size.y))
      .build(event_loop).unwrap();
    self.windows.insert_wkey(id, Window::new(window, display_setting));
  }

  pub fn get(&self, id: Id<Window>) -> Option<&Window> {
    self.windows.get(&id)
  }
}
