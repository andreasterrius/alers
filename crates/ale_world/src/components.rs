use std::any::TypeId;
use std::collections::HashMap;
use ale_camera::CameraRenderInfo;
use ale_data::indexmap::Id;
use ale_opengl::renderer;
use ale_data::alevec::Key;
use ale_data::entity::{Component, Entity};
use ale_input::Input;

pub trait Tickable: Component {
  fn fixed_tick(&mut self, delta_time: f32);

  fn tick(&mut self, delta_time: f32);
}

pub trait Inputable: Component {
  fn input(&mut self, inputs: &Vec<Input>);
}

pub trait Spawnable {
  fn on_spawn(&mut self);

  fn on_kill(&mut self);

  fn id(&self) -> Id<Entity>;
}