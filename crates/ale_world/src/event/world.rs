use crate::components::Spawnable;
use ale_data::indexmap::Id;
use std::any::{Any, TypeId};
use ale_data::entity::Entity;

pub enum WorldCommand {
  Spawn(SpawnCommand),
  Kill(KillCommand),
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
      entity_key
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
