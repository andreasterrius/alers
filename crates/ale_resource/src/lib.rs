use ale_autoid::{Identifiable, ProcessUniqueId};
use ale_error::PassedError;
use parking_lot::lock_api::RawRwLock;
use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs::File;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::sync::Arc;

// #[derive(Debug, Hash, Eq, PartialEq)]
// struct ResourceKey {
//   pub type_id: TypeId,
//   pub uid: ProcessUniqueId,
// }
//
// impl ResourceKey {
//   pub fn new<T: 'static + Identifiable>(t: &T) -> ResourceKey {
//     ResourceKey {
//       type_id: TypeId::of::<T>(),
//       uid: t.uid().into(),
//     }
//   }
// }

#[derive(Debug)]
pub struct ResourcePile {
  resources: HashMap<TypeId, HashMap<ProcessUniqueId, Arc<RwLock<dyn Any>>>>,

  base_path: String,
}

impl ResourcePile {
  pub fn new() -> ResourcePile {
    let home_path = Path::new(env!("CARGO_MANIFEST_DIR"))
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .to_str()
      .unwrap()
      .to_owned();

    ResourcePile {
      resources: Default::default(),
      base_path: format!("{}/resources", home_path),
    }
  }

  pub fn register<T: 'static + Identifiable>(&mut self, t: T) -> Resource<T> {
    //let key = ResourceKey::new(&t);
    let key = t.uid().into();
    let arc = Arc::new(RwLock::new(t));

    //self.resources.insert(key, arc.clone());
    self
      .resources
      .entry(TypeId::of::<T>())
      .or_insert(HashMap::new())
      .insert(key, arc.clone());

    Resource::new(arc)
  }

  pub fn get_resource_path(&self, path: &str) -> String {
    format!("{}/{}", self.base_path, path)
  }
}

pub struct Resource<T> {
  arc: Arc<RwLock<dyn Any>>,

  phantom_data: PhantomData<T>,
}

impl<T> Clone for Resource<T> {
  fn clone(&self) -> Self {
    Self {
      arc: self.arc.clone(),
      phantom_data: Default::default(),
    }
  }
}

impl<T: 'static> Resource<T> {
  pub fn new(arc: Arc<RwLock<dyn Any>>) -> Resource<T> {
    Resource {
      arc: arc,
      phantom_data: Default::default(),
    }
  }

  pub fn read(&self) -> ResourceReadGuard<T> {
    ResourceReadGuard::new(self)
  }

  pub fn write(&self) -> ResourceWriteGuard<T> {
    ResourceWriteGuard::new(self)
  }
}

pub struct ResourceReadGuard<'a, T> {
  resource_rw_lock: &'a RwLock<dyn Any>,

  phantom_data: PhantomData<T>,
}

impl<'a, T> ResourceReadGuard<'a, T> {
  pub fn new(resource: &'a Resource<T>) -> ResourceReadGuard<T> {
    unsafe {
      resource.arc.raw().lock_shared();
      ResourceReadGuard {
        resource_rw_lock: &*resource.arc,
        phantom_data: Default::default(),
      }
    }
  }
}

impl<'a, T> Drop for ResourceReadGuard<'a, T> {
  fn drop(&mut self) {
    unsafe {
      self.resource_rw_lock.raw().unlock_shared();
    }
  }
}

impl<'a, T: 'static> Deref for ResourceReadGuard<'a, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    let k = unsafe { &*self.resource_rw_lock.data_ptr() };
    k.downcast_ref().unwrap()
  }
}

pub struct ResourceWriteGuard<'a, T> {
  resource_rw_lock: &'a RwLock<dyn Any>,

  phantom_data: PhantomData<T>,
}

impl<'a, T> ResourceWriteGuard<'a, T> {
  pub fn new(resource: &'a Resource<T>) -> ResourceWriteGuard<'a, T> {
    unsafe {
      resource.arc.raw().lock_exclusive();
    }
    ResourceWriteGuard {
      resource_rw_lock: &resource.arc,
      phantom_data: Default::default(),
    }
  }
}

impl<'a, T> Drop for ResourceWriteGuard<'a, T> {
  fn drop(&mut self) {
    unsafe { self.resource_rw_lock.raw().unlock_exclusive() }
  }
}

impl<'a, T: 'static> Deref for ResourceWriteGuard<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.resource_rw_lock.data_ptr() }.downcast_ref().unwrap()
  }
}

impl<'a, T: 'static> DerefMut for ResourceWriteGuard<'a, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.resource_rw_lock.data_ptr() }
      .downcast_mut()
      .unwrap()
  }
}

unsafe impl<T> Sync for Resource<T> {}
unsafe impl<T> Send for Resource<T> {}
