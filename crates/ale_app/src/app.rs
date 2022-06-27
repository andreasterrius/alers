use std::slice::Windows;
use ale_opengl::{ale_opengl_clear_render, ale_opengl_clear_render_color};
use ale_resources::resources;
use ale_resources::resources::Resources;
use ale_window::backend;
use ale_window::window::Window;
use ale_world::engine::Engine;
use ale_world::viewport::ViewportDescriptor;
use ale_world::world::World;
use crate::{AppError, DisplaySetting, FixedStep, WorldTick};

pub trait Genesis {
  fn register_components(&self, world: &mut World);

  fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), AppError>;
}

pub struct App {}

impl App {
  pub fn run(init: &dyn Genesis) {
    App::internal_run(init).unwrap();
  }

  fn internal_run(genesis: &dyn Genesis) -> anyhow::Result<()> {
    let mut tick = WorldTick::FixedStep(FixedStep::new(0.01f32));
    let mut world = World::new();
    let mut engine = Engine::new();

    genesis.register_components(&mut world);
    genesis.init(&mut engine, &mut world)?;

    while windows.len() > 0 {
      window_backend.poll_inputs();
      for window in &mut engine.windows {
        window.input();
      }

      tick.prepare_tick();
      let delta_time = tick.delta_time();

      while tick.should_tick() {
        tick.tick();
        world.fixed_tick(tick.delta_time());
      }

      world.tick(&mut engine, delta_time);

      App::render(&mut engine, &mut world);
      App::cleanup_closed_windows(&mut windows);
    }

    Ok(())
  }

  fn render(engine: &mut Engine,
            world : &mut World) {

    for vw in &mut engine.viewport_descriptor.iter() {
      let window = engine.windows.get(vw.window_id);
      window.make_current();
      ale_opengl_clear_render();
    }

    for window in engine.windows {
      window.make_current();
      ale_opengl_clear_render();
      // render the world to each window
      world.render(window.display_setting.id);
      window.swap_buffers();
    }
  }

  fn cleanup_closed_windows(windows : &mut Vec<Window>) {
    let mut removed_index = vec!();
    for (index, window) in &mut windows.iter().enumerate() {
      if window.is_closing() {
        removed_index.push(index);
      }
    }
    for ri in removed_index {
      windows.remove(ri);
    }
  }
}