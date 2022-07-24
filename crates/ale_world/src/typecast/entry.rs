use std::any::TypeId;
use crate::typecast::registry::{CastIntoTrait, ImplEntry, Registry, TraitcastFrom};

pub trait Traitcast<To: ?Sized> {
  fn cast_mut(&mut self, registry: &Registry) -> Option<&mut To>;

  fn cast_ref(&self, registry: &Registry) -> Option<&To>;
}

impl<From, To> Traitcast<To> for From
  where
    From: TraitcastFrom + ?Sized,
    To: ?Sized + 'static,
{
  fn cast_mut(&mut self, registry: &Registry) -> Option<&mut To> {
    registry.get::<To>()?.cast_mut(self)
  }

  fn cast_ref(&self, registry: &Registry) -> Option<&To> {
    registry.get::<To>()?.cast_ref(self)
  }
}

pub struct EntryBuilder {
  pub insert: Box<dyn Fn(&mut Registry)>,
  pub struct_impl: TypeId,
  pub dyn_trait: TypeId,
}

impl EntryBuilder {
  pub fn insert<To>(entry: ImplEntry<To>) -> EntryBuilder
    where
      To: 'static + ?Sized,
  {
    let source_impl = entry.tid.clone();
    EntryBuilder {
      insert: Box::new(move |master| {
        let ctid = TypeId::of::<CastIntoTrait<To>>();
        let table = master
          .tables
          .entry::<>(ctid)
          .or_insert(Box::new(CastIntoTrait::<To>::new()));

        match table.downcast_mut::<CastIntoTrait<To>>() {
          None => { return; }
          Some(table) => table.map.insert(entry.tid, entry.clone())
        };
      }),
      struct_impl: source_impl,
      dyn_trait: TypeId::of::<To>(),
    }
  }
}

#[macro_export]
macro_rules! wire_component {
  ($source:ty, $target:ty) => {
    ale_world::typecast::entry::EntryBuilder::insert(ale_world::typecast::registry::ImplEntry::<$source> {
      cast_box: |x| {
        let x: Box<$target> = x.downcast()?;
        let x: Box<$source> = x;
        Ok(x)
      },
      cast_mut: |x| {
        let x: &mut $target = x.downcast_mut()?;
        let x: &mut $source = x;
        Some(x)
      },
      cast_ref: |x| {
        let x: &$target = x.downcast_ref()?;
        let x: &$source = x;
        Some(x)
      },
      tid: std::any::TypeId::of::<$target>(),
      from_name: stringify!($source),
      into_name: stringify!($target),
    })
  };
}
