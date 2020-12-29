use crate::console::ale_opengl_console_render;
use crate::envmap::{OpenGLEnvmap, OpenGLEnvmapLoader};
use crate::mesh::{OpenGLMesh, OpenGLMeshLoader};
use crate::pbr::render_pbr;
use crate::resource_pile::OpenGLResourcePile;
use crate::shader::{OpenGLShader, OpenGLShaderLoader};
use crate::texture::OpenGLTextureLoader;
use ale_autoid::ProcessUniqueId;
use ale_camera::fly_camera::FlyCamera;
use ale_ecs::World;
use ale_material::PBRMaterial;
use ale_math::num_traits::real::Real;
use ale_math::transform::Transform;
use ale_mesh::Mesh;
use ale_resource::{Resource, ResourcePile, ResourcePileObserver};
use ale_shader::Shader;
use ale_texture::envmap::Envmap;
use ale_texture::Texture;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub mod console;
pub mod envmap;
pub mod mesh;
pub mod pbr;
pub mod raw;
pub mod render_frame;
pub mod resource_pile;
pub mod shader;
pub mod text;
pub mod texture;

pub struct OpenGLRenderer {
  resource: Rc<RefCell<OpenGLResourcePile>>,
}

impl OpenGLRenderer {
  pub fn new() -> OpenGLRenderer {
    OpenGLRenderer {
      resource: Rc::new(RefCell::new(OpenGLResourcePile {
        resource: Default::default(),
        loader: Default::default(),
      })),
    }
  }

  pub fn attach_resource_loader(&mut self, resource_pile: &mut ResourcePile) {
    self
      .resource
      .borrow_mut()
      .add_loader::<OpenGLMeshLoader, Mesh>(OpenGLMeshLoader);
    self
      .resource
      .borrow_mut()
      .add_loader::<OpenGLShaderLoader, Shader>(OpenGLShaderLoader);
    self
      .resource
      .borrow_mut()
      .add_loader::<OpenGLTextureLoader, Texture>(OpenGLTextureLoader);
    self
      .resource
      .borrow_mut()
      .add_loader::<OpenGLEnvmapLoader, Envmap>(OpenGLEnvmapLoader);

    resource_pile.add_observer(Rc::downgrade(&self.resource))
  }

  pub fn clear_buffer() {
    // Clear the screen buffer
    unsafe {
      raw::clear_buffer();
    }
  }

  pub fn enable_blend() {
    unsafe {
      raw::enable_blend();
      raw::enable_blend_transparent();
    }
  }

  pub fn enable_depth_test() {
    unsafe {
      raw::enable_depth_test();
    }
  }

  pub fn render(&mut self, world: &mut World, camera: &mut FlyCamera) {
    for (id, (mesh, pbr, mut transform)) in world.query_mut::<(&Resource<Mesh>, &PBRMaterial, &mut Transform)>() {
      let this = self.resource.borrow();
      let ogl_mesh = this.retrieve_resource(&mesh).unwrap();
      let ogl_shader = this.retrieve_resource(&pbr.shader).unwrap();
      let ogl_envmap = this.retrieve_resource(&pbr.envmap).unwrap();

      render_pbr(
        &ogl_mesh.read(),
        &ogl_shader.read(),
        &ogl_envmap.read(),
        pbr,
        &mut transform,
        &camera.calculate_render_info(),
      );
    }
  }
}
