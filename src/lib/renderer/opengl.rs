use std::collections::HashMap;
use std::ptr;

use cgmath::{Matrix, Matrix4};

use crate::camera::CameraRenderInfo;
use crate::data::id::Id;
use crate::data::id::Identifiable;
use crate::renderer::opengl::cubemap::CubemapDrawInfo;
use crate::renderer::opengl::renderbuffer::RenderbufferDrawInfo;
use crate::renderer::opengl::shader::{ShaderDrawInfo, ShaderError, ShaderVariableType, ShaderVariable};
use crate::renderer::opengl::static_mesh::{StaticMeshDrawInfo, StaticMeshError};
use crate::renderer::opengl::texture::{TextureDrawInfo, TextureError};
use crate::resource::shader::ShaderFile;
use crate::resource::static_mesh::StaticMesh;
use crate::resource::texture::Texture;

pub mod static_mesh;
pub mod renderbuffer;
pub mod cubemap;
pub mod shader;
pub mod texture;
pub mod raw;

pub struct Context {
  static_meshes: HashMap<Id, StaticMeshDrawInfo>,
  shaders: HashMap<Id, ShaderDrawInfo>,
  textures: HashMap<Id, TextureDrawInfo>,
  _renderbuffer: HashMap<Id, RenderbufferDrawInfo>,
  _cubemap: HashMap<Id, CubemapDrawInfo>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      static_meshes: HashMap::new(),
      shaders: HashMap::new(),
      textures: HashMap::new(),
      _renderbuffer: HashMap::new(),
      _cubemap: HashMap::new()
    }
  }

  pub fn setup(&mut self) {
    unsafe { raw::enable_depth_test(); }
  }

  pub fn static_mesh(&mut self, mesh: &StaticMesh) -> Result<(), StaticMeshError> {
    self.static_meshes.insert(mesh.uid(), StaticMeshDrawInfo::new(mesh)?);
    Ok(())
  }

  pub fn shader(&mut self, shader: &ShaderFile) -> Result<(), ShaderError> {
    self.shaders.insert(shader.uid(), ShaderDrawInfo::new(shader)?);
    Ok(())
  }

  pub fn texture(&mut self, texture: &Texture) -> Result<(), TextureError> {
    self.textures.insert(texture.uid(), TextureDrawInfo::new(texture)?);
    Ok(())
  }

  pub fn get_static_mesh(&self, mesh_id: &Id) -> Option<&StaticMeshDrawInfo> {
    self.static_meshes.get(&mesh_id)
  }

  pub fn get_shader(&self, shader_id: &Id) -> Option<&ShaderDrawInfo> {
    self.shaders.get(&shader_id)
  }

  pub fn get_texture(&self, texture_id: &Id) -> Option<&TextureDrawInfo> {
    self.textures.get(&texture_id)
  }
}

enum Renderable {
  StaticMesh {
    shader_id: Id,
    mesh_id: Id,
    texture_ids: Vec<Id>,
    transform: Matrix4<f32>,
    shader_variables: Vec<ShaderVariable>,
  },

  EquirectCubemapProjection {
    shader_id: Id, // Projection shader
    mesh_id: Id, // Cube mesh
    texture_id: Id, // Equirect
    shader_variables: Vec<ShaderVariable>,
  },
}

pub trait RenderTasks {
  fn queue_static_mesh(&mut self,
                       shader: &ShaderFile,
                       mesh: &StaticMesh,
                       textures: Vec<&Texture>,
                       transform: Matrix4<f32>,
                       shader_vars: Vec<ShaderVariable>);

  fn queue_cubemap_projection(&mut self,
                              equirect_shader_id : &ShaderFile,
                              cube_mesh : &StaticMesh,
                              equirect_texture : &Texture,
                              shader_vars : Vec<ShaderVariable>);

  fn render(&mut self, context: &Context, camera: &mut CameraRenderInfo);
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
  fn queue_static_mesh(&mut self,
                       shader: &ShaderFile,
                       mesh: &StaticMesh,
                       textures: Vec<&Texture>,
                       transform: Matrix4<f32>,
                       shader_variables: Vec<ShaderVariable>)
  {
    self.renderables.push(Renderable::StaticMesh {
      shader_id: shader.uid(),
      mesh_id: mesh.uid(),
      texture_ids: textures.into_iter().map(|x| x.uid()).collect(),
      transform,
      shader_variables,
    });
  }

  fn queue_cubemap_projection(&mut self,
                              equirect_shader_id : &ShaderFile,
                              cube_mesh : &StaticMesh,
                              equirect_texture : &Texture,
                              shader_variables: Vec<ShaderVariable>) {
    self.renderables.push(Renderable::EquirectCubemapProjection {
      shader_id: equirect_shader_id.uid(),
      mesh_id: cube_mesh.uid(),
      texture_id: equirect_texture.uid(),
      shader_variables
    })
  }

  fn render(&mut self, context: &Context, camera: &mut CameraRenderInfo) {

    // Clear the screen buffer
    unsafe { raw::clear_buffer(); }

    for renderable in &self.renderables {
      match renderable {
        Renderable::StaticMesh { shader_id, mesh_id, texture_ids, transform, shader_variables } => {
          let mesh_draw_info = match context.get_static_mesh(mesh_id) {
            None => continue,
            Some(x) => x
          };
          let shader_draw_info = match context.get_shader(shader_id) {
            None => continue,
            Some(x) => x,
          };

          unsafe {
            // Bind shader
            raw::use_shader(shader_draw_info.shader);

            // Bind textures here
            for i in 0..texture_ids.len() {
              let texture_draw_info = match context.get_texture(&texture_ids[i]) {
                None => continue,
                Some(x) => x,
              };

              raw::active_texture(i as u32);
              raw::bind_texture(texture_draw_info.texture);
            }

            // Pass shader specific uniforms
            for shader_variable in shader_variables {
              match shader_variable.variable_type {
                ShaderVariableType::F32_3(vec) => raw::uniform3f(shader_draw_info.shader, &shader_variable.name, vec.x, vec.y, vec.z),
                ShaderVariableType::F32_4(vec) => raw::uniform4f(shader_draw_info.shader, &shader_variable.name, vec.x, vec.y, vec.z, vec.w),
              }
            }

            // Pass uniforms
            raw::matrix4f(shader_draw_info.shader, "model", transform.as_ptr());
            raw::matrix4f(shader_draw_info.shader, "view", camera.view.as_ptr());
            raw::matrix4f(shader_draw_info.shader, "projection", camera.projection.as_ptr());

            // Bind Array Buffer
            gl::BindVertexArray(mesh_draw_info.vao);

            // Draw according to EBO
            match mesh_draw_info.ebo {
              None => gl::DrawArrays(gl::TRIANGLES, 0, mesh_draw_info.draw_size as i32),
              Some(_) => gl::DrawElements(gl::TRIANGLES, mesh_draw_info.draw_size as i32, gl::UNSIGNED_INT, ptr::null()),
            }
          }
        }
        Renderable::EquirectCubemapProjection { shader_id: _, mesh_id: _, texture_id: _, shader_variables: _ } => {

        }
      }
    }
  }
}