pub use crossbeam_channel::{Receiver, Sender};

pub struct Channel<T> {
  pub sender: Sender<T>,
  pub receiver: Receiver<T>,
}

impl<T> Channel<T> {
  pub fn new() -> Channel<T> {
    let (s, r) = crossbeam_channel::unbounded();
    Channel { sender: s, receiver: r }
  }
}