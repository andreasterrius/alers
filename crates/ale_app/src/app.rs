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

    while engine.windows.len() > 0 {
      engine.windows.poll_inputs();

      tick.prepare_tick();
      let delta_time = tick.delta_time();

      while tick.should_tick() {
        tick.tick();
        world.fixed_tick(tick.delta_time());
      }

      world.tick(&mut engine, delta_time);
      App::render(&mut engine, &mut world);

      engine.windows.cleanup();
    }

    Ok(())
  }

  fn render(engine: &mut Engine,
            world: &mut World) {
    for vw in &mut engine.viewport_descriptor.iter() {
      let window = engine.windows.get_mut(vw.window_key);
      if window.is_none() {
        return;
      }
      let window = window.unwrap();

      engine.panels.render();

      window.make_current();
      ale_opengl_clear_render();
      window.swap_buffers();
    }

    // for window in engine.windows {
    //   window.make_current();
    //   ale_opengl_clear_render();
    //   // render the world to each window
    //   world.render(window.display_setting.id);
    //   window.swap_buffers();
    // }
  }
}