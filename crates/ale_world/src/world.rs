use bus::Bus;
use downcast_rs::{impl_downcast, Downcast};
use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::collections::{HashMap, HashSet};
use std::path::Component;
use std::ptr::null_mut;
use std::thread;
use traitcast_core::{Registry, TraitcastFrom};

use crate::command::WorldCommand;
use ale_data::alevec::AleVec;
use ale_data::indexmap::{AleIndexMap, AleIndexSet, Key};
use ale_data::queue::fast::{FastQueue, Sender};

use crate::components::{Camera, Renderable, Spawnable, Tick};
use crate::event::{Event, EventBuffer};
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

  world_event_buffer: Box<dyn Any>,
  world_event: Event<WorldCommand>,
  world_event_channels :

  // Has to be declared at the bottom, ensure others are dropped first before this
  event_buffer: Option<Box<dyn Any>>,
}

impl World {
  pub fn new() -> World {
    let world_event_buffer = Box::new(EventBuffer::<WorldCommand>::new(1000));
    let world_event = Event::new(&mut *world_event_buffer);

    World {
      entities: AleIndexMap::new(),
      registry: Registry::new(),
      //event_queue: EventQueue::new(),
      component_to_entity: HashMap::new(),
      component_index: Default::default(),
      entities_meta: Default::default(),
      world_event_buffer,
      world_event,
      event_buffer: None,
    }
  }

  pub fn gen_entity_key(&self) -> Key<Entity> {
    self.entities.gen_key()
  }

  pub fn spawn<T: 'static + Spawnable>(&mut self, entity: T) {
    // Get ownership of pointer, save it to entities
    let b = Box::new(entity);
    let key = b.get_key();
    self.entities.insert_wkey(key, b);
    self.entities_meta.insert(
      key,
      EntityMeta {
        impl_type: TypeId::of::<T>(),
      },
    );

    // check what components it has, then save them
    self.save_components::<T>(key);
  }

  pub fn remove(&mut self, entity_key: Key<Entity>) -> Option<Entity> {
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

    return self.entities.remove(&entity_key);
  }

  pub fn register_components(&mut self, e: &[EntryBuilder]) {
    for eb in e {
      (eb.insert)(&mut self.registry);
      self
        .component_index
        .entry(eb.struct_impl)
        .or_insert(vec![])
        .push(eb.dyn_trait);
    }
  }

  fn save_components<T: ?Sized + 'static>(&mut self, entity_key: Key<Entity>) {
    let components = match self.component_index.get(&TypeId::of::<T>()) {
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

  pub fn get_entity_event_stream<T: Sync>(&mut self, size: usize) -> Event<T> {
    let event_buffer = EventBuffer::<T>::new(size);
    unsafe {
      let box_event_buffer = Box::new(event_buffer);
      self.event_buffer = Some(box_event_buffer);
      return Event::new(&mut *box_event_buffer);
    }
  }

  pub fn get_world_event_stream(&mut self) -> Event<WorldCommand> {
    return self.world_event;
  }
}

#[test]
fn t() {
  let mut bus = Bus::new(10);
  let mut rx1 = bus.add_rx();
  let mut rx2 = bus.add_rx();

  // start a thread that sends 1..100
  let j = thread::spawn(move || {
    for i in 1..100 {
      bus.broadcast(i);
    }
  });

  // every value should be received by both receivers
  for i in 1..100 {
    // rx1
    assert_eq!(rx1.recv(), Ok(i));
    // and rx2
    assert_eq!(rx2.recv(), Ok(i));
  }

  j.join().expect("bla");
}
