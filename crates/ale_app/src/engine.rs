use ale_data::alevec;
use std::collections::HashMap;
use log::info;

use ale_data::alevec::AleVec;
use ale_data::channel::Channel;
use ale_data::indexmap::Id;
use ale_math::rect::Rect;
use ale_math::{Vector2, Zero};
use ale_opengl::renderer::sprite::SpriteRenderer;
use ale_opengl::renderer::text::TextRenderer;
use ale_render::target::RenderTargets;
use ale_resources::resources::Resources;
use ale_ui::panels::Panels;
use ale_window::backend::Windows;
use ale_window::display::{DisplaySetting, TargetMonitor};


use crate::AppError;
use crate::event::EngineEvent;



pub struct Engine {
  pub windows: Windows,
  pub panels: Panels,
  pub resources: Resources,
  pub render_targets: RenderTargets,

  pub text_renderer: TextRenderer,
  pub sprite_renderer: SpriteRenderer,

  pub event_queue: Channel<EngineEvent>,
}

impl Engine {
  pub fn new() -> Result<Engine, AppError> {
    let mut resources = Resources::new();

    // have to create first hidden window for context
    let mut windows = Windows::new();

    let text_renderer = TextRenderer::new_with_resources(&mut resources)?;
    let sprite_renderer = SpriteRenderer::new_with_resource(&mut resources)?;

    Ok(Engine {
      windows,
      panels: Panels::new(),
      resources,
      render_targets: RenderTargets::new(),
      text_renderer,
      sprite_renderer,
      event_queue: Channel::new(),
    })
  }
}
