use log::info;
use crate::scene::camera::EditorCamera;
use crate::ui::viewport::Viewport;
use ale_app::app::{App, Genesis};
use ale_app::engine::Render;
use ale_math::rect::Rect;
use ale_math::{Vector2, Zero};
use ale_resources::resources::Resources;
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_world::event::world::SpawnCommand;
use ale_world::world::World;

mod scene;
mod ui;

struct Editor;

// impl Genesis for Editor {
//   fn register_components(&self, world: &mut World) {
//     EditorCamera::register_components(world);
//   }
//
//   fn init(&self, engine: &mut Engine, world: &mut World) -> Result<(), ale_app::AppError> {
//
//     // Register window and UI panels
//     let viewport = Viewport::new(engine)?;
//
//     // Create entities
//     let editor_camera = EditorCamera::new(world.gen_entity_key());
//
//     world.spawn(SpawnCommand::new(editor_camera));
//
//     Ok(())
//   }
// }

fn main() {
 // App::new(Editor).run();
}
