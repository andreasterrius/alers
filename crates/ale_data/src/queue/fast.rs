use std::marker::PhantomData;
pub use crossbeam_channel::{Receiver, Sender};

pub struct FastQueue<T> {
  pub sender: Sender<T>,
  pub receiver: Receiver<T>,
}

impl<T> FastQueue<T> {
  pub fn new() -> FastQueue<T> {
    let (s, r) = crossbeam_channel::unbounded();
    FastQueue { sender: s, receiver: r }
  }
}