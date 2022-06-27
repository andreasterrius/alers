use std::path::Components;
use std::rc::Rc;
use ale_camera::CameraRenderInfo;
use ale_camera::flycamera::FlyCamera;
use ale_math::Vector3;
use ale_world::components::{Camera, OnSpawn};
use ale_world::wire_component;
use ale_world::world::{EntityId, World};
use crate::Editor;

pub struct EditorCamera {
  pub key: EntityKey,
  pub fly_camera: FlyCamera,
}

impl EditorCamera {
  pub fn register_components(world: &mut World) {
    world.enable(&[
      wire_component!(dyn OnSpawn, EditorCamera),
      wire_component!(dyn Camera, EditorCamera),
    ])
  }

  pub fn new() -> EditorCamera {
    let mut fly_camera = FlyCamera::new(Camera::new(
      Vector3::new(0.0, 10.0, 0.0),
      window.get_display_info().dimension.clone(),
      90.0,
    ));

    EditorCamera { key: EntityKey::empty(), fly_camera }
  }
}

impl Camera for EditorCamera {
  fn camera(&mut self) -> CameraRenderInfo {
    self.fly_camera.get_camera_render_info()
  }
}

impl OnSpawn for EditorCamera {
  fn take_key(&mut self, key: EntityKey) {
    self.key = key;
  }
}