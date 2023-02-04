use std::collections::HashMap;

use log::info;

use ale_data::channel::{Channel, Receiver, Sender};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::{Vector2, Zero};
use ale_opengl::{
  ale_opengl_blend_enable, ale_opengl_clear_render, ale_opengl_clear_render_color, ale_opengl_depth_test_enable, raw,
};
use ale_opengl::renderer::task::RenderTask;
use ale_render::target::RenderTargetType;
use ale_resources::resources::Resources;
use ale_wgpu::backend::Graphics;
use ale_wgpu::sprite_renderer::SpriteRenderPipeline;
use ale_window_winit::backend::{EventLoopWindowTarget, Windows};
use ale_window_winit::event_loop::EventLoop;
use ale_window_winit::window::Window;
use ale_world::event::world::WorldCommand;
use ale_world::world::World;

use crate::{AppError, FixedStep, init_term, WorldTick};
use crate::command::AppCommand;
use crate::visitor::{CameraVisitor, FixedTickVisitor, RenderableVisitor, TickVisitor};

pub trait Genesis {
  fn init(&self,
          app_commands: Sender<AppCommand>,
          world_commands: Sender<WorldCommand>,
  ) -> Result<(), AppError>;
}

pub struct App {
  genesis: Box<dyn Genesis>,
}

impl App {
  pub fn run<T: Genesis + 'static>(init: T) {
    let app = App { genesis: Box::new(init) };
    app.run_app_loop().unwrap();
  }

  fn run_app_loop(mut self) -> anyhow::Result<()> {
    init_term();

    let mut tick = WorldTick::FixedStep(FixedStep::new(0.01f32));
    let mut world = World::new();
    let mut resources = Resources::new();
    let mut graphics = Graphics::new();
    let mut event_loop = EventLoop::new();
    let mut windows = ale_window_winit::backend::Windows::new();
    let mut app_channel = Channel::<AppCommand>::new();

    let world_commands = world.get_world_command_sender();
    let app_commands = app_channel.sender;
    let mut app_commands_receiver = app_channel.receiver.clone();

    self.genesis.init(app_commands.clone(), world_commands.clone())?;

    self.resolve_commands(&mut windows, &mut graphics, &mut app_commands_receiver, &event_loop.0);

    // Renderers
    let mut sprite_renderer = SpriteRenderPipeline::new(&mut graphics, &mut resources)?;

    event_loop.run(move |window_target| {
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

      self.resolve_commands(&mut windows, &mut graphics, &mut app_commands_receiver, window_target);
      world.resolve_commands();

      //tick
      let mut tick_vis = TickVisitor { delta_time };
      world.visit_mut(&mut tick_vis);

      // render
      self.render(&mut graphics, &mut world, &mut sprite_renderer);
    });

    Ok(())
  }

  fn resolve_commands(&mut self,
                      windows: &mut Windows,
                      graphics: &mut Graphics,
                      app_commands: &mut Receiver<AppCommand>,
                      event_loop_target: &EventLoopWindowTarget<()>) {
    let cmds: Vec<AppCommand> = app_commands.try_iter().collect();
    for cmd in cmds {
      match cmd {
        AppCommand::CreateWindow(cwc) => {
          windows.add(cwc.id, cwc.display_setting.clone(), event_loop_target);
          let w = windows.get(cwc.id).unwrap();
          let inner_size = w.winit_window.inner_size();
          let s = graphics.create_surface(w, Rect{
            position: Vector2::zero(),
            size: Vector2::new(inner_size.width, inner_size.height)
          });
        }
      }
    }
  }

  fn render(&mut self,
            graphics: &mut Graphics,
            world: &mut World,
            sprite_renderer: &mut SpriteRenderPipeline) {
    let mut renderable_vis = RenderableVisitor {
      render_tasks: HashMap::new(),
    };
    world.visit_mut(&mut renderable_vis);

    let mut camera_vis = CameraVisitor {
      camera_render_info: HashMap::new(),
    };
    world.visit_mut(&mut camera_vis);

    graphics.execute(|view, encoder| {
      sprite_renderer.render(view, encoder);
    });

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
