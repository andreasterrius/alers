use ale_autoid::ProcessUniqueId;
use ale_resource::{Resource, ResourcePileObserver, ResourceType};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ops::Deref;

pub struct OpenGLResourcePile {
  pub resource: HashMap<TypeId, HashMap<ProcessUniqueId, Box<dyn Any>>>,

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
}

#[macro_export]
macro_rules! route_loader {
  ($loader_ident:ident, $struct_ident:ident) => {
    impl crate::resource_pile::OpenGLResourceRouter for $loader_ident {
      fn route_create(&self, any: crate::Resource<dyn ale_resource::ResourceType>) -> Box<dyn std::any::Any> {
        let r = self.create(any.read().cast::<$struct_ident>());
        Box::new(r)
      }
    }
  };
}

pub trait OpenGLResourceRouter {
  fn route_create(&self, any: Resource<dyn ResourceType>) -> Box<dyn Any>;
}

pub trait OpenGLResourceLoader<Before, After> {
  fn create(&self, before: &Before) -> After;
}

impl ResourcePileObserver for OpenGLResourcePile {
  fn on_create(&mut self, type_id: TypeId, process_unique_id: ProcessUniqueId, resource: Resource<dyn ResourceType>) {
    let loader = self.loader.get(&type_id).unwrap();
    let result = { loader.route_create(resource) };
    self
      .resource
      .entry(type_id)
      .or_insert(HashMap::new())
      .insert(process_unique_id, result);
  }
}
