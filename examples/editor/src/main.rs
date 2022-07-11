use crate::scene::camera::EditorCamera;
use crate::ui::viewport::Viewport;
use ale_app::app::{App, Genesis};
use ale_app::engine::Engine;
use ale_math::rect::Rect;
use ale_math::{Vector2, Zero};
use ale_resources::resources::Resources;
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_world::world::World;

mod scene;
mod ui;

struct Editor;

impl Genesis for Editor {
  fn register_components(&self, world: &mut World) {
    EditorCamera::register_components(world);
  }

  fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), ale_app::AppError> {
    // Spawn entities required by a viewport
    let editor_camera_key = world.spawn(EditorCamera::new());

    // Register viewport and how to render
    let viewport = Viewport::new(engine, editor_camera_key)?;

    // Create world entities

    Ok(())
  }
}

fn main() {
  App::new(Editor).run();
}
