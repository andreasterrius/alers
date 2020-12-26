pub use snowflake::*;

pub trait Identifiable {
  type Ret: Into<ProcessUniqueId>;
  fn uid(&self) -> Self::Ret;
}

#[macro_export]
macro_rules! struct_id_impl {
  ($id_ident: ident, $struct_ident:ident, $field_ident:ident) => {
    impl Identifiable for $struct_ident {
      type Ret = $id_ident;
      fn uid(&self) -> $id_ident {
        self.$field_ident
      }
    }

    impl From<$struct_ident> for ProcessUniqueId {
      fn from(t: $struct_ident) -> Self {
        t.$field_ident.0
      }
    }
  };
}

#[macro_export]
macro_rules! struct_id {
  ($T: ident) => {
    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
    pub struct $T(pub ProcessUniqueId);

    impl $T {
      pub fn new() -> $T {
        $T(ProcessUniqueId::new())
      }
    }

    impl Default for $T {
      fn default() -> Self {
        $T::new()
      }
    }

    impl From<$T> for ProcessUniqueId {
      fn from(t: $T) -> ProcessUniqueId {
        t.0
      }
    }
  };
}
