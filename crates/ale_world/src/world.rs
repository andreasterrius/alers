use std::any::Any;
use traitcast_core::{ImplEntry, Registry, TraitcastFrom};
use ale_data::alevec::{AleVec, Key};
use crate::components::{Camera, OnSpawn, Render, Tick};
use crate::engine::Engine;
use crate::event::EventQueue;
use crate::registry::{EntryBuilder, Traitcast};
use crate::viewport::ViewportDescriptor;

type EntityKey = Key<Box<dyn Any>>;

pub struct World {
  registry: Registry,

  // Owning pointer
  entities: AleVec<Box<dyn Any>>,

  // TODO: remember which entity has which index for deletion (unordered_set)
  // Weak pointers
  ticks: Vec<*mut dyn Tick>,
  renders: Vec<*mut dyn Render>,
  cameras: Vec<*mut dyn Camera>,
  // input: Vec<*mut dyn Input>,

  event_queue: EventQueue,
}

impl World {
  pub fn new() -> World {
    World {
      entities: AleVec::new(),
      ticks: vec!(),
      renders: vec!(),
      cameras: vec!(),
      registry: Registry::new(),
      event_queue: EventQueue::new(),
    }
  }

  pub fn spawn<T: 'static>(&mut self, mut entity: T) -> EntityKey {
    // Register the traits this thing have
    World::save_component(&self.registry, &mut self.ticks, &mut entity);
    World::save_component(&self.registry, &mut self.renders, &mut entity);
    World::save_component(&self.registry, &mut self.cameras, &mut entity);

    // Get ownership of pointer, save it to entities
    let b = Box::new(entity);
    let key = self.entities.push(b);

    let ent = self.entities.get_mut(key).unwrap();
    if let Some(comp) = World::get::<dyn Key>(&self.registry, ent) {
      comp.id(key);
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
      Some(t) => { v.push(t as *mut T) }
    }
  }

  fn get<'a, T: ?Sized + 'static>(registry: &'a Registry, entity: &'a mut dyn Any) -> Option<&'a mut T> {
    let item: Option<&mut T> = entity.cast_mut(registry);
    item
  }

  pub fn fixed_tick(&mut self, delta_time: f32) {
    for t in &self.ticks {
      unsafe { (**t).fixed_tick(delta_time); }
    }
  }

  pub fn tick(&mut self, engine: &mut Engine, delta_time: f32) {
    for t in &self.ticks {
      unsafe { (**t).tick(delta_time); }
    }
  }

  pub fn render(&mut self, window_id: u32) {
    for t in &self.renders {
      unsafe { (**t).render(); }
    }
  }
}