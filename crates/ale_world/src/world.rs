use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::path::Component;

use ale_data::alevec::AleVec;
use ale_data::indexmap::{AleIndexMap, AleIndexSet, Key};

use crate::components::{Camera, OnSpawn, Renderable, Tick};
use crate::event::EventQueue;
use crate::typecast::entry::{EntryBuilder, Traitcast};
use crate::typecast::registry::Registry;
use crate::visitor;
use crate::visitor::{CameraVisitor, RenderableVisitor, Visitor, VisitorMut};

pub type EntityKey = Key<Box<dyn Any>>;

macro_rules! visitor_impl {
  ($component_name: ident, $visitor_name: ident, $field_ident:ident, $fn_name:ident) => {
    pub fn $fn_name<T: $visitor_name>(&mut self, visitor: &mut T) {
      for t in &self.$field_ident {
        unsafe {
          match (*t).as_mut() {
            None => {}
            Some(ent) => visitor.visit(ent),
          };
        }
      }
    }
  };
}

pub type Entity = Box<dyn Any>;

pub struct World {
  // Owning pointer
  entities: AleIndexMap<Entity>,

  // Components
  registry: Registry,
  component_to_entity: HashMap<TypeId, AleIndexSet<Key<Entity>>>,
  component_index: HashMap<TypeId, Vec<TypeId>>,

  // Events
  event_queue: EventQueue,
}

impl World {
  pub fn new() -> World {
    World {
      entities: AleIndexMap::new(),
      registry: Registry::new(),
      event_queue: EventQueue::new(),
      component_to_entity: HashMap::new(),
      component_index: Default::default(),
    }
  }

  pub fn spawn<T: 'static>(&mut self, entity: T) -> EntityKey {
    // Get ownership of pointer, save it to entities
    let b = Box::new(entity);
    let key = self.entities.insert(b);

    self.save_components::<T>(key);

    // trigger on spawn once
    let entity = self.entities.get_mut(&key).unwrap();
    let on_spawn: Option<&mut dyn OnSpawn> = entity.cast_mut(&self.registry);
    match on_spawn {
      None => {}
      Some(on_spawn) => on_spawn.take_key(key)
    }

    return key;
  }

  pub fn register_components(&mut self, e: &[EntryBuilder]) {
    for eb in e {
      (eb.insert)(&mut self.registry);
    }
  }

  fn save_components<T: ?Sized + 'static>(&mut self, entity_key: EntityKey) {
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
      let entity = match self.entities.get(entity_key) {
        None => { continue; }
        Some(entity) => { entity }
      };

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
      let entity = match self.entities.get_mut(entity_key) {
        None => { continue; }
        Some(entity) => { entity }
      };

      let component: Option<&mut T> = entity.cast_mut(&self.registry);
      match component {
        None => {}
        Some(component) => {
          visitor.visit(component);
        }
      }
    }
  }
}
