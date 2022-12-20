use indexmap::IndexMap;
use lazy_static::lazy_static;
use snowflake::ProcessUniqueId;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

lazy_static! {
  static ref EMPTY_UNIQUE_ID: ProcessUniqueId = ProcessUniqueId::new();
}

#[derive(Debug)]
pub struct Key<T>(ProcessUniqueId, PhantomData<T>);

impl<T> Key<T> {
  pub fn empty() -> Key<T> {
    return Key {
      0: EMPTY_UNIQUE_ID.clone(),
      1: PhantomData::default(),
    };
  }
}

impl<T> Clone for Key<T> {
  fn clone(&self) -> Self {
    Key {
      0: self.0,
      1: Default::default(),
    }
  }
}

impl<T> Copy for Key<T> {}

impl<T> Hash for Key<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}

impl<T> PartialEq<Self> for Key<T> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T> Eq for Key<T> {}

pub struct AleIndexMap<T> {
  inner: IndexMap<Key<T>, T>,
}

impl<T> AleIndexMap<T> {
  pub fn new() -> AleIndexMap<T> {
    AleIndexMap { inner: IndexMap::new() }
  }

  pub fn insert(&mut self, item: T) -> Key<T> {
    let key = Key(ProcessUniqueId::new(), PhantomData::default());
    self.inner.insert(key, item);
    key
  }

  pub fn gen_key(&self) -> Key<T> {
    Key(ProcessUniqueId::new(), PhantomData::default())
  }

  pub fn insert_wkey(&mut self, key: Key<T>, item: T) {
    self.inner.insert(key, item);
  }

  pub fn get_mut(&mut self, key: &Key<T>) -> Option<&mut T> {
    self.inner.get_mut(key)
  }

  pub fn get(&self, key: &Key<T>) -> Option<&T> {
    self.inner.get(key)
  }

  pub fn remove(&mut self, key: &Key<T>) -> Option<T> {
    self.inner.remove(key)
  }
}

pub type AleIndexSet<T> = indexmap::IndexSet<T>;
