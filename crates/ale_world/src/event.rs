use std::collections::HashMap;
use crate::world::EntityKey;

// Empty trait
pub trait Event {}

pub struct EventQueue {
  targeted: HashMap<EntityKey, Box<dyn Event>>,
}

impl EventQueue {
  pub fn new() -> EventQueue {
    EventQueue {
      targeted: Default::default()
    }
  }
}