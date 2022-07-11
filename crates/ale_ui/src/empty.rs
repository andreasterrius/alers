use crate::layout::Layout;

pub struct Empty {
  pub layout: Layout,
}

impl Empty {
  pub fn new() -> Empty {
    Empty {
      layout: Layout::new(),
    }
  }
}
