use std::collections::HashMap;
use std::ptr;

use cgmath::{Matrix, Matrix4, Point3, Deg, Vector3};
use cgmath::prelude::*;

use crate::camera::CameraRenderInfo;
use crate::data::id::Id;
use crate::data::id::Identifiable;
use crate::renderer::constant::{MODEL, PROJECTION, VIEW};
use crate::renderer::opengl::cubemap::CubemapDrawInfo;
use crate::renderer::opengl::renderbuffer::RenderbufferDrawInfo;
use crate::renderer::opengl::shader::{ShaderDrawInfo, ShaderError, ShaderVariable, ShaderVariableType};
use crate::renderer::opengl::static_mesh::{StaticMeshDrawInfo, StaticMeshError};
use crate::renderer::opengl::texture::{TextureDrawInfo, TextureError};
use crate::resource::shader::ShaderFile;
use crate::resource::static_mesh::StaticMesh;
use crate::resource::texture::Texture;
use crate::renderer::opengl::RenderError::{UnregisteredMesh, UnregisteredShader, UnregisteredTexture, UnregisteredFramebuffer};
use crate::renderer::opengl::framebuffer::{FramebufferDrawInfo, FramebufferError};

pub mod static_mesh;
pub mod renderbuffer;
pub mod framebuffer;
pub mod cubemap;
pub mod shader;
pub mod texture;
pub mod raw;

pub struct Context {
  static_meshes: HashMap<Id, StaticMeshDrawInfo>,
  shaders: HashMap<Id, ShaderDrawInfo>,
  textures: HashMap<Id, TextureDrawInfo>,

  //internal context
  framebuffers: HashMap<Id, FramebufferDrawInfo>,
  _cubemap: HashMap<Id, CubemapDrawInfo>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      static_meshes: HashMap::new(),
      shaders: HashMap::new(),
      textures: HashMap::new(),
      framebuffers: HashMap::new(),
      _cubemap: HashMap::new(),
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

  pub fn framebuffer(&mut self) -> Result<Id, FramebufferError> {
    let id = Id::new();
    self.framebuffers.insert(id, FramebufferDrawInfo::new()?);
    Ok(id)
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

  pub fn get_framebuffer(&self, framebuffer: &Id) -> Option<&FramebufferDrawInfo> {
    self.framebuffers.get(&framebuffer)
  }
}

enum Renderable {
  StaticMesh {
    shader_id: Id,
    mesh_id: Id,
    texture_ids: Vec<Id>,
    transform: Matrix4<f32>,
    camera_render_info : CameraRenderInfo,
    shader_variables: Vec<ShaderVariable>,
  },

  EquirectCubemapProjection {
    shader_id: Id,
    mesh_id: Id,
    texture_id: Id,
    shader_variables: Vec<ShaderVariable>,
  },
}

pub trait RenderTasks {
  fn queue_static_mesh(&mut self,
                       shader: &ShaderFile,
                       mesh: &StaticMesh,
                       textures: Vec<&Texture>,
                       transform: Matrix4<f32>,
                       camera_render_info: CameraRenderInfo,
                       shader_vars: Vec<ShaderVariable>);

  fn queue_cubemap_projection(&mut self,
                              equirect_shader_id: &ShaderFile,
                              cube_mesh: &StaticMesh,
                              equirect_texture: &Texture,
                              shader_vars: Vec<ShaderVariable>);

  fn render(&mut self, context: &Context) -> Result<(), RenderError>;
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
                       camera_render_info: CameraRenderInfo,
                       shader_variables: Vec<ShaderVariable>)
  {
    self.renderables.push(Renderable::StaticMesh {
      shader_id: shader.uid(),
      mesh_id: mesh.uid(),
      texture_ids: textures.into_iter().map(|x| x.uid()).collect(),
      transform,
      camera_render_info,
      shader_variables,
    });
  }

  fn queue_cubemap_projection(&mut self,
                              equirect_shader_id: &ShaderFile,
                              cube_mesh: &StaticMesh,
                              equirect_texture: &Texture,
                              shader_variables: Vec<ShaderVariable>) {
    self.renderables.push(Renderable::EquirectCubemapProjection {
      shader_id: equirect_shader_id.uid(),
      mesh_id: cube_mesh.uid(),
      texture_id: equirect_texture.uid(),
      shader_variables,
    })
  }

  fn render(&mut self, context: &Context) -> Result<(), RenderError> {

    // Clear the screen buffer
    unsafe { raw::clear_buffer(); }

    for renderable in &self.renderables {
      match renderable {
        Renderable::StaticMesh { shader_id, mesh_id, texture_ids, transform, camera_render_info, shader_variables } => {
          let mesh_draw_info = context.get_static_mesh(mesh_id).ok_or(UnregisteredMesh(*mesh_id))?;
          let shader_draw_info = context.get_shader(shader_id).ok_or(UnregisteredShader(*shader_id))?;

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
            raw::matrix4f(shader_draw_info.shader, MODEL, transform.as_ptr());
            raw::matrix4f(shader_draw_info.shader, VIEW, camera_render_info.view.as_ptr());
            raw::matrix4f(shader_draw_info.shader, PROJECTION, camera_render_info.projection.as_ptr());

            // Bind Array Buffer
            raw::bind_vao(mesh_draw_info.vao);

            // Draw according to EBO
            match mesh_draw_info.ebo {
              None => raw::draw_arrays(0, mesh_draw_info.draw_size),
              Some(_) => raw::draw_elements(mesh_draw_info.draw_size),
            }
          }
        }
        Renderable::EquirectCubemapProjection { shader_id, mesh_id, texture_id, shader_variables } => {

          let cube_mesh_draw_info = context.get_static_mesh(mesh_id).ok_or(UnregisteredMesh(*mesh_id))?;
          let shader_draw_info = context.get_shader(shader_id).ok_or(UnregisteredShader(*shader_id))?;
          let texture_draw_info = context.get_texture(&texture_id).ok_or(UnregisteredTexture(*texture_id))?;

          let projection = cgmath::perspective(Deg(90.0f32), 1.0f32, 0.1f32, 10.0f32);
          let views = vec!(
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(1.0f32, 0.0, 0.0), -Vector3::unit_y()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(-1.0f32, 0.0, 0.0), -Vector3::unit_y()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 1.0, 0.0), -Vector3::unit_z()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, -1.0, 0.0), Vector3::unit_z()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, 1.0), -Vector3::unit_y()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, 1.0), -Vector3::unit_y()),
          );
          let equirect_shader = shader_draw_info.shader;
          let texture = texture_draw_info.texture;

          unsafe {
            let (framebuffer, _) = raw::create_framebuffer();

            raw::use_shader(equirect_shader);
            raw::uniform1i(equirect_shader, "equirectangularMap", 0);
            raw::matrix4f(equirect_shader, PROJECTION, projection.as_ptr());

            raw::active_texture(0);
            raw::bind_texture(texture);

            raw::set_viewport(0, 0, 512, 512);
            raw::bind_framebuffer(framebuffer);
            for i in 0..6 {
              raw::matrix4f(equirect_shader, VIEW, views[i].as_ptr());
              raw::clear_buffer();

              raw::bind_vao(cube_mesh_draw_info.vao);
              raw::draw_arrays(0, cube_mesh_draw_info.draw_size);
              raw::bind_vao(0);
            }
            // unbind framebuffer
            raw::bind_framebuffer(0);
          }


        }
      }

    }
    Ok(())
  }
}

pub enum RenderError {
  UnregisteredMesh(Id),
  UnregisteredShader(Id),
  UnregisteredTexture(Id),
  UnregisteredFramebuffer(Id),
}