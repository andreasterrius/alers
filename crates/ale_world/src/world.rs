use std::any::Any;
use traitcast_core::{ImplEntry, Registry, TraitcastFrom};
use crate::components::{Camera, Id, Render, Tick};
use crate::registry::{EntryBuilder, Traitcast};

pub struct World {
  registry: Registry,

  // Owning pointer
  entities: Vec<Box<dyn Any>>,

  // Weak pointers
  ticks: Vec<*mut dyn Tick>,
  renders: Vec<*mut dyn Render>,
  camera: Vec<*mut dyn Camera>,
  // input: Vec<*mut dyn Input>,

  unique_id_counter: u32,

  //TODO: remember which entity has which index for deletion (unordered_set)
}

impl World {
  pub fn new() -> World {
    World {
      entities: vec![],
      ticks: vec![],
      renders: vec![],
      camera: vec![],
      registry: Registry::new(),
      unique_id_counter: 1,
      // input: vec![]
    }
  }

  pub fn spawn<T: 'static>(&mut self, mut entity: T) {
    // Register the traits this thing have
    World::save_component(&self.registry, &mut self.ticks, &mut entity);
    World::save_component(&self.registry, &mut self.renders, &mut entity);

    // Pass id if entity needs it
    if let Some(comp) = World::get_component::<dyn Id>(&self.registry, &mut entity) {
      comp.id(self.unique_id_counter);
      self.unique_id_counter += 1;
    }

    // Get ownership of pointer, save it to entities
    let b = Box::new(entity);
    self.entities.push(b);
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

  fn get_component<'a, T: ?Sized + 'static>(registry: &'a Registry, entity: &'a mut dyn Any) -> Option<&'a mut T> {
    let item: Option<&mut T> = entity.cast_mut(registry);
    item
  }

  pub fn fixed_tick(&mut self, delta_time: f32) {
    for t in &self.ticks {
      unsafe { (**t).fixed_tick(delta_time); }
    }
  }

  pub fn tick(&mut self, delta_time: f32) {
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