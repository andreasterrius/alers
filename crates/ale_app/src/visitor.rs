use ale_camera::component::Camera;
use ale_camera::CameraRenderInfo;
use ale_data::alevec::Key;
use ale_data::entity::Entity;
use ale_data::indexmap::Id;
use ale_opengl::renderer;
use ale_opengl::renderer::task::RenderTask;
use ale_render::component::Renderable;
use ale_render::target::RenderTarget;
use ale_world::components::Tickable;
use ale_world::visitor::VisitorMut;
use std::collections::HashMap;

pub struct CameraVisitor {
  pub camera_render_info: HashMap<Id<Entity>, CameraRenderInfo>,
}

impl VisitorMut<dyn Camera> for CameraVisitor {
  fn visit(&mut self, camera: &mut (dyn Camera + 'static)) {
    let (key, renderable) = camera.get_camera_info();
    self.camera_render_info.insert(key, renderable);
  }
}

pub struct RenderableVisitor {
  pub render_tasks: Vec<RenderTask>,
}

impl VisitorMut<dyn Renderable> for RenderableVisitor {
  fn visit(&mut self, component: &mut (dyn Renderable + 'static)) {
    self.render_tasks.extend(component.get_render_tasks())
  }
}

pub struct TickVisitor {
  pub delta_time: f32,
}

impl VisitorMut<dyn Tickable> for TickVisitor {
  fn visit(&mut self, component: &mut (dyn Tickable + 'static)) {
    component.tick(self.delta_time)
  }
}

pub struct FixedTickVisitor {
  pub delta_time: f32,
}

impl VisitorMut<dyn Tickable> for FixedTickVisitor {
  fn visit(&mut self, component: &mut (dyn Tickable + 'static)) {
    component.fixed_tick(self.delta_time)
  }
}
