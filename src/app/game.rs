use alers::renderer::opengl::{RenderTasks, Context};
use alers::resource;
use std::fs;
use alers::math::transform::Transform;
use alers::data::id::Identifiable;

pub struct Game {

}

impl Game {
  pub fn new() -> Game {
    Game {

    }
  }

  pub fn load(&mut self, context : &mut Context) {
    // Load meshes
    let mut mesh = resource::fbx_convert::to_static_meshes(
      resource::fbx::load("resources/test/geom/triangle.fbx").unwrap()).remove(0);

    // Load shaders
    let mut lambert = resource::shader::ShaderFile::new(
      fs::read_to_string("shaders/lambert.vs").unwrap(),
      fs::read_to_string("shaders/lambert.fs").unwrap()
    );

    context.static_mesh(&mesh);
    context.shader(&lambert);
  }

  pub fn tick(&mut self) {

  }

  pub fn render<T: RenderTasks>(&mut self, render_tasks: &mut T) {

  }
}