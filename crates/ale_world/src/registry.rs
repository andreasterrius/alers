use traitcast_core::{CastIntoTrait, Registry, TraitcastFrom};
pub use traitcast_core::ImplEntry;

pub trait Traitcast<To: ?Sized> {
  fn cast_mut(&mut self, registry: &Registry) -> Option<&mut To>;
}

impl<From, To> Traitcast<To> for From
  where
    From: TraitcastFrom + ?Sized,
    To: ?Sized + 'static,
{
  fn cast_mut(&mut self, registry: &Registry) -> Option<&mut To> {
    cast_mut(self, registry)
  }
}

pub fn cast_mut<'a, From, To>(x: &'a mut From, registry: &Registry) -> Option<&'a mut To>
  where
    From: TraitcastFrom + ?Sized,
    To: ?Sized + 'static,
{
  registry
    .cast_into::<To>()
    .expect("Calling cast_mut to cast into an unregistered trait object")
    .from_mut(x)
}

pub struct EntryBuilder {
  pub insert: Box<dyn Fn(&mut Registry)>,
}

impl EntryBuilder {
  pub fn insert<To>(entry: ImplEntry<To>) -> EntryBuilder
    where
      To: 'static + ?Sized,
  {
    EntryBuilder {
      insert: Box::new(move |master| {
        let table: &mut CastIntoTrait<To> =
          master.tables
            .entry::<CastIntoTrait<To>>()
            .or_insert(CastIntoTrait::new());

        table.map.insert(entry.tid, entry.clone());
      })
    }
  }
}

#[macro_export]
macro_rules! wire_component {
    ($source:ty, $target:ty) => {
        $crate::registry::EntryBuilder::insert($crate::registry::ImplEntry::<$source> {
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
            into_name: stringify!($target)
        })
    };
}
