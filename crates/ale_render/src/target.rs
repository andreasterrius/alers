use ale_data::alevec::{AleVec, Key};
use ale_data::entity::Entity;
use ale_data::indexmap::Id;
use ale_resources::texture::Texture;
use ale_ui::element::Element;
use ale_window::window::Window;

pub struct RenderTarget {
  pub camera: Id<Entity>,
  pub render_target_type: RenderTargetType,
}

pub enum RenderTargetType {
  Window(Key<Window>),
  Viewport(Key<Element>),
  Texture(Key<Texture>),
}

pub struct RenderTargets {
  render_targets: AleVec<RenderTarget>,
}

impl RenderTargets {
  pub fn new() -> RenderTargets {
    RenderTargets {
      render_targets: AleVec::new(),
    }
  }

  pub fn push(&mut self, render_target: RenderTarget) -> Key<RenderTarget> {
    self.render_targets.push(render_target)
  }

  pub fn get(&self, key: Key<RenderTarget>) -> Option<&RenderTarget> {
    self.render_targets.get(key)
  }
}
