use std::path::Path;

pub struct ResourcePath;

impl ResourcePath {
  pub fn find(path: &str) -> String {
    let p = Path::new(env!("CARGO_MANIFEST_DIR"))
      .parent()
      .unwrap()
      .parent()
      .unwrap()
      .join("resources")
      .join(path);
    p.to_str().unwrap().to_owned()
  }
}


