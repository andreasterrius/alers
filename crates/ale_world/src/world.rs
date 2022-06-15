use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use traitcast::{Traitcast, TraitcastFrom};

pub struct World {
  // Owning pointer
  entities: Vec<*mut dyn Entity>,

  // Weak pointers
  ticks: Vec<*mut dyn Tick>,
  // renders : Vec<*mut dyn Render>,
  // input: Vec<*mut dyn Input>,
}

impl World {
  pub fn new() -> World {
    World {
      entities: vec![],
      ticks: vec![],
      // renders: vec![],
      // input: vec![]
    }
  }

  pub fn spawn<T: 'static + Entity>(&mut self, mut entity: T) {
    // Register the traits this thing has
    self.register_tick(&mut entity);


    // Get ownership of pointer, save it to entities
    let b = Box::new(entity);
    let ptr = Box::into_raw(b);
    self.entities.push(ptr);
  }

  pub fn register_tick(&mut self, entity: &mut dyn Entity) {
    let tick: Option<&mut dyn Tick> = entity.cast_mut();
    match tick {
      None => {}
      Some(t) => { self.ticks.push(t as *mut dyn Tick) }
    }
  }

  pub fn register_tick(&mut self, entity: &mut dyn Entity) {
    let tick: Option<&mut dyn Tick> = entity.cast_mut();
    match tick {
      None => {}
      Some(t) => { self.ticks.push(t as *mut dyn Tick) }
    }
  }
}

pub trait Tick {
  fn tick(&mut self, delta_time: f32);
}

pub trait Input {
  fn input(&mut self, input : ale_input::Input);
}

pub trait Entity: TraitcastFrom {}
