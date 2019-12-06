use data::buffer::SeparateBufferBuilder;
use data::id::Id;
use resource::static_mesh::StaticMesh;

#[derive(Debug)]
pub enum ConversionError {
  NGonNotSupported,
  IncompleteLastPoly
}

pub fn to_static_meshes(fbx: fbxcel_dom::v7400::Document) -> Result<Vec<StaticMesh>, ConversionError> {

  //Get root node
  let root = fbx.scenes().nth(0).unwrap().node().tree().root();
  let objects = root.children_by_name("Objects").nth(0).unwrap();
  let mut meshes = vec!();
  for object in objects.children_by_name("Geometry") {
    // Get position of vertices
    let mut vbuffer_builder = SeparateBufferBuilder::new();
    let position = object.children_by_name("Vertices").nth(0).unwrap().attributes().iter().nth(0).unwrap().get_arr_f64().unwrap();
    vbuffer_builder = vbuffer_builder.info("vertex", 3, position.iter().map(|x| *x as f32).collect::<Vec<f32>>());

    // Get uv coords
//    let uv_node = object.children_by_name("LayerElementUV").nth(0);
//    if let Some(uv_node) = uv_node {
//      let uvs = uv_node.children_by_name("UV").nth(0).unwrap().attributes().iter().nth(0).unwrap().get_arr_f64().unwrap();
//      vbuffer_builder = vbuffer_builder.info("uv", 2, uvs.iter().map(|x| *x as f32).collect::<Vec<f32>>());
//    }

    // Get normals
//    let normal_node = object.children_by_name("LayerElementNormal").nth(0);
//    if let Some(normal_node) = normal_node {
//      let uvs = normal_node.children_by_name("Normals").nth(0).unwrap().attributes().iter().nth(0).unwrap().get_arr_f64().unwrap();
//      vbuffer_builder = vbuffer_builder.info("normal", 3, uvs.iter().map(|x| *x as f32).collect::<Vec<f32>>());
//    }

    let element_node = object.children_by_name("PolygonVertexIndex").nth(0);
    let mut ibuffer_builder = SeparateBufferBuilder::new();
    if let Some(element_node) = element_node {
      let mut indices = element_node.attributes().iter().nth(0).unwrap().get_arr_i32().unwrap();
      ibuffer_builder = ibuffer_builder.info("index", 3, parse_indices(indices)?);
    }

    meshes.push(StaticMesh::new(
      vbuffer_builder.build().unwrap(),
      ibuffer_builder.build().ok())
    )
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
      println!("start {}, end {}, indices {}", start, end, i);
      if end - start == 2 { // 2 index apart
        // 1 tri
        arr.push(indices[start]);
        arr.push(indices[start + 1]);
        arr.push(!indices[start + 2]); // flip last bit
      } else if end - start == 3 { // 3 index apart
        // Quad, convert to 2 tris
        arr.push(indices[start]);
        arr.push(indices[start+1]);
        arr.push(indices[start+2]);

        arr.push(indices[start]);
        arr.push(indices[start+2]);
        arr.push(!indices[start+3]);
      } else if end - start >= 4 {
        return Err(ConversionError::NGonNotSupported);
      }
      // Reset counter to the next index
      // since we have consumed a set of polys
      start = i+1;
    } else {
      end = i;
    }
  }

  println!("result {:?}", arr);
  println!("start {}, end {}", start, end);

  // there's a poly that isn't consumed
  // possibly because it has less than 2 vertices
  if end != indices.len()-1 {
    return Err(ConversionError::IncompleteLastPoly);
  }

  Ok(arr)
}

#[test]
pub fn fbx_to_buffers_should_properly_parse() {
  crate::log::init_test();

  let mut fbx = crate::resource::fbx::load("resources/test/geom/basic_blender.fbx")
    .expect("Fail to load rigged fbx");

  let meshes = to_static_meshes(fbx);
  println!("{:#?}", meshes);
}