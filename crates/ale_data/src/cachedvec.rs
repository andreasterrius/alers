use std::collections::HashMap;

pub trait Cacheable<Key> {
  fn to_key(&self) -> Key;
}

pub struct CachedVec<Key, T: Cacheable<Key>> {
  vec: Vec<T>,
  map: HashMap<Key, usize>,
}








