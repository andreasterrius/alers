use std::any::TypeId;
use std::collections::HashMap;
use ale_camera::CameraRenderInfo;
use ale_data::indexmap::Key;
use ale_opengl::renderer;
use crate::typecast::registry::TraitcastFrom as Component;
use crate::world::{Entity, EntityKey, World};


pub trait Tick: Component {
  fn fixed_tick(&mut self, delta_time: f32);

  fn tick(&mut self, delta_time: f32);
}

pub trait Inputable: Component {
  fn input(&mut self, input: ale_input::Input);
}

pub trait OnSpawn: Component {
  fn take_key(&mut self, key: EntityKey);
}

pub trait Renderable: Component {
  fn get_render_task(&mut self) -> renderer::task::Task;
}

pub trait Camera: Component {
  fn get_camera_info(&mut self) -> CameraRenderInfo;
}