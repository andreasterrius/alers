use ale_camera::CameraRenderInfo;
use ale_camera::component::Camera;
use ale_data::entity::Entity;
use ale_data::indexmap::Id;
use ale_data::wire_component;
use ale_math::rect::Rect;
use ale_math::Vector3;
use ale_world::components::{Spawnable};
use ale_world::world::{World};

pub struct Camera2D {
  id : Id<Entity>,
  camera : ale_camera::Camera,
}

impl Camera2D {
  pub fn register_components(world : &mut World) {
    world.register_components(&[
      wire_component!(dyn Camera, Camera2D),
      wire_component!(dyn Spawnable, Camera2D)
    ])
  }

  pub fn new(position : Vector3<f32>, rect : Rect ) -> Camera2D {
    Camera2D{ id: Id::new(), camera: ale_camera::Camera::new(position, rect, 90.0) }
  }
}

impl Camera for Camera2D {
  fn get_camera_info(&mut self) -> (Id<Entity>, CameraRenderInfo) {
    (self.id, self.camera.camera_render_info())
  }
}

impl Spawnable for Camera2D {
  fn on_spawn(&mut self) {}

  fn on_kill(&mut self) {}

  fn id(&self) -> Id<Entity> {
    self.id
  }
}