use std::collections::HashMap;
use hecs::{Entity, View};
use ale_camera::Camera;
use ale_data::alevec::Key;
use ale_ui::element::Panel;
use ale_window::window::Window;
use crate::world::EntityKey;

pub struct ViewportDescriptor {
  pub panel_camera_key: HashMap<String, EntityKey>,
  pub panel_key: Option<Key<Panel>>,
  pub camera_key: Option<EntityKey>,
  pub window_key: Key<Window>,
}

impl ViewportDescriptor {
  pub fn new(camera_key: EntityKey, window_key: Key<Window>) -> ViewportDescriptor {
    ViewportDescriptor {
      panel_camera_key: HashMap::new(),
      camera_key: Some(camera_key),
      panel_key: None,
      window_key,
    }
  }

  pub fn with_panel(camera_key: EntityKey, panel_key: Key<Panel>, window_key: Key<Window>) -> ViewportDescriptor {
    ViewportDescriptor {
      panel_camera_key: HashMap::new(),
      camera_key: Some(camera_key),
      panel_key: Some(panel_key),
      window_key,
    }
  }

  pub fn with_panel_multi(panel_camera_key: HashMap<String, EntityKey>,
                          panel_key: Key<Panel>,
                          window_key: Key<Window>) -> ViewportDescriptor {
    ViewportDescriptor {
      panel_camera_key,
      camera_key: None,
      panel_key: Some(panel_key),
      window_key,
    }
  }
}