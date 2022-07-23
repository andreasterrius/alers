use crate::components::{Camera, Components, OnSpawn, Renderable, Tick};
use crate::event::EventQueue;
use crate::typecast::entry::{EntryBuilder, Traitcast};
use crate::visitor;
use crate::visitor::{CameraVisitor, RenderableVisitor};
use ale_data::alevec::{AleVec};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use traitcast_core::{ImplEntry, Registry, TraitcastFrom};
use ale_data::indexmap::{Key, AleIndexMap};

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

type Entity = Box<dyn Any>;

pub struct World {
  registry: Registry,

  // Owning pointer
  entities: AleIndexMap<Entity>,

  // Weak pointers
  ticks: Vec<*mut dyn Tick>,
  renders: Vec<*mut dyn Renderable>,
  cameras: Vec<*mut dyn Camera>,

  components : Components,

  // input: Vec<*mut dyn Input>,
  event_queue: EventQueue,
}

impl World {
  pub fn new() -> World {
    World {
      entities: AleIndexMap::new(),
      ticks: vec![],
      renders: vec![],
      cameras: vec![],
      registry: Registry::new(),
      event_queue: EventQueue::new(),
      components: HashMap::new(),
    }
  }

  pub fn spawn<T: 'static>(&mut self, entity: T) -> EntityKey {
    // Get ownership of pointer, save it to entities
    let b = Box::new(entity);
    let key = self.entities.insert(b);
    let ent = self.entities.get_mut(&key).unwrap();

    // // Register the traits this thing have
    // World::save_component(&self.registry, &mut self.ticks, &mut b.as_mut());
    // World::save_component(&self.registry, &mut self.renders, ent.as_any_mut());
    // World::save_component(&self.registry, &mut self.cameras, ent.as_any_mut());
    self.save_component<>(key);

    let ent = self.entities.get_mut(&key).unwrap();
    if let Some(comp) = World::get::<dyn OnSpawn>(&self.registry, ent) {
      comp.take_key(key);
    }

    return key;
  }

  pub fn enable(&mut self, e: &[EntryBuilder]) {
    for eb in e {
      (eb.insert)(&mut self.registry);
    }
  }

  fn save_component<T: ?Sized + 'static>(&mut self, entity_key: Key<Entity>) {
    let entity = match self.entities.get_mut(&entity_key) {
      None => { return; }
      Some(ent) => { ent }
    };

    let component: Option<&mut T> = entity.cast(&self.registry);
    match component {
      None => { return; }
      Some(t) => t,
    }
    let type_id = TypeId::of::<T>();
    let components = self.components.entry(type_id).or_insert(vec!());
    components.push(entity_key);
  }

  fn get<'a, T: ?Sized + 'static>(registry: &'a Registry, entity: &'a mut dyn Any) -> Option<&'a mut T> {
    let item: Option<&mut T> = entity.cast_mut(registry);
    item
  }

  pub fn fixed_tick(&mut self, delta_time: f32) {
    for t in &self.ticks {
      unsafe {
        (**t).fixed_tick(delta_time);
      }
    }
  }

  pub fn tick(&mut self, delta_time: f32) {
    for t in &self.ticks {
      unsafe {
        (**t).tick(delta_time);
      }
    }
  }

  visitor_impl!(Renderable, RenderableVisitor, renders, visit_renderables);
  visitor_impl!(Camera, CameraVisitor, cameras, visit_cameras);
}
