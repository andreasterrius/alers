use ale_data::alevec;
use log::info;
use std::collections::HashMap;

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

use crate::event::EngineEvent;
use crate::AppError;

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
  pub fn new(display_setting: DisplaySetting) -> Result<Engine, AppError> {
    let mut resources = Resources::new();

    let mut windows = Windows::new();
    windows.add(display_setting);

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
