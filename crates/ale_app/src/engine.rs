use ale_data::alevec;
use log::info;
use std::collections::HashMap;

use ale_data::alevec::AleVec;
use ale_data::channel::{Channel, Sender};
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
use ale_window_winit::backend::EventLoopWindowTarget;
use ale_window_winit::event_loop::EventLoop;

use crate::event::EngineCommand;
use crate::AppError;

pub struct Engine {
  pub windows: ale_window_winit::backend::Windows,
  pub resources: Resources,
  pub render_targets: RenderTargets,

  pub text_renderer: TextRenderer,
  pub sprite_renderer: SpriteRenderer,

  pub engine_commands: Channel<EngineCommand>,
}

impl Engine {
  pub fn new() -> Result<Engine, AppError> {
    let mut resources = Resources::new();

    // have to create first hidden window for context
    let mut windows = ale_window_winit::backend::Windows::new();

    let text_renderer = TextRenderer::new_with_resources(&mut resources)?;
    let sprite_renderer = SpriteRenderer::new_with_resource(&mut resources)?;

    Ok(Engine {
      windows,
      resources,
      render_targets: RenderTargets::new(),
      text_renderer,
      sprite_renderer,
      engine_commands: Channel::new(),
    })
  }

  pub fn get_engine_command_sender(&mut self) -> Sender<EngineCommand> {
    self.engine_commands.sender.clone()
  }

  pub fn resolve_commands(&mut self, event_loop_target : &EventLoopWindowTarget<()>) {
    let cmds: Vec<EngineCommand> = self.engine_commands.receiver.try_iter().collect();
    for cmd in cmds {
      match cmd {
        EngineCommand::CreateWindow(cwc) => self.windows.add(cwc.id,
                                                             cwc.display_setting,
                                                             event_loop_target),
      }
    }
  }

  pub fn run_loop(&mut self) {

  }
}
