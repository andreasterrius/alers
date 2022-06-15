use ale_resources::resources;
use ale_resources::resources::Resources;
use ale_world::world::World;
use crate::{AppError, DisplaySetting, engine, FixedStep, WorldTick};

pub trait Init {
  fn request_windows(&self) -> Vec<DisplaySetting>;

  fn init(&self, resources: &mut Resources, world: &mut World) -> Result<(), AppError>;
}

pub struct App {

}

impl App {
  pub fn run(init : &dyn Init)  {
    App::internal_run(init).unwrap();
  }

  fn internal_run(init : &dyn Init) -> anyhow::Result<()> {
    let mut engine = engine::Engine::new();

    let mut windows = vec!();
    for window_settings in init.request_windows() {
      windows.push(engine.windows().new(window_settings))
    }

    let mut resources = Resources::new();
    let mut world = World::new();

    init.init(&mut resources, &mut world)?;

    let mut tick = WorldTick::FixedStep(FixedStep::new(0.01f32));
    let mut should_run = true;

    // while should_run {
    //   engine.poll_inputs();
    //   for window in &mut windows {
    //     window.input();
    //   }
    //
    //   tick.prepare_tick();
    //   while tick.should_tick() {
    //     tick.tick();
    //     // tick the world with delta_time
    //     tick.delta_time();
    //   }
    //
    //   for window in &mut windows {
    //     // render the world to each window
    //     window.swap_buffers();
    //   }
    // }

    // Main Game Loop
    // while !window.is_closing() {
    //   engine.poll_inputs();
    //   app.input(&mut state, window.input());
    //
    //   tick.prepare_tick();
    //   while tick.should_tick() {
    //     tick.tick();
    //     app.fixed_tick(&mut state, tick.delta_time());
    //   }
    //
    //   app.tick(&mut state);
    //   app.render(&mut state);
    //
    //   window.swap_buffers();
    // }
    Ok(())
  }
}


/*
struct SomeApp;
impl Init for SomeApp{
   fn request_windows() -> Vec<DisplaySetting> {
     return vec![]
   };

   fn init(resources: &mut Resources, world: &mut World) {
      // load meshes here

      // create_entities with world
   }
}

fn main(){
  App::run(SomeApp)
}
 */