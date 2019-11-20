use alers::resource;

#[test]
pub fn load_fbx_simple_get_vertices(){
  alers::log::init_test();

  let mut cube_fbx = resource::fbx::load("resources/test/geom/basic_blender.fbx")
    .expect("Fail to load cube fbx");

  let cube_object = cube_fbx.objects()
                            .filter(move |object| object.name() == Some("Cube") && object.class() == "Geometry")
                            .into_iter()
                            .nth(0);
  assert_eq!(cube_object.is_some(), true);

  // Get vertices
  let mut node = cube_object.unwrap().node().children_by_name("Vertices").nth(0).unwrap();
  info!("{} : {:?}", node.name(), node.attributes());
}

#[test]
pub fn load_fbx_rigged() {
  alers::log::init_test();

  let mut fbx = resource::fbx::load("resources/test/geom/cylinder_rig.fbx")
    .expect("Fail to load rigged fbx");

  for object in fbx.objects() {
    info!("Object: {:?}", object.name())
  }
}