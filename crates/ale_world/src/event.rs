use std::collections::HashMap;
use ale_data::indexmap::Key;
use crate::world::Entity;

// Empty trait
pub trait Event {}

pub struct EventQueue {
  targeted: HashMap<Key<Entity>, Box<dyn Event>>,
}

impl EventQueue {
  pub fn new() -> EventQueue {
    EventQueue {
      targeted: Default::default()
    }
  }
}