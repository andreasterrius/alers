use fbxcel_dom::any::AnyDocument;
use fbxcel_dom::v7400::Document;
use std::fs::File;
use std::io::BufReader;
use std::iter::repeat;

pub fn load(path: &str) -> Result<Document, LoadError> {
  let file: File = File::open(path)?;
  let reader = BufReader::new(file);
  match AnyDocument::from_seekable_reader(reader)? {
    AnyDocument::V7400(doc) => Ok(*doc),
    _ => Err(LoadError::VersionUnsupported),
  }
}

pub fn get_node_info_recursively(node: &fbxcel_dom::fbxcel::tree::v7400::NodeHandle, space: usize) -> String {
  let mut text = format!(
    "{}{}: {:?}\n",N
    repeat(' ').take(space).collect::<String>(),
    node.name(),
    node.attributes()
  );
  for child in node.children() {
    text.push_str(&get_node_info_recursively(&child, space + 2));
  }
  text
}

#[derive(Debug)]
pub enum LoadError {
  FileNotFound,
  BadParsing,
  VersionUnsupported,
}

impl From<std::io::Error> for LoadError {
  fn from(_: std::io::Error) -> Self {
    return LoadError::FileNotFound;
  }
}

impl From<fbxcel_dom::any::Error> for LoadError {
  fn from(_test: fbxcel_dom::any::Error) -> Self {
    return LoadError::BadParsing;
  }
}
