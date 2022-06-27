use ale_app::app::{App, Genesis};
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_math::{Vector2, Zero};
use ale_math::rect::Rect;
use ale_resources::resources::Resources;
use ale_world::engine::Engine;
use ale_world::viewport::ViewportDescriptor;
use ale_world::world::World;
use crate::scene::camera::EditorCamera;
use crate::ui::main_frame::MainFrame;

mod ui;
mod scene;

struct Editor;

impl Genesis for Editor {
  fn register_components(&self, world: &mut World) {
    MainFrame::register_components(world);
    EditorCamera::register_components(world);
  }

  fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), ale_app::AppError> {
    let main_window_key = engine.windows.add(
      DisplaySetting {
        dimension: Rect {
          position: Vector2::zero(),
          size: Vector2::new(800, 600),
        },
        initial_target: TargetMonitor::PRIMARY,
      });
    let sub_window_key = engine.windows.add(DisplaySetting {
      dimension: Rect {
        position: Vector2::zero(),
        size: Vector2::new(400, 300),
      },
      initial_target: TargetMonitor::PRIMARY,
    });

    engine.windows.insert(window_settings.id, window_backend.windows().new(window_settings));

    // Spawn entities
    let editor_camera = EditorCamera::new();
    let editor_camera_key = world.spawn(editor_camera);

    world.spawn(MainFrame::new(engine, editor_camera_key));


    Ok(())
  }
}

fn main() {
  App::run(&mut Editor);
}
