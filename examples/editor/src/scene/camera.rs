use std::path::Components;
use std::rc::Rc;
use log::info;
use ale_camera::CameraRenderInfo;
use ale_camera::component::Camera;
use ale_camera::flycamera::FlyCamera;
use ale_data::entity::Entity;
use ale_data::indexmap::Id;
use ale_data::wire_component;
use ale_math::rect::Rect;
use ale_math::{Vector3, Zero};
use ale_world::components::{ Spawnable, Tickable};
use ale_world::world::{World};
use crate::{Editor, Vector2};

#[derive(Debug)]
pub struct EditorCamera {
  pub key: Id<Entity>,
  pub fly_camera: FlyCamera,
}

impl EditorCamera {
  pub fn register_components(world: &mut World) {
    world.register_components(&[
      wire_component!(dyn Camera, EditorCamera),
      wire_component!(dyn Tickable, EditorCamera)
    ])
  }

  pub fn new(key : Id<Entity>) -> EditorCamera {
    let fly_camera = FlyCamera::new(ale_camera::Camera::new(
      Vector3::new(0.0, 10.0, 0.0),
      Rect {
        position: Vector2::zero(),
        size: Vector2::new(800, 600),
      },
      90.0,
    ));

    EditorCamera { key, fly_camera }
  }
}

impl Tickable for EditorCamera {
  fn fixed_tick(&mut self, delta_time: f32) {
  }

  fn tick(&mut self, delta_time: f32) {
  }
}

impl Camera for EditorCamera {
  fn get_camera_info(&mut self) -> (Id<Entity>, CameraRenderInfo) {
    todo!()
  }
}

impl Spawnable for EditorCamera {
  fn on_spawn(&mut self) {
    // do nothing
  }

  fn on_kill(&mut self) {
    // do nothing
  }

  fn id(&self) -> Id<Entity> {
    self.key
  }
}