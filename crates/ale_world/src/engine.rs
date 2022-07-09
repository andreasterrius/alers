use std::collections::HashMap;

use ale_data::alevec::AleVec;
use ale_resources::resources::Resources;
use ale_ui::element::{Element, Panel};
use ale_ui::panels::Panels;
use ale_window::backend;
use ale_window::backend::Windows;
use ale_window::window::Window;

use crate::viewport::ViewportDescriptor;

pub struct Engine {
  pub windows: Windows,
  pub panels: Panels,
  pub resources: Resources,
  pub viewport_descriptor: AleVec<ViewportDescriptor>,
}

impl Engine {
  pub fn new() -> Engine {
    Engine {
      windows: Windows::new(),
      panels: Panels::new(),
      resources: Resources::new(),
      viewport_descriptor: AleVec::new(),
    }
  }
}