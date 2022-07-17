use crate::components::{Camera, OnSpawn, Renderable, Tick};
use crate::event::EventQueue;
use crate::registry::{EntryBuilder, Traitcast};
use crate::visitor;
use crate::visitor::{CameraVisitor, RenderableVisitor};
use ale_data::alevec::{AleVec, Key};
use std::any::Any;
use traitcast_core::{ImplEntry, Registry, TraitcastFrom};

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

pub struct World {
  registry: Registry,

  // Owning pointer
  entities: AleVec<Box<dyn Any>>,

  // TODO: remember which entity has which index for deletion (unordered_set)
  // Weak pointers
  ticks: Vec<*mut dyn Tick>,
  renders: Vec<*mut dyn Renderable>,
  cameras: Vec<*mut dyn Camera>,
  // input: Vec<*mut dyn Input>,
  event_queue: EventQueue,
}

impl World {
  pub fn new() -> World {
    World {
      entities: AleVec::new(),
      ticks: vec![],
      renders: vec![],
      cameras: vec![],
      registry: Registry::new(),
      event_queue: EventQueue::new(),
    }
  }

  pub fn spawn<T: 'static>(&mut self, mut entity: T) -> EntityKey {
    // Get ownership of pointer, save it to entities
    let mut b = Box::new(entity);
    let key = self.entities.push(b);
    let mut ent = self.entities.get_mut(key).unwrap();

    // Register the traits this thing have
    World::save_component(&self.registry, &mut self.ticks, ent.as_any_mut());
    World::save_component(&self.registry, &mut self.renders, ent.as_any_mut());
    World::save_component(&self.registry, &mut self.cameras, ent.as_any_mut());

    let ent = self.entities.get_mut(key).unwrap();
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

  fn save_component<T: ?Sized + 'static>(registry: &Registry, v: &mut Vec<*mut T>, entity: &mut dyn Any) {
    let item: Option<&mut T> = entity.cast_mut(registry);
    match item {
      None => { /* do nothing */ }
      Some(t) => v.push(t as *mut T),
    }
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
