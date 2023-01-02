use crate::world::Entity;
use ale_data::indexmap::Key;
use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[derive(Copy, Clone)]
pub struct EventStream<T: Sync>(*mut EventStreamBuffer<T>);

impl<T: Sync> EventStream<T> {
  pub fn new(event_buffer: *mut EventStreamBuffer<T>) -> EventStream<T> {
    EventStream { 0: event_buffer }
  }

  pub fn stream(&self, reader_entity_id: Key<Entity>) -> EventStreamReader<T> {
    unsafe { (*self.0).stream(reader_entity_id) }
  }

  pub fn broadcast(&mut self, data: T) {
    unsafe { (*self.0).push(data, HashSet::new()) }
  }

  pub fn send(&mut self, data: T, target: HashSet<Key<Entity>>) {
    unsafe { (*self.0).push(data, target) }
  }
}

unsafe impl<T: Sync> Sync for EventStream<T> {}

pub struct Holder<T: Sync> {
  data: Option<T>,
  gen: usize,
  target_entity: HashSet<Key<Entity>>,
}

pub struct EventStreamBuffer<T: Sync> {
  pub inner: Vec<Holder<T>>, // this is a circular buffer
  pub head: AtomicUsize,
  pub gen: usize,
  pub size: usize,
}

unsafe impl<T: Sync> Sync for EventStreamBuffer<T> {}

impl<T: Sync> EventStreamBuffer<T> {
  pub fn new(size: usize) -> EventStreamBuffer<T> {
    let inner = Vec::with_capacity(size);

    EventStreamBuffer {
      inner,
      head: AtomicUsize::new(0),
      gen: 0,
      size,
    }
  }

  pub fn push(&mut self, data: T, target_entity: HashSet<Key<Entity>>) {
    self.inner[self.head.load(Ordering::Acquire)] = Holder {
      data: Some(data),
      gen: self.gen,
      target_entity,
    };
    self.increment_head();
  }

  fn increment_head(&mut self) {
    let new_value = self.head.fetch_add(1, Ordering::AcqRel);
    if new_value > self.size {
      self.increment_gen();
    }
    self.head.store(new_value % self.size, Ordering::Release);
  }

  fn increment_gen(&mut self) {
    self.gen = self.gen.wrapping_add(1);
  }

  fn stream(&self, reader_entity_id: Key<Entity>) -> EventStreamReader<T> {
    return EventStreamReader::new(
      self as *const EventStreamBuffer<T>,
      self.head.load(Ordering::Relaxed),
      self.gen,
      reader_entity_id,
    );
  }
}

pub struct EventStreamReader<T: Sync> {
  event_buffer: *const EventStreamBuffer<T>,
  curr: usize,
  gen: usize,
  reader_entity_id: Key<Entity>,
}

impl<T: Sync> EventStreamReader<T> {
  fn new(
    event_buffer: *const EventStreamBuffer<T>,
    curr: usize,
    gen: usize,
    reader_entity_id: Key<Entity>,
  ) -> EventStreamReader<T> {
    EventStreamReader {
      event_buffer,
      curr,
      gen,
      reader_entity_id,
    }
  }

  fn try_read(&mut self) -> Option<&T> {
    unsafe {
      // seek next valid gen
      loop {
        let holder_opt = (*self.event_buffer).inner.get(self.curr);
        let item: Option<&T> = match holder_opt {
          None => None,
          Some(holder) => {
            if holder.gen != self.gen {
              self.gen = (*self.event_buffer).gen;
              self.curr = 0;
              return None;
            }
            if holder.target_entity.is_empty() {
              // no target (broadcast)
              self.increment_index();
              return holder.data.as_ref();
            }
            if holder.target_entity.contains(&self.reader_entity_id) {
              // is one of the targets
              self.increment_index();
              return holder.data.as_ref();
            }
            self.increment_index();
            return None;
          }
        };
        if item.is_some() {
          return item;
        }
        //if self.gen == (*self.event_buffer).size && self.curr == (*self.event_buffer).head {}
      }
    }
  }

  fn increment_index(&mut self) {
    unsafe {
      let size = (*self.event_buffer).size;
      self.curr += 1;
      if self.curr >= size {
        self.curr = 0;
        self.gen = (*self.event_buffer).gen + 1;
      }
    }
  }
}

#[test]
fn thread_safety_check() {
  let event_buffer = EventStreamBuffer::<i32>::new(1000);
  let mut event = unsafe {
    let box_event_buffer = Box::new(event_buffer);
    EventStreamReader::new(&mut *box_event_buffer)
  };

  for i in 0..1000 {
    event.broadcast(i);
  }

  let mut handles = vec![];
  for _ in 0..100 {
    handles.push(thread::spawn(|| {
      let mut stream = event.stream(Key::empty());
      for i in 0..1000 {
        match stream.try_read() {
          None => {}
          Some(_) => {}
        }
      }
    }));
  }
  // create 100 threads, each streaming from the event pipeline
}
