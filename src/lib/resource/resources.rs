use resource::static_mesh::StaticMesh;
use resource::shader::ShaderFile;
use std::collections::HashMap;
use resource::{ResourceEventListener, fbx, fbx_convert};
use resource::fbx::LoadError;

pub struct Resources {
  pub static_mesh : HashMap<String, Vec<StaticMesh>>,
  pub shader_files : HashMap<String, ShaderFile>,
  pub listener: Vec<Box<ResourceEventListener>>,
}

impl Resources {
  pub fn new() -> Resources {
    Resources {
      static_mesh: HashMap::new(),
      shader_files: HashMap::new(),
      listener: vec![]
    }
  }

  pub fn add_listener(&mut self, listener : Box<ResourceEventListener>) {
    self.listener.push(listener);
  }

  pub fn load_static_mesh_from_fbx(&mut self, path : &str) -> Result<&Vec<StaticMesh>, LoadError> {
    let fbx = fbx::load(path)?;
    let meshes = fbx_convert::to_static_meshes(fbx);
    for listener in &mut self.listener { listener.on_static_mesh_loaded(&meshes);  }
    self.static_mesh.insert(path.to_owned(), meshes);
    Ok(self.static_mesh.get(path).unwrap())
  }

  pub fn load_shaders_from_file(&mut self) {

  }


}