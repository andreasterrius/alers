use std::iter::FromIterator;

struct BufferElementInfo {
  name: String,
  size: usize,
}

pub struct Buffer<T: Clone> {
  data: Vec<T>,
  element_info: Vec<BufferElementInfo>,

  // Denotes how many attributes a row contains
  total_row_size: usize,

  // Denotes the actual size of the buffer
  size: usize,
}

impl<T: Clone> Buffer<T> {
  pub fn elements(&self, name: &str) -> Option<BufferElementIterator<T>> {
    let mut size_now = 0;
    for i in 0..self.element_info.len() {
      let info = &self.element_info[i];
      if &info.name == name {
        return Some(BufferElementIterator {
          data: &self.data,
          offset: self.total_row_size - info.size,
          ctr: 0,
          size: info.size,
          index: size_now
        });
      }
      size_now += self.element_info[i].size;
    }
    None
  }
}

#[derive(Debug)]
pub enum BufferBuildError {
  BadElementSize
}

pub struct BufferBuilder<T: Clone> {
  data: Vec<T>,
  element_info: Vec<BufferElementInfo>,
  total_row_size: usize,
}

impl<T: Clone> BufferBuilder<T> {
  pub fn new(data: Vec<T>) -> BufferBuilder<T> {
    BufferBuilder {
      data,
      total_row_size: 0,
      element_info: vec![]
    }
  }

  pub fn info(mut self, name: &str, size: usize) -> BufferBuilder<T> {
    self.element_info.push(BufferElementInfo { name: name.to_owned(), size });
    self.total_row_size += size;
    self
  }

  pub fn build(mut self) -> Result<Buffer<T>, BufferBuildError> {
    let size = self.data.len() / self.total_row_size;
    Ok(Buffer {
      data: self.data,
      element_info: self.element_info,
      total_row_size: self.total_row_size,
      size
    })
  }
}

pub struct SeparateBufferBuilder<T: Clone> {
  element_data: Vec<Vec<T>>,
  element_info: Vec<BufferElementInfo>,
  total_row_size: usize,
}

impl<T : Clone> SeparateBufferBuilder<T> {
  pub fn new() -> SeparateBufferBuilder<T> {
    SeparateBufferBuilder {
      element_data: vec![],
      element_info: vec![],
      total_row_size: 0
    }
  }

  pub fn info(mut self, name: &str, size: usize, data: Vec<T>) -> SeparateBufferBuilder<T> {
    self.element_data.push(data);
    self.element_info.push(BufferElementInfo { name: name.to_owned(), size });
    self.total_row_size += size;
    self
  }

  pub fn build(mut self) -> Result<Buffer<T>, BufferBuildError> {
    let mut column_size = 0;
    if !self.element_data.is_empty() {
      column_size = self.element_data[0].len() / self.element_info[0].size;
    }
    for i in 0..self.element_data.len() {
      let this_column_size = self.element_data[i].len() / self.element_info[i].size;
      if this_column_size != column_size {
        return Err(BufferBuildError::BadElementSize);
      }
    }

    let mut data = vec!();
    for i in 0..column_size {
      for j in 0..self.element_info[i].size {
        data.extend_from_slice(&self.element_data[i])
      }
    }

    Ok(Buffer {
      data,
      element_info: self.element_info,
      total_row_size: self.total_row_size,
      size: column_size
    })
  }
}

pub struct BufferElementIterator<'a, T> {
  pub data: &'a Vec<T>,

  // Skip how many
  pub offset: usize,

  // If ctr == size-1, do offset then reset to 0
  pub ctr: usize,
  pub size: usize,
  pub index: usize
}

impl<'a, T : Clone> Iterator for BufferElementIterator<'a, T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    match self.index >= self.data.len() {
      true => None,
      false => {
        let value = self.data[self.index].clone();
        if self.ctr >= self.size - 1 {
          self.ctr = 0;
          self.index += self.offset;
        } else {
          self.ctr += 1;
        }
        self.index += 1;
        Some(value)
      }
    }
  }
}

#[test]
pub fn test_buffers() {
  let data: Vec<f32> = vec![
    // vertices (3), uv(2), normals(3)
    1.0, 1.0, 1.0, 10.0, 10.0, -1.0, -1.0, -1.0,
    2.0, 2.0, 2.0, 20.0, 20.0, -2.0, -2.0, -3.0,
    3.0, 3.0, 3.0, 30.0, 30.0, -2.0, -2.0, -3.0,
  ];

  let buffer: Buffer<f32> = BufferBuilder::new(data)
    .info("vertex", 3)
    .info("uv", 2)
    .info("normal", 3)
    .build()
    .unwrap();

  let vertices_expected = vec!(
    1.0, 1.0, 1.0,
    2.0, 2.0, 2.0,
    3.0, 3.0, 3.0,
  );
  let vertices: Vec<f32> = buffer.elements("vertex").unwrap().collect();
  for i in 0..vertices.len() { relative_eq!(vertices[i], vertices_expected[i]); }

  let uvs_expected = vec!(
    10.0, 10.0,
    20.0, 20.0,
    30.0, 30.0,
  );
  let uvs: Vec<f32> = buffer.elements("uv").unwrap().collect();
  for i in 0..uvs.len() { relative_eq!(uvs[i], uvs_expected[i]); }

  let normals_expected = vec!(
    -1.0, -1.0, -1.0,
    -2.0, -2.0, -2.0,
    -3.0, -3.0, -3.0,
  );
  let normals: Vec<f32> = buffer.elements("normal").unwrap().collect();
  for i in 0..normals.len() { relative_eq!(normals[i], normals_expected[i]); }
}
