use data::id::Id;
use cgmath::{Matrix4};
use std::collections::HashMap;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use resource::static_mesh::StaticMesh;
use resource::shader::ShaderFile;
use data::id::Identifiable;

pub struct Context {
  static_meshes: HashMap<Id, StaticMeshDrawInfo>,
  shaders: HashMap<Id, ShaderDrawInfo>
}

impl Context {
  pub fn new() -> Context {
    Context {
      static_meshes: HashMap::new(),
      shaders: HashMap::new(),
    }
  }

  pub fn static_mesh(&mut self, mesh: &StaticMesh) {
    self.static_meshes.insert(mesh.uid(), StaticMeshDrawInfo::new(mesh));
  }

  pub fn shader(&mut self, shader: &ShaderFile) {
    self.shaders.insert(shader.uid(), ShaderDrawInfo::new(shader));
  }
}

pub struct StaticMeshDrawInfo {}

impl StaticMeshDrawInfo {
  pub fn new(mesh: &StaticMesh) -> StaticMeshDrawInfo {
    StaticMeshDrawInfo {}
  }
}

pub struct ShaderDrawInfo {}

impl ShaderDrawInfo {
  pub fn new(shader: &ShaderFile) -> ShaderDrawInfo {
    ShaderDrawInfo {}
  }
}

enum Renderable {
    StaticMesh { shader_id : Id, mesh_id : Id, transform : Matrix4<f32> }
}

pub trait RenderTasks {
  fn queue_static_mesh(&mut self, shader: &ShaderFile, mesh: &StaticMesh, transform: Matrix4<f32>);

  fn render(&mut self, context:  &Context);
}

pub struct SimpleRenderTasks {
  renderables: Vec<Renderable>
}

impl SimpleRenderTasks {
  pub fn new() -> SimpleRenderTasks {
    SimpleRenderTasks { renderables: vec![] }
  }
}

impl RenderTasks for SimpleRenderTasks {
  fn queue_static_mesh(&mut self, shader: &ShaderFile, mesh: &StaticMesh, transform: Matrix4<f32>) {
    self.renderables.push(Renderable::StaticMesh {
      shader_id: shader.uid(),
      mesh_id: mesh.uid(),
      transform
    });
  }

  fn render(&mut self, context: &Context) {
    unimplemented!()
  }
}
