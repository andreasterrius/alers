use std::path::Components;
use std::rc::Rc;
use log::info;
use ale_camera::CameraRenderInfo;
use ale_camera::flycamera::FlyCamera;
use ale_data::indexmap::Key;
use ale_math::rect::Rect;
use ale_math::{Vector3, Zero};
use ale_world::components::{Camera, OnSpawn, Spawnable, Tick};
use ale_world::wire_component;
use ale_world::world::{Entity, World};
use crate::{Editor, Vector2};

#[derive(Debug)]
pub struct EditorCamera {
  pub key: Key<Entity>,
  pub fly_camera: FlyCamera,
}

impl EditorCamera {
  pub fn register_components(world: &mut World) {
    world.register_components(&[
      wire_component!(dyn Camera, EditorCamera),
      wire_component!(dyn Tick, EditorCamera)
    ])
  }

  pub fn new(key : Key<Entity>) -> EditorCamera {
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

impl Tick for EditorCamera {
  fn fixed_tick(&mut self, delta_time: f32) {
  }

  fn tick(&mut self, delta_time: f32) {
  }
}

impl Camera for EditorCamera {
  fn get_camera_info(&mut self) -> CameraRenderInfo {
    self.fly_camera.get_camera_render_info()
  }
}

impl Spawnable for EditorCamera {
  fn get_key(&self) -> Key<Entity> {
    self.key
  }
}