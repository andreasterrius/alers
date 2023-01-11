use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;

use ale_data::channel::{Channel, Sender};
use ale_data::entity::{Entity, Registry};
use ale_data::entity::entry::{ComponentEntry, Traitcast};

use ale_data::indexmap::{AleIndexMap, AleIndexSet, Id};

use crate::components::Spawnable;
use crate::event::world::{KillCommand, SpawnCommand, WorldCommand};
use crate::visitor::{Visitor, VisitorMut};

pub struct EntityMeta {
  impl_type: TypeId,
}

pub struct World {
  // Owning pointer
  entities: AleIndexMap<Entity>,

  // Components
  registry: Registry,
  component_to_entity: HashMap<TypeId, AleIndexSet<Id<Entity>>>,
  // components to entity
  component_index: HashMap<TypeId, Vec<TypeId>>,
  //impl to components
  entities_meta: HashMap<Id<Entity>, EntityMeta>,

  // Channels
  channel: Channel<WorldCommand>,
}

impl World {
  pub fn new() -> World {
    World {
      entities: AleIndexMap::new(),
      registry: Registry::new(),
      //event_queue: EventQueue::new(),
      component_to_entity: HashMap::new(),
      component_index: Default::default(),
      entities_meta: Default::default(),
      channel: Channel::new(),
    }
  }

  pub fn gen_entity_key(&self) -> Id<Entity> {
    self.entities.gen_key()
  }

  pub fn spawn(&mut self, mut spawn_cmd: SpawnCommand) {
    // Get ownership of pointer, save it to entities
    let b = spawn_cmd.entity;
    let entity_key = spawn_cmd.entity_key;
    self.entities.insert_wkey(entity_key, b);
    self.entities_meta.insert(
      entity_key,
      EntityMeta {
        impl_type: spawn_cmd.type_id,
      },
    );

    // check what components it has, then save them
    self.save_components(spawn_cmd.type_id, entity_key);

    // trigger on_spawn() once
    match self.entities.get_mut(&entity_key) {
      None => {}
      Some(box_entity) => {
        let entity : &mut dyn Any = box_entity.borrow_mut();
        let component: Option<&mut dyn Spawnable> = entity.cast_mut(&self.registry);
        match component {
          None => {
            panic!("Spawnable is not wired")
          }
          Some(component) => {
            component.on_spawn();
          }
        }
      }
    }
  }

  pub fn remove(&mut self, kill_cmd: KillCommand) -> Option<Entity> {
    let entity_key = kill_cmd.entity_key;
    // delete all components
    match self.entities_meta.remove(&entity_key) {
      None => {}
      Some(entity_meta) => match self.component_index.get_mut(&entity_meta.impl_type) {
        None => {}
        Some(components) => {
          for component in components {
            match self.component_to_entity.get_mut(component) {
              None => {}
              Some(entities_set) => {
                entities_set.remove(&entity_key);
              }
            }
          }
        }
      },
    };

    // trigger on_kill() once
    match self.entities.get_mut(&entity_key) {
      None => {}
      Some(box_entity) => {
        let entity : &mut dyn Any = box_entity.borrow_mut();
        let component: Option<&mut dyn Spawnable> = entity.cast_mut(&self.registry);
        match component {
          None => {
            panic!("Spawnable is not wired")
          }
          Some(component) => {
            component.on_kill();
          }
        }
      }
    }

    return self.entities.remove(&entity_key);
  }

  pub fn register_components(&mut self, e: &[ComponentEntry]) {
    for eb in e {
      (eb.insert)(&mut self.registry);
      self
        .component_index
        .entry(eb.struct_impl)
        .or_insert(vec![])
        .push(eb.dyn_trait);
    }
  }

  fn save_components(&mut self, type_id: TypeId, entity_key: Id<Entity>) {
    let components = match self.component_index.get(&type_id) {
      None => {
        return;
      }
      Some(components) => components,
    };

    for component_type_id in components {
      self
        .component_to_entity
        .entry(component_type_id.clone())
        .or_insert(AleIndexSet::new())
        .insert(entity_key);
    }
  }

  pub fn visit<T: 'static>(&self, visitor: &mut dyn Visitor<T>) {
    let type_id = TypeId::of::<T>();
    let entity_keys = match self.component_to_entity.get(&type_id) {
      None => {
        return;
      }
      Some(entity_keys) => entity_keys,
    };

    for entity_key in entity_keys {
      let entity: &dyn Any = match self.entities.get(entity_key) {
        None => {
          continue;
        }
        Some(entity) => entity,
      }
      .borrow();

      let component: Option<&T> = entity.cast_ref(&self.registry);
      match component {
        None => {}
        Some(component) => {
          visitor.visit(component);
        }
      }
    }
  }

  pub fn visit_mut<T: ?Sized + 'static>(&mut self, visitor: &mut dyn VisitorMut<T>) {
    let type_id = TypeId::of::<T>();
    let entity_keys = match self.component_to_entity.get(&type_id) {
      None => {
        return;
      }
      Some(entity_keys) => entity_keys,
    };

    for entity_key in entity_keys {
      let entity: &mut dyn Any = match self.entities.get_mut(entity_key) {
        None => {
          continue;
        }
        Some(entity) => entity,
      }
      .borrow_mut();

      let component: Option<&mut T> = entity.cast_mut(&self.registry);
      match component {
        None => {}
        Some(component) => {
          visitor.visit(component);
        }
      }
    }
  }

  pub fn resolve_commands(&mut self) {
    let cmds : Vec<WorldCommand> = self.channel.receiver.try_iter().collect();
    for cmd in cmds {
      match cmd {
        WorldCommand::Spawn(se) => self.spawn(se),
        WorldCommand::Kill(ke) => {
          let _ = self.remove(ke);
        },
        WorldCommand::RegisterComponent(rc) => {
          self.register_components(&rc.component_entries)
        }
      }
    }
  }

  pub fn get_world_command_sender(&self) -> Sender<WorldCommand> {
    return self.channel.sender.clone();
  }
}
