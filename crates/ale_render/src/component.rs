use crate::target::RenderTarget;
use ale_data::alevec::Key;
use ale_data::entity::Component;
use ale_opengl::renderer::task::RenderTask;
use std::collections::HashMap;

pub trait Renderable: Component {
  fn get_render_tasks(&mut self) -> Vec<RenderTask>;
}
