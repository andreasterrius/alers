use ale_opengl::{ale_opengl_clear_render, ale_opengl_clear_render_color};
use ale_resources::resources;
use ale_resources::resources::Resources;
use ale_world::world::World;
use crate::{AppError, DisplaySetting, engine, FixedStep, WorldTick};

pub trait Init {
  fn request_initial_windows(&self) -> Vec<DisplaySetting>;

  fn register_components(&self, world: &mut World);

  fn init(&self, resources: &mut Resources, world: &mut World) -> Result<(), AppError>;
}

pub struct App {}

impl App {
  pub fn run(init: &dyn Init) {
    App::internal_run(init).unwrap();
  }

  fn internal_run(init: &dyn Init) -> anyhow::Result<()> {
    let mut engine = engine::Engine::new();

    let mut windows = vec!();
    for window_settings in init.request_initial_windows() {
      windows.push(engine.windows().new(window_settings))
    }

    let mut resources = Resources::new();
    let mut world = World::new();

    init.register_components(&mut world);
    init.init(&mut resources, &mut world)?;

    let mut tick = WorldTick::FixedStep(FixedStep::new(0.01f32));

    while windows.len() > 0 {
      engine.poll_inputs();
      for window in &mut windows {
        window.input();
      }

      tick.prepare_tick();
      let delta_time = tick.delta_time();

      while tick.should_tick() {
        tick.tick();
        world.fixed_tick(tick.delta_time());
      }

      world.tick(delta_time);

      for window in &mut windows {
        window.make_current();
        ale_opengl_clear_render();
        // render the world to each window
        world.render(window.display_setting.id);
        window.swap_buffers();
      }

      // Cleanup closed window
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

    Ok(())
  }
}