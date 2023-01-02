use std::any::TypeId;
use std::collections::HashMap;
use traitcast_core::TraitcastFrom as Component;
use ale_camera::CameraRenderInfo;
use ale_data::indexmap::Key;
use ale_opengl::renderer;
pub use anyhow::Error;
use crate::world::{Entity, World};

pub trait Tickable: Component {
  fn fixed_tick(&mut self, delta_time: f32);

  fn tick(&mut self, delta_time: f32);
}

pub trait Inputable: Component {
  fn input(&mut self, input: ale_input::Input);
}

pub trait Renderable: Component {
  fn get_render_task(&mut self) -> renderer::task::Task;
}

pub trait Camera: Component {
  fn get_camera_info(&mut self) -> CameraRenderInfo;
}

pub trait Spawnable {
  fn on_spawn(&mut self);

  fn on_kill(&mut self);

  fn get_key(&self) -> Key<Entity>;
}