use std::collections::HashMap;
use crate::world::EntityId;

// Empty trait
pub trait Event {}

pub struct EventQueue {
  targeted: HashMap<EntityId, Box<dyn Event>>,
}

impl EventQueue {
  pub fn new() -> EventQueue {
    EventQueue {
      targeted: Default::default()
    }
  }
}