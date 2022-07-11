use ale_data::alevec;
use std::collections::HashMap;

use ale_data::alevec::AleVec;
use ale_opengl::renderer::sprite::SpriteRenderer;
use ale_opengl::renderer::text::TextRenderer;
use ale_resources::resources::Resources;
use ale_ui::element::{Element, Panel};
use ale_ui::panels::Panels;
use ale_window::backend;
use ale_window::backend::Windows;
use ale_window::window::Window;
use ale_world::world::EntityKey;

use crate::AppError;

pub enum RenderTarget {
  Window(alevec::Key<Window>),
  Viewport(alevec::Key<Element>),
  Texture(/*Key<Texture>*/),
}

pub struct Engine {
  pub windows: Windows,
  pub panels: Panels,
  pub resources: Resources,
  pub camera_target: HashMap<EntityKey, RenderTarget>,

  pub text_renderer: TextRenderer,
  pub sprite_renderer: SpriteRenderer,
}

impl Engine {
  pub fn new() -> Result<Engine, AppError> {
    let mut resources = Resources::new();
    let text_renderer = TextRenderer::new_with_resources(&mut resources)?;
    let sprite_renderer = SpriteRenderer::new_with_resource(&mut resources)?;

    Ok(Engine {
      windows: Windows::new(),
      panels: Panels::new(),
      resources,
      camera_target: HashMap::new(),
      text_renderer,
      sprite_renderer,
    })
  }
}
