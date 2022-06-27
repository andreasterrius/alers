use traitcast_core::TraitcastFrom as Component;
use ale_camera::CameraRenderInfo;
use crate::engine::Engine;
use crate::world::{EntityId, World};


pub trait Tick: Component {
  fn fixed_tick(&mut self, delta_time: f32);

  fn tick(&mut self, delta_time: f32);
}

pub trait Input: Component {
  fn input(&mut self, input: ale_input::Input);
}

pub trait OnSpawn: Component {
  fn take_key(&mut self, key: EntityKey);
}

pub trait Render: Component {
  fn render(&mut self) {}
}

pub trait Camera: Component {
  fn camera(&mut self) -> CameraRenderInfo;
}