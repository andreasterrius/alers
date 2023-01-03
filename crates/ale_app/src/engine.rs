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
use ale_resources::resources::Resources;
use ale_ui::element::{Element, Panel};
use ale_ui::panels::Panels;
use ale_window::backend;
use ale_window::backend::Windows;
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_window::window::Window;
use ale_world::world::Entity;

use crate::AppError;
use crate::event::EngineEvent;

pub enum RenderTarget {
  Window(alevec::Key<Window>),
  Viewport(alevec::Key<Element>),
  Texture(/*Key<Texture>*/),
}

pub struct Engine {
  pub windows: Windows,
  pub panels: Panels,
  pub resources: Resources,
  pub camera_target: HashMap<Id<Entity>, RenderTarget>,

  pub text_renderer: TextRenderer,
  pub sprite_renderer: SpriteRenderer,

  pub event_queue: Channel<EngineEvent>,
}

impl Engine {
  pub fn new() -> Result<Engine, AppError> {
    let mut resources = Resources::new();

    // have to create first hidden window for context
    let mut windows = Windows::new();
    windows.add(DisplaySetting{
      dimension: Rect {
        position : Vector2::new(0,0),
        size : Vector2::new(1,1),
      },
      initial_target: TargetMonitor::PRIMARY,
      is_hidden: true
    });

    let text_renderer = TextRenderer::new_with_resources(&mut resources)?;
    let sprite_renderer = SpriteRenderer::new_with_resource(&mut resources)?;

    Ok(Engine {
      windows,
      panels: Panels::new(),
      resources,
      camera_target: HashMap::new(),
      text_renderer,
      sprite_renderer,
      event_queue: Channel::new(),
    })
  }
}
