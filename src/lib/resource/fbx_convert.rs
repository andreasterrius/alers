use data::buffer::{Buffer, SeparateBufferBuilder};
use resource::static_mesh::StaticMesh;

#[derive(Debug)]
pub enum ConversionError {
  NGonNotSupported,
  IncompleteLastPoly,
  PolgyonVertexIndexNotFound,
}

pub fn to_static_meshes(fbx: fbxcel_dom::v7400::Document) -> Result<Vec<StaticMesh>, ConversionError> {

  //Get root node
  let root = fbx.scenes().nth(0).unwrap().node().tree().root();
  let objects = root.children_by_name("Objects").nth(0).unwrap();
  let mut meshes = vec!();
  for object in objects.children_by_name("Geometry") {
    let element_node = object.children_by_name("PolygonVertexIndex").nth(0);
    let indices = match element_node {
      None => { return Err(ConversionError::PolgyonVertexIndexNotFound); }
      Some(element_node) => {
        let indices = element_node.attributes().iter().nth(0).unwrap().get_arr_i32().unwrap();
        parse_indices(indices)?
      }
    };

    // Get position of vertices
    let position_arr = object.children_by_name("Vertices").nth(0).unwrap().attributes().iter().nth(0).unwrap().get_arr_f64().unwrap();

    // Get uv coords
    let uv_node = object.children_by_name("LayerElementUV").nth(0);
    let uv_arr = match uv_node {
      None => { &[0.0f64; 0] }
      Some(uv_node) => uv_node.children_by_name("UV").nth(0).unwrap().attributes().iter().nth(0).unwrap().get_arr_f64().unwrap()
    };

    //Get normals
    let normal_node = object.children_by_name("LayerElementNormal").nth(0);
    let normal_arr = match normal_node {
      None => { &[0.0f64; 0] }
      Some(normal_node) => normal_node.children_by_name("Normals").nth(0).unwrap().attributes().iter().nth(0).unwrap().get_arr_f64().unwrap(),
    };

    let vbuffer = construct_buffer(&indices, position_arr, uv_arr, normal_arr)?;

    meshes.push(StaticMesh::new(vbuffer, None));
  }

  Ok(meshes)
}

// Only receives tris or quads
pub fn parse_indices(indices: &[i32]) -> Result<Vec<i32>, ConversionError> {
  let mut start = 0;
  let mut end = 0;
  let mut arr = vec!();
  for i in 0..indices.len() {
    if indices[i] < 0 {
      end = i;
      //println!("start {}, end {}, indices {}", start, end, i);
      if end - start == 2 { // 2 index apart
        // 1 tri
        arr.push(indices[start]);
        arr.push(indices[start + 1]);
        arr.push(!indices[start + 2]); // flip last bit
      } else if end - start == 3 { // 3 index apart
        // Quad, convert to 2 tris
        arr.push(indices[start]);
        arr.push(indices[start + 1]);
        arr.push(indices[start + 2]);

        arr.push(indices[start]);
        arr.push(indices[start + 2]);
        arr.push(!indices[start + 3]);
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

  Ok(arr)
}

pub fn construct_buffer(indices: &[i32],
                        position_arr: &[f64],
                        uv_arr: &[f64],
                        normal_arr: &[f64]) -> Result<Buffer<f32>, ConversionError>
{
  let mut position_vec = vec!();
  let mut uv_vec = vec!();
  let mut normal_vec = vec!();
  for i in 0..indices.len() {

    // calculate offsets
    let index_3 = (indices[i] * 3) as usize;
    let index_2 = (indices[i] * 2) as usize;

    // Push vertices
    position_vec.push(position_arr[index_3] as f32);
    position_vec.push(position_arr[index_3 + 1] as f32);
    position_vec.push(position_arr[index_3 + 2] as f32);

    if uv_arr.len() != 0 {
      uv_vec.push(uv_arr[index_2] as f32);
      uv_vec.push(uv_arr[index_2 + 1] as f32);
    } else {
      uv_vec.push(0.0f32);
      uv_vec.push(0.0f32);
    }
    if normal_arr.len() != 0 {
      normal_vec.push(normal_arr[index_3] as f32);
      normal_vec.push(normal_arr[index_3 + 1] as f32);
      normal_vec.push(normal_arr[index_3 + 2] as f32);
    } else {
      normal_vec.push(0.0f32);
      normal_vec.push(0.0f32);
      normal_vec.push(0.0f32);
    }
  }

//  println!("{:?}", position_vec);
//  println!("{:?}", uv_vec);
//  println!("{:?}", normal_vec);

  let vbuffer = SeparateBufferBuilder::new()
    .info("position", 3, position_vec)
    .info("uv", 2, uv_vec)
    .info("normal", 3, normal_vec)
    .build().unwrap();
  Ok(vbuffer)
}

#[test]
pub fn fbx_to_buffers_should_properly_parse() {
  crate::log::init_test();

  let mut fbx = crate::resource::fbx::load("resources/test/geom/basic_blender.fbx")
    .expect("Fail to load rigged fbx");

  let meshes = to_static_meshes(fbx);
  println!("{:#?}", meshes);
}