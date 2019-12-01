use std::collections::HashMap;
use std::fs;
use std::ops::Add;

use renderer::opengl::ShaderError;
use resource::{fbx, fbx_convert, ResourceEventObserver};
use resource;
use resource::fbx::LoadError;
use resource::shader::ShaderFile;
use resource::static_mesh::StaticMesh;

//pub struct Resources {
//  static_mesh: HashMap<String, Vec<StaticMesh>>,
//  shader_files: HashMap<String, ShaderFile>,
//}
//
//impl Resources {
//  pub fn new() -> Resources {
//    Resources {
//      static_mesh: HashMap::new(),
//      shader_files: HashMap::new()
//    }
//  }
//
//  pub fn load_static_mesh_from_fbx<T: ResourceEventObserver>(&mut self,
//                                                             path: &str,
//                                                             observer: Option<&mut T>) -> Result<&Vec<StaticMesh>, LoadError>
//  {
//    let key = path.to_owned();
//    match self.static_mesh.contains_key(&key) {
//      Some(meshes) => {
//        return Ok(meshes);
//      }
//      None => {
//        let fbx = fbx::load(path)?;
//        let meshes = fbx_convert::to_static_meshes(fbx);
//        if let Some(observer) = observer { observer.on_static_mesh_loaded(&meshes) };
//
//        self.static_mesh.insert(key.clone(), meshes);
//        return Ok(self.static_mesh.get(&key).unwrap());
//      }
//    }
//  }
//
//  pub fn load_shaders_from_file<T: ResourceEventObserver>(&mut self,
//                                                          vs_path: &str,
//                                                          fs_path: &str,
//                                                          observer: Option<&mut T>) -> Result<&ShaderFile, ()>
//  {
//    let shader = resource::shader::ShaderFile::new(
//      fs::read_to_string(vs_path).unwrap(),
//      fs::read_to_string(fs_path).unwrap());
//
//    if let Some(observer) = observer { observer.on_shader_loaded(&shader) };
//
//    let key = vs_path.to_owned().add(fs_path);
//    self.shader_files.insert(key.clone(), shader);
//    Ok(&self.shader_files.get(&key).unwrap())
//  }
//}