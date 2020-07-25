#[macro_export]
macro_rules! impl_id {
  ($struct_ident:ident<$generic:tt>, $field_ident:ident) => {
    impl<$generic> crate::data::id::Identifiable for $struct_ident<$generic> {
      fn uid(&self) -> Id {
        self.$field_ident
      }
    }
  };
  ($struct_ident:ident, $field_ident:ident) => {
    impl crate::data::id::Identifiable for $struct_ident {
      fn uid(&self) -> Id {
        self.$field_ident
      }
    }
  };
}
