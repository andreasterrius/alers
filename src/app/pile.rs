use alers::resource;
use alers::resource::shader::ShaderFile;
use alers::resource::static_mesh::StaticMesh;
use std::fs;

pub struct Pile {
  pub cube_mesh : StaticMesh,
  pub lambert_shader : ShaderFile,
}

impl Pile {
  pub fn load_initial() -> Pile {

    // Load meshes
    let mut fbx = resource::fbx::load("resources/test/geom/triangle.fbx").unwrap();
    let mut meshes = resource::fbx_convert::to_simple_static_meshes(fbx);

    // Load shaders
    let mut lambert = resource::shader::ShaderFile::new(
      fs::read_to_string("shaders/lambert.vs").unwrap(),
      fs::read_to_string("shaders/lambert.fs").unwrap()
    );

    Pile {
      cube_mesh: meshes.remove(0),
      lambert_shader: lambert
    }
  }
}