// use crate::world::Entity;
// use ale_data::indexmap::Key;
// use bus::{Bus, BusReader};
// use std::collections::HashSet;
// use std::sync::mpsc::TryRecvError;
// use std::sync::{Arc, Mutex};
//
// // This idea doesn't work because it can we need mutex to
// // - broadcast (we can make a dispatcher)
// // - add_rx
//
// // Let's try another idea later with a thread safe CircularBuffer<T> where T: Sync
// // the implementation should use atomic integers, this should allow us to create a lock free
// // reader and writer
// pub struct Events<T: Sync + Clone> {
//   bus: Arc<Mutex<Bus<Event<T>>>>,
// }
//
// impl<T: Sync + Clone> Events<T> {
//   pub fn new() -> Events<T> {
//     Events { bus: Arc::new(Mutex::new(Bus::new(1000))) }
//   }
//
//   pub fn send(&mut self, target: HashSet<Key<Entity>>, event: T) {
//     self.bus.broadcast(Event {
//       target_keys: Arc::new(target),
//       data: event,
//     })
//   }
//
//   pub fn broadcast(&mut self, event: T) {
//     self.bus.broadcast(Event{
//       target_keys: Arc::new(HashSet::new()),
//       data: event
//     })
//   }
//
//   pub fn receiver(&mut self, key: Key<Entity>) -> Receiver<T> {
//     Receiver {
//       rx: self.bus.add_rx(),
//       key,
//     }
//   }
// }
//
// unsafe impl<T: Sync + Clone> Sync for Events<T> {}
//
// #[derive(Clone)]
// struct Event<T: Sync + Clone> {
//   pub target_keys: Arc<HashSet<Key<Entity>>>, // empty means broadcast
//   pub data: T,
// }
//
// unsafe impl<T: Sync + Clone> Sync for Event<T> {}
//
// pub struct Receiver<T: Sync + Clone> {
//   rx: BusReader<Event<T>>,
//   key: Key<Entity>,
// }
//
// impl<T: Sync + Clone> Receiver<T> {
//   fn new(rx: BusReader<Event<T>>, key: Key<Entity>) -> Receiver<T> {
//     Receiver { rx, key }
//   }
//
//   fn try_recv(&mut self) -> Option<T> {
//     return match self.rx.try_recv() {
//       Ok(event) => {
//         if event.target_keys.is_empty() {
//           return Some(event.data);
//         }
//         if event.target_keys.contains(&self.key) {
//           return Some(event.data);
//         }
//         None
//       }
//       Err(error) => match error {
//         TryRecvError::Empty => None,
//         TryRecvError::Disconnected => None,
//       },
//     };
//   }
// }
