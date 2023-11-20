use std::collections::HashMap;
use std::fmt::Display;

use ale_math::color::Color;
use ale_math::Vector2;
use ale_opengl::renderer::task::RenderTask;
use ale_opengl::{
  ale_opengl_blend_enable, ale_opengl_clear_render, ale_opengl_clear_render_color, ale_opengl_depth_test_enable,
};
use ale_render::target::RenderTargetType;
use ale_window::display::DisplaySetting;
use ale_window::window::Window;
use ale_world::world::World;

use crate::engine::Engine;
use crate::visitor::{CameraVisitor, FixedTickVisitor, RenderableVisitor, TickVisitor};
use crate::{init_term, AppError, FixedStep, WorldTick};

pub trait Genesis {
  fn register_components(&self, world: &mut World);

  fn window(&self) -> DisplaySetting;

  fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), AppError>;
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

  fn run_app_loop(&mut self) -> anyhow::Result<()> {
    init_term();

    let mut tick = WorldTick::FixedStep(FixedStep::new(0.01f32));
    let mut world = World::new();

    let mut display_setting = self.genesis.window();
    let mut engine = Engine::new(display_setting)?;

    self.genesis.register_components(&mut world);
    self.genesis.init(&mut engine, &mut world)?;

    ale_opengl_depth_test_enable();
    ale_opengl_blend_enable();

    while engine.windows.len() >= 1 {
      engine.windows.poll_inputs();

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

      world.resolve_world_commands();

      //tick
      let mut tick_vis = TickVisitor { delta_time };
      world.visit_mut(&mut tick_vis);

      // render
      self.render(&mut engine, &mut world);

      // cleanup
      engine.windows.cleanup();
    }

    Ok(())
  }

  // This function takes first window and first camera found.
  // Too hard to do multiple windows and multiple camera for now
  // Will implement when the use case arise.
  fn render(&mut self, engine: &mut Engine, world: &mut World) {
    let mut renderable_vis = RenderableVisitor {
      render_tasks: Vec::new(),
    };
    world.visit_mut(&mut renderable_vis);

    let mut camera_vis = CameraVisitor {
      camera_render_info: HashMap::new(),
    };
    world.visit_mut(&mut camera_vis);

    // Just take first window we found
    let mut window = match engine.windows.iter_mut().next() {
      None => {
        return;
      }
      Some(w) => w,
    };
    window.make_current();
    ale_opengl_clear_render();

    // Just take whatever first camera we found
    if let Some((_, camera)) = camera_vis.camera_render_info.iter().next() {
      for renderable in renderable_vis.render_tasks {
        match renderable {
          RenderTask::StaticMesh(_) => {}
          RenderTask::Sprite(sprite) => {
            engine
              .sprite_renderer
              .render_flat_box(sprite.position, sprite.size, sprite.color, camera.orthographic)
          }
        }
      }
    }

    window.swap_buffers();

    // TODO: this needs to be cooked more before implement
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
  }

  // for window in &mut engine.windows.iter_mut() {
  //   if window.is_hidden {
  //     continue;
  //   }
  //   window.make_current();
  //
  //   ale_opengl_clear_render();
  //   window.swap_buffers();
  // }
  // }
}
