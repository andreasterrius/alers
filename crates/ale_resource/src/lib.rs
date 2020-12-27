use ale_autoid::{Identifiable, ProcessUniqueId};
use ale_error::PassedError;
use downcast_rs::impl_downcast;
use downcast_rs::{Downcast, DowncastSync};
use parking_lot::lock_api::RawRwLock;
use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::rc::{Rc, Weak};
use std::sync::Arc;

pub struct ResourcePile {
  resources: HashMap<TypeId, HashMap<ProcessUniqueId, Arc<RwLock<dyn ResourceType>>>>,

  observers: Vec<Weak<RefCell<dyn ResourcePileObserver>>>,

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
      observers: vec![],
      base_path: format!("{}/resources", home_path),
    }
  }

  pub fn add_observer<T: 'static + ResourcePileObserver>(&mut self, observer: Weak<RefCell<T>>) {
    self.observers.push(observer);
  }

  pub fn register<T: Identifiable + ResourceType>(&mut self, t: T) -> Resource<T> {
    let key = t.uid().into();
    let arc = Arc::new(RwLock::new(t));

    self
      .resources
      .entry(TypeId::of::<T>())
      .or_insert(HashMap::new())
      .insert(key, arc.clone());

    self.observers.retain(|x| match x.upgrade() {
      Some(x) => true,
      None => false,
    });
    for obs in &mut self.observers {
      match obs.upgrade() {
        Some(mut o) => o
          .deref()
          .borrow_mut()
          .deref_mut()
          .on_create(TypeId::of::<T>(), key, Resource::new(arc.clone())),
        None => {}
      }
    }

    Resource::new(arc)
  }

  pub fn get_resource_path(&self, path: &str) -> String {
    format!("{}/{}", self.base_path, path)
  }
}

pub struct Resource<T: ?Sized> {
  arc: Arc<RwLock<dyn ResourceType>>,

  phantom_data: PhantomData<T>,
}

impl<T: ?Sized> Clone for Resource<T> {
  fn clone(&self) -> Self {
    Self {
      arc: self.arc.clone(),
      phantom_data: Default::default(),
    }
  }
}

impl<T: ResourceType + ?Sized> Resource<T> {
  pub fn new(arc: Arc<RwLock<dyn ResourceType>>) -> Resource<T> {
    Resource {
      arc,
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

pub struct ResourceReadGuard<'a, T: ResourceType + ?Sized> {
  resource_rw_lock: &'a RwLock<dyn ResourceType>,

  phantom_data: PhantomData<&'a T>,
}

impl<'a, T: ResourceType + ?Sized> ResourceReadGuard<'a, T> {
  pub fn new(resource: &'a Resource<T>) -> ResourceReadGuard<T> {
    unsafe {
      resource.arc.raw().lock_shared();
      ResourceReadGuard {
        resource_rw_lock: &*resource.arc,
        phantom_data: Default::default(),
      }
    }
  }

  pub fn cast<R: ResourceType>(&mut self) -> &R {
    let k = unsafe { &*self.resource_rw_lock.data_ptr() };
    k.downcast_ref::<R>().unwrap()
  }
}

impl<'a, T: ResourceType + ?Sized> Drop for ResourceReadGuard<'a, T> {
  fn drop(&mut self) {
    unsafe {
      self.resource_rw_lock.raw().unlock_shared();
    }
  }
}

impl<'a, T: ResourceType> Deref for ResourceReadGuard<'a, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    let k = unsafe { &*self.resource_rw_lock.data_ptr() };
    k.downcast_ref().unwrap()
  }
}

pub struct ResourceWriteGuard<'a, T: ResourceType + ?Sized> {
  resource_rw_lock: &'a RwLock<dyn ResourceType>,

  phantom_data: PhantomData<T>,
}

impl<'a, T: ResourceType + ?Sized> ResourceWriteGuard<'a, T> {
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

impl<'a, T: ResourceType + ?Sized> Drop for ResourceWriteGuard<'a, T> {
  fn drop(&mut self) {
    unsafe { self.resource_rw_lock.raw().unlock_exclusive() }
  }
}

impl<'a, T: ResourceType> Deref for ResourceWriteGuard<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.resource_rw_lock.data_ptr() }.downcast_ref().unwrap()
  }
}

impl<'a, T: ResourceType> DerefMut for ResourceWriteGuard<'a, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.resource_rw_lock.data_ptr() }
      .downcast_mut()
      .unwrap()
  }
}

unsafe impl<T> Sync for Resource<T> {}
unsafe impl<T> Send for Resource<T> {}

pub trait ResourceType: DowncastSync {}
impl_downcast!(sync ResourceType);

pub trait ResourcePileObserver {
  fn on_create(&mut self, type_id: TypeId, process_unique_id: ProcessUniqueId, resource: Resource<dyn ResourceType>);
}
