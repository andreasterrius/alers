use crate::layout::Layout;

pub struct Empty {
  pub layout: Layout,
  pub(crate) name: String,
}

impl Empty {
  pub fn new(name: String) -> Empty {
    Empty {
      name,
      layout: Layout::new(),
    }
  }
}
