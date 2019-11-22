use data::mesh::SimpleStaticMesh;

pub fn fbx_to_buffers(fbx : fbxcel_dom::v7400::Document) -> Vec<SimpleStaticMesh> {

  //Get root node
  let root = fbx.scenes().nth(0).unwrap().node().tree().root();
  let objects = root.children_by_name("Objects"). nth(0).unwrap();
  for object in objects.children_by_name("Geometry") {
    // Get position of vertices
    let position_node = object.children_by_name("Vertices").nth(0).unwrap();
    for position_av in position_node.attributes().iter().nth(0) {
      match position_av {
        fbxcel_dom::fbxcel::low::v7400::AttributeValue::ArrF64(positions) => {

        }
        _ => panic!()
      }
    }

  }

  vec!()
}

#[test]
pub fn fbx_to_buffers_should_properly_parse() {
  crate::log::init_test();

  let mut fbx = crate::resource::fbx::load("resources/test/geom/basic_blender.fbx")
    .expect("Fail to load rigged fbx");

  fbx_to_buffers(fbx);
}