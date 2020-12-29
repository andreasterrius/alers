use ale_autoid::{Identifiable, ProcessUniqueId};
use ale_resource::{Resource, ResourcePileObserver};
use downcast_rs::{impl_downcast, Downcast, DowncastSync};
use parking_lot::lock_api::RawRwLock;
use parking_lot::RwLock;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub struct OpenGLResourcePile {
  pub resource: HashMap<TypeId, HashMap<ProcessUniqueId, Arc<RwLock<dyn OpenGLResourceType>>>>,

  pub loader: HashMap<TypeId, Box<dyn OpenGLResourceRouter>>,
}

impl OpenGLResourcePile {
  pub fn new() -> OpenGLResourcePile {
    OpenGLResourcePile {
      resource: Default::default(),
      loader: Default::default(),
    }
  }

  pub fn add_loader<T: 'static + OpenGLResourceRouter, R: 'static>(&mut self, t: T) {
    self.loader.insert(TypeId::of::<R>(), Box::new(t));
  }

  pub fn register<T: 'static + OpenGLResourceType + Identifiable>(&mut self, t: T) -> OpenGLResource<T> {
    let key = t.uid().into();
    let arc = Arc::new(RwLock::new(t));
    self
      .resource
      .entry(TypeId::of::<T>())
      .or_insert(HashMap::new())
      .insert(key, arc.clone());

    OpenGLResource::new(arc.clone())
  }

  pub fn retrieve<T: 'static + Identifiable, R: OpenGLResourceType>(&self, t: &T) -> Option<OpenGLResource<R>> {
    let key = t.uid().into();
    let arc = self.resource.get(&TypeId::of::<T>())?.get(&key)?;

    Some(OpenGLResource::new(arc.clone()))
  }

  pub fn retrieve_resource<T: 'static + Identifiable, R: OpenGLResourceType>(
    &self,
    t: &Resource<T>,
  ) -> Option<OpenGLResource<R>> {
    let key = t.uid().into();
    let arc = self.resource.get(&TypeId::of::<T>())?.get(&key)?;

    Some(OpenGLResource::new(arc.clone()))
  }
}

#[macro_export]
macro_rules! route_loader {
  ($loader_ident:ident, $struct_ident:ident) => {
    impl crate::resource_pile::OpenGLResourceRouter for $loader_ident {
      fn route_create(
        &self,
        opengl_resource_pile: &OpenGLResourcePile,
        any: crate::Resource<dyn ale_resource::ResourceType>,
      ) -> std::sync::Arc<parking_lot::RwLock<dyn crate::resource_pile::OpenGLResourceType>> {
        let r = self.create(opengl_resource_pile, any.read().cast::<$struct_ident>());
        std::sync::Arc::new(parking_lot::RwLock::new(r))
      }
    }
  };
}

pub trait OpenGLResourceRouter {
  fn route_create(
    &self,
    opengl_resource_pile: &OpenGLResourcePile,
    any: ale_resource::Resource<dyn ale_resource::ResourceType>,
  ) -> Arc<RwLock<dyn OpenGLResourceType>>;
}

pub trait OpenGLResourceLoader<Before, After> {
  fn create(&self, opengl_resource_pile: &OpenGLResourcePile, before: &Before) -> After;
}

impl ResourcePileObserver for OpenGLResourcePile {
  fn on_create(
    &mut self,
    type_id: TypeId,
    process_unique_id: ProcessUniqueId,
    resource: ale_resource::Resource<dyn ale_resource::ResourceType>,
  ) {
    let loader = self.loader.get(&type_id).unwrap();
    let result = { loader.route_create(&self, resource) };
    self
      .resource
      .entry(type_id)
      .or_insert(HashMap::new())
      .insert(process_unique_id, result);
  }
}

pub trait OpenGLResourceType: DowncastSync {}
impl_downcast!(sync OpenGLResourceType);

pub struct OpenGLResource<T: ?Sized> {
  arc: Arc<RwLock<dyn OpenGLResourceType>>,

  phantom_data: PhantomData<T>,
}

impl<T: ?Sized> Clone for OpenGLResource<T> {
  fn clone(&self) -> Self {
    Self {
      arc: self.arc.clone(),
      phantom_data: Default::default(),
    }
  }
}

impl<T: OpenGLResourceType + ?Sized> OpenGLResource<T> {
  pub fn new(arc: Arc<RwLock<dyn OpenGLResourceType>>) -> OpenGLResource<T> {
    OpenGLResource {
      arc,
      phantom_data: Default::default(),
    }
  }

  pub fn read(&self) -> OpenGLResourceReadGuard<T> {
    OpenGLResourceReadGuard::new(self)
  }

  pub fn write(&self) -> OpenGLResourceWriteGuard<T> {
    OpenGLResourceWriteGuard::new(self)
  }
}

pub struct OpenGLResourceReadGuard<'a, T: OpenGLResourceType + ?Sized> {
  resource_rw_lock: &'a RwLock<dyn OpenGLResourceType>,

  phantom_data: PhantomData<&'a T>,
}

impl<'a, T: OpenGLResourceType + ?Sized> OpenGLResourceReadGuard<'a, T> {
  pub fn new(resource: &'a OpenGLResource<T>) -> OpenGLResourceReadGuard<T> {
    unsafe {
      resource.arc.raw().lock_shared();
      OpenGLResourceReadGuard {
        resource_rw_lock: &*resource.arc,
        phantom_data: Default::default(),
      }
    }
  }

  pub fn cast<R: OpenGLResourceType>(&mut self) -> &R {
    let k = unsafe { &*self.resource_rw_lock.data_ptr() };
    k.downcast_ref::<R>().unwrap()
  }
}

impl<'a, T: OpenGLResourceType + ?Sized> Drop for OpenGLResourceReadGuard<'a, T> {
  fn drop(&mut self) {
    unsafe {
      self.resource_rw_lock.raw().unlock_shared();
    }
  }
}

impl<'a, T: OpenGLResourceType> Deref for OpenGLResourceReadGuard<'a, T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    let k = unsafe { &*self.resource_rw_lock.data_ptr() };
    k.downcast_ref().unwrap()
  }
}

pub struct OpenGLResourceWriteGuard<'a, T: OpenGLResourceType + ?Sized> {
  resource_rw_lock: &'a RwLock<dyn OpenGLResourceType>,

  phantom_data: PhantomData<T>,
}

impl<'a, T: OpenGLResourceType + ?Sized> OpenGLResourceWriteGuard<'a, T> {
  pub fn new(resource: &'a OpenGLResource<T>) -> OpenGLResourceWriteGuard<'a, T> {
    unsafe {
      resource.arc.raw().lock_exclusive();
    }
    OpenGLResourceWriteGuard {
      resource_rw_lock: &resource.arc,
      phantom_data: Default::default(),
    }
  }
}

impl<'a, T: OpenGLResourceType + ?Sized> Drop for OpenGLResourceWriteGuard<'a, T> {
  fn drop(&mut self) {
    unsafe { self.resource_rw_lock.raw().unlock_exclusive() }
  }
}

impl<'a, T: OpenGLResourceType> Deref for OpenGLResourceWriteGuard<'a, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.resource_rw_lock.data_ptr() }.downcast_ref().unwrap()
  }
}

impl<'a, T: OpenGLResourceType> DerefMut for OpenGLResourceWriteGuard<'a, T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.resource_rw_lock.data_ptr() }
      .downcast_mut()
      .unwrap()
  }
}

unsafe impl<T> Sync for OpenGLResource<T> {}
unsafe impl<T> Send for OpenGLResource<T> {}
