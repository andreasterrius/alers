use ale_data::channel::Sender;
use ale_math::color::Color;
use ale_math::Vector2;
use ale_opengl::renderer::task::RenderTask;
use ale_opengl::{
  ale_opengl_blend_enable, ale_opengl_clear_render, ale_opengl_clear_render_color, ale_opengl_depth_test_enable, raw,
};
use ale_render::target::RenderTargetType;
use ale_world::event::world::WorldCommand;
use ale_world::world::World;
use log::info;
use std::collections::HashMap;
use ale_window_winit::event_loop::EventLoop;

use crate::engine::Engine;
use crate::event::EngineCommand;
use crate::visitor::{CameraVisitor, FixedTickVisitor, RenderableVisitor, TickVisitor};
use crate::{init_term, AppError, FixedStep, WorldTick};

pub trait Genesis {
  fn init(&self, engine: Sender<EngineCommand>, world: Sender<WorldCommand>) -> Result<(), AppError>;
}

pub struct App {
  genesis: Box<dyn Genesis>,
}

impl App {
  pub fn new<T: Genesis + 'static>(init: T) -> App {
    App {
      genesis: Box::new(init),
    }
  }

  pub fn run(mut self) {
    self.run_app_loop().unwrap();
  }

  fn run_app_loop(mut self) -> anyhow::Result<()> {
    init_term();

    let mut tick = WorldTick::FixedStep(FixedStep::new(0.01f32));
    let mut world = World::new();
    let mut engine = Engine::new()?;
    let mut event_loop = EventLoop::new();

    let world_commands = world.get_world_command_sender();
    let engine_commands = engine.get_engine_command_sender();

    self.genesis.init(engine_commands.clone(), world_commands.clone())?;

    event_loop.run(move |window_target|{
      tick.prepare_tick();
      let delta_time = tick.delta_time();

      while tick.should_tick() {
        tick.tick();

        // fixed tick
        let mut fixed_tick_vis = FixedTickVisitor {
          delta_time: tick.delta_time(),
        };
        world.visit_mut(&mut fixed_tick_vis);
      }

      engine.resolve_commands(window_target);
      world.resolve_commands();

      //tick
      let mut tick_vis = TickVisitor { delta_time };
      world.visit_mut(&mut tick_vis);

      // render
      self.render(&mut engine, &mut world);
    });

    Ok(())
  }

  fn render(&mut self, engine: &mut Engine, world: &mut World) {
    // let mut renderable_vis = RenderableVisitor {
    //   render_tasks: HashMap::new(),
    // };
    // world.visit_mut(&mut renderable_vis);
    //
    // let mut camera_vis = CameraVisitor {
    //   camera_render_info: HashMap::new(),
    // };
    // world.visit_mut(&mut camera_vis);
    //
    // // TODO: Group by render target in here instead
    // for (render_target_key, renderables) in renderable_vis.render_tasks {
    //   let render_target = engine.render_targets.get(render_target_key);
    //
    //   if let Some(render_target) = render_target {
    //     let camera = camera_vis.camera_render_info.get(&render_target.camera);
    //     if camera.is_none() {
    //       continue;
    //     }
    //     let camera = camera.unwrap();
    //
    //     // TODO: How to handle multiple UI ?
    //     match render_target.render_target_type {
    //       RenderTargetType::Window(window) => {
    //         let window = engine.windows.get_mut(window);
    //         if window.is_none() {
    //           continue;
    //         }
    //         let mut window = window.unwrap();
    //         window.make_current();
    //         ale_opengl_clear_render_color(Color::light_blue());
    //
    //         // TODO: remove this test
    //         engine.sprite_renderer.render_flat_box(
    //           Vector2::new(0.0, 0.0),
    //           Vector2::new(1000.0, 1000.0),
    //           Color::from_rgba(1.0, 1.0, 1.0, 0.5),
    //           camera.orthographic,
    //         );
    //
    //         // Proof of concept first
    //         for renderable in renderables {
    //           match renderable {
    //             RenderTask::StaticMesh(_) => {}
    //             RenderTask::Sprite(sprite) => engine.sprite_renderer.render_flat_box(
    //               sprite.position,
    //               sprite.size,
    //               sprite.color,
    //               camera.orthographic,
    //             ),
    //           }
    //         }
    //
    //         window.swap_buffers();
    //       }
    //       RenderTargetType::Viewport(_) => {}
    //       RenderTargetType::Texture(_) => {}
    //     }
    //   }
    // }

    // for window in &mut engine.windows.iter_mut() {
    //   if window.is_hidden {
    //     continue;
    //   }
    //   window.make_current();
    //
    //   ale_opengl_clear_render();
    //   window.swap_buffers();
    // }
  }
}
