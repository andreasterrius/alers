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

  pub fn render<T: RenderTasks>(&self, render_tasks: &mut T) {
    render_tasks.render(&self.static_meshes, &self.shaders);
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
  ShaderMesh(Id, Id)
}

pub trait RenderTasks {
  fn static_mesh_shader(&mut self, shader: &ShaderFile, mesh: &StaticMesh, transform: Matrix4<f32>);

  fn render(&self,
            static_mesh_info: &HashMap<Id, StaticMeshDrawInfo>,
            shader_info: &HashMap<Id, ShaderDrawInfo>);
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
  fn static_mesh_shader(&mut self, shader: &ShaderFile, mesh: &StaticMesh, transform: Matrix4<f32>) {
    self.renderables.push(Renderable::ShaderMesh(shader.uid(), mesh.uid()));
  }

  fn render(&self, static_mesh_info: &HashMap<Id, StaticMeshDrawInfo>, shader_info: &HashMap<Id, ShaderDrawInfo>) {
    unimplemented!()
  }
}
