use crate::buffer::{Buffer, BufferBuilder, SeparateBufferBuilder};
use ale_autoid::*;

pub mod buffer;

#[derive(Debug)]
pub struct Mesh {
  pub id: MeshId,
  pub vertices: Buffer<f32>,
  pub indices: Option<Buffer<i32>>,
}

struct_id!(MeshId);
struct_id_impl!(MeshId, Mesh, id);

pub fn ale_mesh_new(vertices: Buffer<f32>, indices: Option<Buffer<i32>>) -> Mesh {
  Mesh {
    id: MeshId::new(),
    vertices,
    indices,
  }
}

pub fn ale_mesh_new_cube() -> Mesh {
  let vertices = BufferBuilder::new(vec![
    // back face
    -1.0f32, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // bottom-left
    1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, // top-right
    1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 0.0, // bottom-right
    1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, // top-right
    -1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // bottom-left
    -1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0, // top-left
    // front face
    -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom-left
    1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, // bottom-right
    1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // top-right
    1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // top-right
    -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, // top-left
    -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom-left
    // left face
    -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // top-right
    -1.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, 1.0, // top-left
    -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // bottom-left
    -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // bottom-left
    -1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, // bottom-right
    -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // top-right
    // right face
    1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, // top-left
    1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, // bottom-right
    1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top-right
    1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, // bottom-right
    1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, // top-left
    1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, // bottom-left
    // bottom face
    -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, // top-right
    1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 1.0, 1.0, // top-left
    1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, // bottom-left
    1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, // bottom-left
    -1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, // bottom-right
    -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, // top-right
    // top face
    -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top-left
    1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom-right
    1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0, // top-right
    1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom-right
    -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top-left
    -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, // bottom-left
  ])
  .info("vertex", 3)
  .info("normal", 3)
  .info("uv", 2)
  .build()
  .unwrap();

  ale_mesh_new(vertices, None)
}

pub fn ale_mesh_new_plane() -> Mesh {
  let vertices = BufferBuilder::new(vec![0.0f32, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0])
    .info("vertex", 2)
    .build()
    .unwrap();

  ale_mesh_new(vertices, None)
}

#[derive(Debug)]
enum ConversionError {
  NGonNotSupported,
  IncompleteLastPoly,
  PolgyonVertexIndexNotFound,
}

fn intern_parse_indices(indices: &[i32]) -> Result<(Vec<i32>, Vec<usize>), ConversionError> {
  let mut start = 0;
  let mut end = 0;
  let mut arr = vec![];
  let mut con = Vec::new();
  //println!("bb: {:?}", indices);
  for i in 0..indices.len() {
    if indices[i] < 0 {
      end = i;
      //println!("start {}, end {}, indices {}", start, end, i);
      if end - start == 2 {
        // 2 index apart
        let _last_idx = arr.len() as i32;
        // 1 tri
        arr.push(indices[start]);
        arr.push(indices[start + 1]);
        arr.push(!indices[start + 2]); // flip last bit

        con.push(start);
        con.push(start + 1);
        con.push(start + 2);
      } else if end - start == 3 {
        // 3 index apart
        let _last_idx = arr.len() as i32;
        // Quad, convert to 2 tris
        arr.push(indices[start]);
        arr.push(indices[start + 1]);
        arr.push(indices[start + 2]);

        arr.push(indices[start + 2]);
        arr.push(!indices[start + 3]);
        arr.push(indices[start]);

        con.push(start);
        con.push(start + 1);
        con.push(start + 2);

        con.push(start + 2);
        con.push(start + 3);
        con.push(start);
      } else if end - start >= 4 {
        return Err(ConversionError::NGonNotSupported);
      }
      // Reset counter to the next index
      // since we have consumed a set of polys
      start = i + 1;
    } else {
      end = i;
    }
  }

  //  println!("result {:?}", arr);
  //  println!("start {}, end {}", start, end);

  // there's a poly that isn't consumed
  // possibly because it has less than 2 vertices
  if end != indices.len() - 1 {
    return Err(ConversionError::IncompleteLastPoly);
  }

  Ok((arr, con))
}

fn intern_construct_buffer_flat(
  indices: &[i32],
  indices_con: &Vec<usize>,
  position_arr: &[f64],
  uv_arr: &[f64],
  normal_arr: &[f64],
) -> Result<Buffer<f32>, ConversionError> {
  let mut position_vec = vec![];
  let mut uv_vec = vec![];
  let mut normal_vec = vec![];

  for i in 0..indices.len() {
    let idx = indices[i];

    // calculate offsets
    let index_3 = (idx * 3) as usize;
    let index_2 = (idx * 2) as usize;

    // Push vertices
    position_vec.push(position_arr[index_3] as f32);
    position_vec.push(position_arr[index_3 + 1] as f32);
    position_vec.push(position_arr[index_3 + 2] as f32);

    //    if normal_arr.len() != 0 {
    //      normal_vec.push(normal_arr[i] as f32);
    //      normal_vec.push(normal_arr[i + 1] as f32);
    //      normal_vec.push(normal_arr[i + 2] as f32);
    //    } else {
    //      normal_vec.push(0.0f32);
    //      normal_vec.push(0.0f32);
    //      normal_vec.push(0.0f32);
    //    }

    if uv_arr.len() != 0 {
      uv_vec.push(uv_arr[index_2] as f32);
      uv_vec.push(uv_arr[index_2 + 1] as f32);
    } else {
      uv_vec.push(0.0f32);
      uv_vec.push(0.0f32);
    }
  }

  for i in 0..indices_con.len() {
    let idx = indices_con[i] * 3;
    normal_vec.push(normal_arr[idx] as f32);
    normal_vec.push(normal_arr[idx + 1] as f32);
    normal_vec.push(normal_arr[idx + 2] as f32);
  }

  let vbuffer = SeparateBufferBuilder::new()
    .info("position", 3, position_vec)
    .info("normal", 3, normal_vec)
    .info("uv", 2, uv_vec)
    .build()
    .unwrap();

  //println!("{:#?}", vbuffer);

  Ok(vbuffer)
}
