use crate::components::Spawnable;
use ale_data::entity::entry::ComponentEntry;
use ale_data::entity::Entity;
use ale_data::indexmap::Id;
use std::any::{Any, TypeId};

pub enum WorldCommand {
  Spawn(SpawnCommand),
  Kill(KillCommand),

  RegisterComponent(RegisterComponentCommand),
}

pub struct SpawnCommand {
  pub(crate) type_id: TypeId,
  pub(crate) entity: Box<dyn Any>,
  pub(crate) entity_key: Id<Entity>,
}

impl SpawnCommand {
  pub fn new<T: 'static + Spawnable>(entity: T) -> SpawnCommand {
    let entity_key = entity.id();
    return SpawnCommand {
      type_id: TypeId::of::<T>(),
      entity: Box::new(entity),
      entity_key,
    };
  }
}

pub struct KillCommand {
  pub(crate) entity_key: Id<Entity>,
}

impl KillCommand {
  pub fn new(entity_key: Id<Entity>) -> KillCommand {
    return KillCommand { entity_key };
  }
}

pub struct RegisterComponentCommand {
  pub(crate) component_entries: Vec<ComponentEntry>,
}

impl RegisterComponentCommand {
  pub fn new(component_entries: Vec<ComponentEntry>) -> RegisterComponentCommand {
    RegisterComponentCommand { component_entries }
  }
}
