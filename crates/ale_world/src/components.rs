use std::any::TypeId;
use std::collections::HashMap;
use traitcast_core::TraitcastFrom as Component;
use ale_camera::CameraRenderInfo;
use ale_data::indexmap::Key;
use ale_opengl::renderer;
pub use anyhow::Error;
use crate::world::{Entity, EntityEvent, Event, World};

pub trait Tick: Component {
  fn fixed_tick(&mut self, delta_time: f32);

  fn tick(&mut self, delta_time: f32);
}

pub trait Inputable: Component {
  fn input(&mut self, input: ale_input::Input);
}

pub trait OnSpawn: Component {
  fn take_key(&mut self, key: Key<Entity>);
}

pub trait Renderable: Component {
  fn get_render_task(&mut self) -> renderer::task::Task;
}

pub trait Camera: Component {
  fn get_camera_info(&mut self) -> CameraRenderInfo;
}

pub trait EventSender: Component {
  fn send_event(&mut self) -> Vec<Box<dyn Event>>;
}

pub trait EventListener: Component {
  fn listen_event(&mut self, entity_event: &EntityEvent) -> Result<(), Error>;
}