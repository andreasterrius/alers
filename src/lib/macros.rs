//#[macro_export]
//macro_rules! impl_id {
//  ($struct_ident:ident<$generic:tt>, $field_ident:ident) => {
//    impl<$generic> crate::data::id::Identifiable for $struct_ident<$generic> {
//      fn uid(&self) -> Id {
//        self.$field_ident
//      }
//    }
//  };
//  ($struct_ident:ident, $field_ident:ident) => {
//    impl crate::data::id::Identifiable for $struct_ident {
//      fn uid(&self) -> Id {
//        self.$field_ident
//      }
//    }
//  };
//}

#[macro_export]
macro_rules! struct_id_impl {
  ($id_ident: ident, $struct_ident:ident, $field_ident:ident) => {
    impl $struct_ident {
      pub fn uid(&self) -> $id_ident {
        self.$field_ident
      }
    }
  };
}

#[macro_export]
macro_rules! struct_id {
  ($T: ident) => {
    use snowflake::ProcessUniqueId;

    #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
    #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
    pub struct $T(pub snowflake::ProcessUniqueId);

    impl $T {
      pub fn new() -> $T {
        $T(ProcessUniqueId::new())
      }
    }
  };
}
