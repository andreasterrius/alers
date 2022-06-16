use std::any::Any;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::rc::Rc;
use traitcast_core::{impl_entry, ImplEntry, Registry, TraitcastFrom};
use crate::registry::{EntryBuilder, Traitcast};

pub struct World {
  registry: Registry,

  // Owning pointer
  entities: Vec<*mut dyn Any>,

  // Weak pointers
  ticks: Vec<*mut dyn Tick>,
  // renders : Vec<*mut dyn Render>,
  // input: Vec<*mut dyn Input>,

  //TODO: remember which entity has which index for deletion (unordered_set)
}

impl World {
  pub fn new() -> World {
    World {
      entities: vec![],
      ticks: vec![],
      registry: Registry::new(),
      // renders: vec![],
      // input: vec![]
    }
  }

  pub fn spawn<T: 'static>(&mut self, mut entity: T) {
    // Register the traits this thing have
    World::save_component(&self.registry, &mut self.ticks, &mut entity);

    // Get ownership of pointer, save it to entities
    let b = Box::new(entity);
    let ptr = Box::into_raw(b);
    self.entities.push(ptr);

    for t in &self.ticks {
      unsafe{ (**t).tick(1.0); }
    }
  }

  pub fn enable(&mut self, e: EntryBuilder) {
    (e.insert)(&mut self.registry);
  }

  pub fn enable_many(&mut self, e: &[EntryBuilder]){
    for eb in e {
      (eb.insert)(&mut self.registry);
    }
  }

  fn save_component<T: ?Sized + 'static>(registry: &Registry, v: &mut Vec<*mut T>, entity: &mut dyn Any) {
    let item: Option<&mut T> = entity.cast_mut(registry);
    match item {
      None => { panic!("asd") }
      Some(t) => { v.push(t as *mut T) }
    }
  }
}

pub trait Tick: TraitcastFrom {
  fn tick(&mut self, delta_time: f32);
}

pub trait Input: TraitcastFrom {
  fn input(&mut self, input: ale_input::Input);
}


