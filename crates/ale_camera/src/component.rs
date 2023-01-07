use ale_data::entity::{Component, Entity};
use ale_data::indexmap::Id;
use crate::CameraRenderInfo;

pub trait Camera: Component {
  fn get_camera_info(&mut self) -> (Id<Entity>, CameraRenderInfo);
}