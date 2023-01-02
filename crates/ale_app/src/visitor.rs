use ale_camera::CameraRenderInfo;
use ale_opengl::renderer;
use ale_world::components::{Camera, Renderable, Tickable};
use ale_world::visitor;
use ale_world::visitor::{Visitor, VisitorMut};

pub struct CameraVisitor {
  pub camera_render_info: Vec<CameraRenderInfo>,
}

impl VisitorMut<dyn Camera> for CameraVisitor {
  fn visit(&mut self, camera: &mut (dyn Camera + 'static)) {
    self.camera_render_info.push(camera.get_camera_info());
  }
}

pub struct RenderableVisitor {
  pub render_task: Vec<renderer::task::Task>,
}

impl VisitorMut<dyn Renderable> for RenderableVisitor {
  fn visit(&mut self, component: &mut (dyn Renderable + 'static)) {
    self.render_task.push(component.get_render_task());
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
