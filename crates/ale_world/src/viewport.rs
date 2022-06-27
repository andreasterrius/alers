use ale_data::alevec::Key;
use ale_window::window::Window;
use crate::world::EntityId;

pub struct ViewportDescriptor {
  pub camera_id : EntityId,
  pub window_id: WindowId
}