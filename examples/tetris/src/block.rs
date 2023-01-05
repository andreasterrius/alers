use ale_data::indexmap::Id;
use ale_opengl::renderer::task::Task;
use ale_world::components::{Renderable, Spawnable, Tickable};
use ale_world::wire_component;
use ale_world::world::{Entity, World};

pub struct Block {
  id : Id<Entity>
}

impl Block {
  pub fn register_components(world: &mut World) {
    world.register_components(&[
      wire_component!(dyn Spawnable, Block),
      wire_component!(dyn Tickable, Block),
      wire_component!(dyn Renderable, Block),
    ]);
  }

  pub fn new() -> Block {
    Block{
      id: Id::new()
    }
  }
}

impl Tickable for Block {
  fn fixed_tick(&mut self, delta_time: f32) {
  }

  fn tick(&mut self, delta_time: f32) {

  }
}

impl Spawnable for Block {
  fn on_spawn(&mut self) {}

  fn on_kill(&mut self) {}

  fn id(&self) -> Id<Entity> {
    self.id
  }
}

impl Renderable for Block {
  fn get_render_task(&mut self) -> Task {
    //Task::StaticMesh()
    todo!()
  }
}

/*
XXXXXXXXX
X       X
XXXX    X
   X     XXXX
   X        X
   XXXXXXXXXX

   110
   011

*/