use std::fs::File;
use std::io::{BufReader};
use fbxcel_dom::any::AnyDocument;
use fbxcel_dom::v7400::Document;

pub fn load(path : &str) -> Result<Document, LoadError> {
  let file: File = File::open(path)?;
  let reader = BufReader::new(file);
  match AnyDocument::from_seekable_reader(reader)? {
    AnyDocument::V7400(doc) => Ok(*doc),
    _ => Err(LoadError::VersionUnsupported),
  }
}

#[derive(Debug)]
pub enum LoadError {
  FileNotFound,
  BadParsing,
  VersionUnsupported
}

impl From<std::io::Error> for LoadError {
  fn from(_: std::io::Error) -> Self {
     return LoadError::FileNotFound
  }
}

impl From<fbxcel_dom::any::Error> for LoadError {
  fn from(test: fbxcel_dom::any::Error) -> Self {
      println!("{:?}", test);
      return LoadError::BadParsing
  }
}

