use indexmap::IndexMap;
use lazy_static::lazy_static;
use snowflake::ProcessUniqueId;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

lazy_static! {
  static ref EMPTY_UNIQUE_ID: ProcessUniqueId = ProcessUniqueId::new();
}

#[derive(Debug)]
pub struct Id<T>(ProcessUniqueId, PhantomData<T>);

impl<T> Id<T> {
  pub fn empty() -> Id<T> {
    return Id {
      0: EMPTY_UNIQUE_ID.clone(),
      1: PhantomData::default(),
    };
  }

  pub fn new() -> Id<T> {
    return Id {
      0: ProcessUniqueId::new(),
      1: PhantomData::default(),
    }
  }
}

impl<T> Clone for Id<T> {
  fn clone(&self) -> Self {
    Id {
      0: self.0,
      1: Default::default(),
    }
  }
}

impl<T> Copy for Id<T> {}

impl<T> Hash for Id<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}

impl<T> PartialEq<Self> for Id<T> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T> Eq for Id<T> {}

pub struct AleIndexMap<T> {
  inner: IndexMap<Id<T>, T>,
}

impl<T> AleIndexMap<T> {
  pub fn new() -> AleIndexMap<T> {
    AleIndexMap { inner: IndexMap::new() }
  }

  pub fn insert(&mut self, item: T) -> Id<T> {
    let key = Id(ProcessUniqueId::new(), PhantomData::default());
    self.inner.insert(key, item);
    key
  }

  pub fn gen_key(&self) -> Id<T> {
    Id(ProcessUniqueId::new(), PhantomData::default())
  }

  pub fn insert_wkey(&mut self, key: Id<T>, item: T) {
    self.inner.insert(key, item);
  }

  pub fn get_mut(&mut self, key: &Id<T>) -> Option<&mut T> {
    self.inner.get_mut(key)
  }

  pub fn get(&self, key: &Id<T>) -> Option<&T> {
    self.inner.get(key)
  }

  pub fn remove(&mut self, key: &Id<T>) -> Option<T> {
    self.inner.remove(key)
  }

  pub fn len(&self) -> usize {
    self.inner.len()
  }
}

pub type AleIndexSet<T> = indexmap::IndexSet<T>;
