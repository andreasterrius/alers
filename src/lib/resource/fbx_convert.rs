use data::buffer::SeparateBufferBuilder;
use resource::static_mesh::StaticMesh;
use data::id::Id;

pub fn to_simple_statich_meshes(fbx: fbxcel_dom::v7400::Document) -> Vec<StaticMesh> {

  //Get root node
  let root = fbx.scenes().nth(0).unwrap().node().tree().root();
  let objects = root.children_by_name("Objects").nth(0).unwrap();
  let mut meshes = vec!();
  for object in objects.children_by_name("Geometry") {
    // Get position of vertices
    let mut vbuffer_builder = SeparateBufferBuilder::new();
    let position = object.children_by_name("Vertices").nth(0).unwrap().attributes().iter().nth(0).unwrap().get_arr_f64().unwrap();
    vbuffer_builder = vbuffer_builder.info("vertex", 3, position.to_vec());

    // Get uv coords
    let uv_node = object.children_by_name("LayerElementUV").nth(0);
    if let Some(uv_node) = uv_node {
      let uvs = uv_node.children_by_name("UV").nth(0).unwrap().attributes().iter().nth(0).unwrap().get_arr_f64().unwrap();
      vbuffer_builder = vbuffer_builder.info("uv", 2, uvs.to_vec());
    }

    // Get normals
    let normal_node = object.children_by_name("LayerElementNormal").nth(0);
    if let Some(normal_node) = normal_node {
      let uvs = normal_node.children_by_name("Normals").nth(0).unwrap().attributes().iter().nth(0).unwrap().get_arr_f64().unwrap();
      vbuffer_builder = vbuffer_builder.info("normal", 3, uvs.to_vec());
    }

    let element_node = object.children_by_name("PolygonVertexIndex").nth(0);
    let mut ibuffer_builder = SeparateBufferBuilder::new();
    if let Some(element_node) = element_node {
      let indices = element_node.attributes().iter().nth(0).unwrap().get_arr_i32().unwrap();
      ibuffer_builder = ibuffer_builder.info("index", 3, indices.to_vec());
    }
    
    meshes.push(StaticMesh {
      id: Id::new(),
      vertices: vbuffer_builder.build().unwrap(),
      indices: ibuffer_builder.build().ok()
    })
  }

  meshes
}

#[test]
pub fn fbx_to_buffers_should_properly_parse() {
  crate::log::init_test();

  let mut fbx = crate::resource::fbx::load("resources/test/geom/basic_blender.fbx")
    .expect("Fail to load rigged fbx");

  let meshes = to_simple_statich_meshes(fbx);
  println!("{:#?}", meshes);
}