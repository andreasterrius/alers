use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};
use std::path::Component;
use downcast_rs::Downcast;
use traitcast_core::{Registry, TraitcastFrom};

use ale_data::alevec::AleVec;
use ale_data::indexmap::{AleIndexMap, AleIndexSet, Key};
use ale_data::queue::fast::FastQueue;

use crate::components::{Camera, EventListener, OnSpawn, Renderable, Tick};
use crate::typecast::entry::{EntryBuilder, Traitcast};
use crate::visitor;
use crate::visitor::{Visitor, VisitorMut};

pub type Entity = Box<dyn Any>;

pub struct EntityMeta {
  impl_type: TypeId,
}

pub struct World {
  // Owning pointer
  entities: AleIndexMap<Entity>,

  // Components
  registry: Registry,
  component_to_entity: HashMap<TypeId, AleIndexSet<Key<Entity>>>,
  // components to entity
  component_index: HashMap<TypeId, Vec<TypeId>>,
  //impl to components
  entities_meta: HashMap<Key<Entity>, EntityMeta>,

  // Events
  event_queue: FastQueue<EntityEvent>,
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
      event_queue: FastQueue::new(),
    }
  }

  pub fn spawn<T: 'static>(&mut self, entity: T) -> Key<Entity> {
    // Get ownership of pointer, save it to entities
    let b = Box::new(entity);
    let key = self.entities.insert(b);

    self.entities_meta.insert(key, EntityMeta {
      impl_type: TypeId::of::<T>(),
    });

    // check what components it has, then save them
    self.save_components::<T>(key);

    // trigger on spawn once
    let entity: &mut dyn Any = self.entities.get_mut(&key).unwrap().borrow_mut();
    let on_spawn: Option<&mut dyn OnSpawn> = entity.cast_mut(&self.registry);
    match on_spawn {
      None => {}
      Some(on_spawn) => on_spawn.take_key(key)
    }

    return key;
  }

  pub fn remove(&mut self, entity_key: Key<Entity>) -> Option<Entity> {

    // delete all components
    match self.entities_meta.remove(&entity_key) {
      None => {}
      Some(entity_meta) => {
        match self.component_index.get_mut(&entity_meta.impl_type) {
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
        }
      }
    };

    return self.entities.remove(&entity_key);
  }

  pub fn register_components(&mut self, e: &[EntryBuilder]) {
    for eb in e {
      (eb.insert)(&mut self.registry);
      self.component_index
        .entry(eb.struct_impl)
        .or_insert(vec!())
        .push(eb.dyn_trait);
    }
  }

  fn save_components<T: ?Sized + 'static>(&mut self, entity_key: Key<Entity>) {
    let components = match self.component_index.get(&TypeId::of::<T>()) {
      None => { return; }
      Some(components) => components
    };

    for component_type_id in components {
      self.component_to_entity
        .entry(component_type_id.clone())
        .or_insert(AleIndexSet::new())
        .insert(entity_key);
    }
  }

  pub fn visit<T: 'static>(&self, visitor: &mut dyn Visitor<T>) {
    let type_id = TypeId::of::<T>();
    let entity_keys = match self.component_to_entity.get(&type_id) {
      None => { return; }
      Some(entity_keys) => { entity_keys }
    };

    for entity_key in entity_keys {
      let entity: &dyn Any = match self.entities.get(entity_key) {
        None => { continue; }
        Some(entity) => { entity }
      }.borrow();

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
      None => { return; }
      Some(entity_keys) => { entity_keys }
    };

    for entity_key in entity_keys {
      let entity: &mut dyn Any = match self.entities.get_mut(entity_key) {
        None => { continue; }
        Some(entity) => { entity }
      }.borrow_mut();

      let component: Option<&mut T> = entity.cast_mut(&self.registry);
      match component {
        None => {}
        Some(component) => {
          visitor.visit(component);
        }
      }
    }
  }

  pub fn resolve_events(&mut self) {
    for mut entity_event in self.event_queue.receiver.recv() {
      match entity_event.target_entity {
        None => {
          let mut event_visitor = EventVisitor { entity_event };
          self.visit_mut(&mut event_visitor);
        }
        Some(entity_key) => {
          let entity: &mut dyn Any = match self.entities.get_mut(&entity_key) {
            None => { continue; }
            Some(entity) => { entity }
          }.borrow_mut();

          let component: Option<&mut dyn EventListener> = entity.cast_mut(&self.registry);
          match component {
            None => {}
            Some(component) => {
              component.listen_event(&mut entity_event);
            }
          }
        }
      }
    }
  }
}


// marker trait
pub trait Event {}

pub struct EntityEvent {
  pub(crate) event: Box<dyn Event>,

  pub(crate) event_id: TypeId,

  // none = broadcast
  pub(crate) target_entity: Option<Key<Entity>>,
}

pub struct EventVisitor {
  entity_event: EntityEvent,
}

impl VisitorMut<dyn EventListener> for EventVisitor {
  fn visit(&mut self, component: &mut (dyn EventListener + 'static)) {
    component.listen_event(&self.entity_event)
  }
}