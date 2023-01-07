use std::collections::HashMap;
use ale_data::alevec::Key;
use ale_data::entity::Component;
use ale_opengl::renderer::task::RenderTask;
use crate::target::RenderTarget;

pub trait Renderable: Component {
  fn get_render_task(&mut self) -> HashMap<Key<RenderTarget>, Vec<RenderTask>>;
}