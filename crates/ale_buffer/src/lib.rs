use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug)]
pub struct BufferElementInfo {
  pub name: String,
  pub size: usize,
}

#[derive(Debug)]
pub struct Buffer<T: Clone> {
  data: Vec<T>,
  element_info_order: Vec<BufferElementInfo>,

  // Lookup from the name and the offset it has
  element_info_offset: HashMap<String, usize>,

  // Denotes how many attributes a row contains
  column_len: usize,

  // Denotes the actual size of the buffer
  row_len: usize,
}

impl<T: Clone> Buffer<T> {
  pub fn elements(&self) -> &Vec<BufferElementInfo> {
    &self.element_info_order
  }

  pub fn element_iter(&self, name: &str) -> Option<BufferElementIterator<T>> {
    let mut size_now = 0;
    for i in 0..self.element_info_order.len() {
      let info = &self.element_info_order[i];
      if &info.name == name {
        return Some(BufferElementIterator {
          data: &self.data,
          offset: self.column_len - info.size,
          ctr: 0,
          size: info.size,
          index: size_now,
        });
      }
      size_now += self.element_info_order[i].size;
    }
    None
  }

  pub fn len(&self) -> usize {
    self.data.len()
  }

  pub fn total_column_len(&self) -> usize {
    self.column_len
  }

  pub fn total_row_len(&self) -> usize {
    self.row_len
  }

  // Get pointer to the start of the data
  pub fn as_ptr(&self) -> *const T {
    self.data.as_ptr()
  }

  pub fn offset(&self, name: &str) -> Option<usize> {
    self.element_info_offset.get(name).cloned()
  }
}

impl<T: Clone> Index<usize> for Buffer<T> {
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    &self.data[index]
  }
}

#[derive(Debug)]
pub enum BufferBuildError {
  BadElementSize,
}

pub struct BufferBuilder<T: Clone> {
  data: Vec<T>,
  element_info: Vec<BufferElementInfo>,
  element_info_offset: HashMap<String, usize>,
  column_len: usize,
}

impl<T: Clone> BufferBuilder<T> {
  pub fn new(data: Vec<T>) -> BufferBuilder<T> {
    BufferBuilder {
      data,
      column_len: 0,
      element_info: vec![],
      element_info_offset: HashMap::new(),
    }
  }

  pub fn info(mut self, name: &str, size: usize) -> BufferBuilder<T> {
    self.element_info.push(BufferElementInfo {
      name: name.to_owned(),
      size,
    });

    self.element_info_offset.insert(name.to_owned(), self.column_len);

    self.column_len += size;
    self
  }

  pub fn build(self) -> Result<Buffer<T>, BufferBuildError> {
    let size = self.data.len() / self.column_len;
    Ok(Buffer {
      data: self.data,
      element_info_order: self.element_info,
      element_info_offset: self.element_info_offset,
      column_len: self.column_len,
      row_len: size,
    })
  }
}

pub struct SeparateBufferBuilder<T: Clone> {
  element_data: Vec<Vec<T>>,
  element_info: Vec<BufferElementInfo>,
  element_offset: HashMap<String, usize>,
  total_column_size: usize,
}

impl<T: Clone> SeparateBufferBuilder<T> {
  pub fn new() -> SeparateBufferBuilder<T> {
    SeparateBufferBuilder {
      element_data: vec![],
      element_info: vec![],
      element_offset: HashMap::new(),
      total_column_size: 0,
    }
  }

  pub fn info(mut self, name: &str, size: usize, data: Vec<T>) -> SeparateBufferBuilder<T> {
    self.element_data.push(data);
    self.element_info.push(BufferElementInfo {
      name: name.to_owned(),
      size,
    });
    self.element_offset.insert(name.to_owned(), self.total_column_size);
    self.total_column_size += size;
    self
  }

  pub fn build(self) -> Result<Buffer<T>, BufferBuildError> {
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

    let mut data: Vec<T> = vec![];
    for i in 0..column_size {
      for j in 0..self.element_info.len() {
        let start = i * self.element_info[j].size;
        let end = start + self.element_info[j].size;
        data.extend_from_slice(&self.element_data[j][start..end])
      }
    }

    Ok(Buffer {
      data,
      element_info_order: self.element_info,
      element_info_offset: self.element_offset,
      column_len: self.total_column_size,
      row_len: column_size,
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
  pub index: usize,
}

impl<'a, T: Clone> Iterator for BufferElementIterator<'a, T> {
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
  use approx::relative_eq;

  let data: Vec<f32> = vec![
    // vertices (3), uv(2), normals(3)
    1.0, 1.0, 1.0, 10.0, 10.0, -1.0, -1.0, -1.0, 2.0, 2.0, 2.0, 20.0, 20.0, -2.0, -2.0, -3.0, 3.0, 3.0, 3.0, 30.0, 30.0,
    -2.0, -2.0, -3.0,
  ];

  let buffer: Buffer<f32> = BufferBuilder::new(data)
    .info("vertex", 3)
    .info("uv", 2)
    .info("normal", 3)
    .build()
    .unwrap();

  assert_eq!(buffer.element_info_offset.get("vertex"), Some(&0));
  assert_eq!(buffer.element_info_offset.get("uv"), Some(&3));
  assert_eq!(buffer.element_info_offset.get("normal"), Some(&5));

  let vertices_expected = vec![1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 3.0, 3.0, 3.0];
  let vertices: Vec<f32> = buffer.element_iter("vertex").unwrap().collect();
  for i in 0..vertices.len() {
    relative_eq!(vertices[i], vertices_expected[i]);
  }

  let uvs_expected = vec![10.0, 10.0, 20.0, 20.0, 30.0, 30.0];
  let uvs: Vec<f32> = buffer.element_iter("uv").unwrap().collect();
  for i in 0..uvs.len() {
    relative_eq!(uvs[i], uvs_expected[i]);
  }

  let normals_expected = vec![-1.0, -1.0, -1.0, -2.0, -2.0, -2.0, -3.0, -3.0, -3.0];
  let normals: Vec<f32> = buffer.element_iter("normal").unwrap().collect();
  for i in 0..normals.len() {
    relative_eq!(normals[i], normals_expected[i]);
  }
}

#[test]
pub fn test_separate_buffers() {
  use approx::relative_eq;

  let vertices_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
  let uvs_data = vec![10.0, 20.0, 30.0, -30.0, 40.0, -40.0];
  let normals_data = vec![-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, -7.0, -8.0, -9.0];

  let buffer = SeparateBufferBuilder::new()
    .info("vertex", 3, vertices_data.clone())
    .info("uv", 2, uvs_data.clone())
    .info("normal", 3, normals_data.clone())
    .build()
    .unwrap();

  assert_eq!(buffer.element_info_offset.get("vertex"), Some(&0));
  assert_eq!(buffer.element_info_offset.get("uv"), Some(&3));
  assert_eq!(buffer.element_info_offset.get("normal"), Some(&5));

  let vertices: Vec<f64> = buffer.element_iter("vertex").unwrap().collect();
  let uvs: Vec<f64> = buffer.element_iter("uv").unwrap().collect();
  let normals: Vec<f64> = buffer.element_iter("normal").unwrap().collect();

  for i in 0..vertices.len() {
    relative_eq!(vertices[i], vertices_data[i]);
  }
  for i in 0..uvs.len() {
    relative_eq!(uvs[i], uvs_data[i]);
  }
  for i in 0..normals.len() {
    relative_eq!(normals[i], normals_data[i]);
  }
}
