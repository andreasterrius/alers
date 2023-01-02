use crate::components::Spawnable;
use crate::world::Entity;
use ale_data::indexmap::Key;
use std::any::{Any, TypeId};

pub enum WorldCommand {
  Spawn(SpawnCommand),
  Kill(KillCommand),
}

pub struct SpawnCommand {
  pub(crate) type_id: TypeId,
  pub(crate) entity: Box<dyn Any>,
  pub(crate) entity_key: Key<Entity>,
}

impl SpawnCommand {
  pub fn new<T: 'static + Spawnable>(entity: T) -> SpawnCommand {
    let entity_key = entity.get_key();
    return SpawnCommand {
      type_id: TypeId::of::<T>(),
      entity: Box::new(entity),
      entity_key
    };
  }
}

pub struct KillCommand {
  pub(crate) entity_key: Key<Entity>,
}

impl KillCommand {
  pub fn new(entity_key: Key<Entity>) -> KillCommand {
    return KillCommand { entity_key };
  }
}
