use std::collections::HashMap;
use std::ptr;

use cgmath::{Deg, Matrix, Matrix4, Point3, Vector3};
use cgmath::prelude::*;

use crate::camera::CameraRenderInfo;
use crate::data::display_info::DisplayInfo;
use crate::data::id::Id;
use crate::data::id::Identifiable;
use crate::data::rect2d::Rect2d;
use crate::renderer::constant::{CAMERA_POSITION, MODEL, PROJECTION, VIEW};
use crate::renderer::opengl::cubemap::{CubemapDrawInfo, CubemapError};
use crate::renderer::opengl::framebuffer::{FramebufferDrawInfo, FramebufferError};
use crate::renderer::opengl::renderbuffer::RenderbufferDrawInfo;
use crate::renderer::opengl::RenderError::{NoCameraSet, UnregisteredCubemap, UnregisteredFramebuffer, UnregisteredMesh, UnregisteredShader, UnregisteredTexture};
use crate::renderer::opengl::shader::{ShaderDrawInfo, ShaderError, ShaderVariable, ShaderVariableType};
use crate::renderer::opengl::static_mesh::{StaticMeshDrawInfo, StaticMeshError};
use crate::renderer::opengl::texture::{TextureDrawInfo, TextureError};
use crate::resource::cubemap::Cubemap;
use crate::resource::shader::ShaderFile;
use crate::resource::mesh::Mesh;
use crate::resource::texture::Texture;

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
  cubemap: HashMap<Id, CubemapDrawInfo>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      static_meshes: HashMap::new(),
      shaders: HashMap::new(),
      textures: HashMap::new(),
      framebuffers: HashMap::new(),
      cubemap: HashMap::new(),
    }
  }

  pub fn setup(&mut self) {
    unsafe {
      raw::enable_depth_test();
    }
  }

  pub fn static_mesh(&mut self, mesh: &Mesh) -> Result<(), StaticMeshError> {
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
    //self.framebuffers.insert(id, FramebufferDrawInfo::new()?);
    Ok(id)
  }

  pub fn cubemap(&mut self, cubemap: &Cubemap) -> Result<(), CubemapError> {
    self.cubemap.insert(cubemap.uid(), CubemapDrawInfo::new(&cubemap)?);
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

  pub fn get_framebuffer(&self, framebuffer: &Id) -> Option<&FramebufferDrawInfo> {
    self.framebuffers.get(&framebuffer)
  }

  pub fn get_cubemap(&self, cubemap: &Id) -> Option<&CubemapDrawInfo> {
    self.cubemap.get(&cubemap)
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
    equirect_shader_id: Id,
    cube_mesh_id: Id,
    projection_target: ProjectionTarget,
    cubemap_id: Id,
    projection_dimension: Rect2d,
    original_dimension: Rect2d,
    shader_variables: Vec<ShaderVariable>,
  },

  Skybox {
    skybox_shader_id: Id,
    cube_mesh_id: Id,
    rendered_cubemap_id: Id,
    shader_variables: Vec<ShaderVariable>,
  },
}

pub trait RenderTasks {
  fn with_camera(&mut self,
                 camera_render_info: CameraRenderInfo);

  fn queue_static_mesh(&mut self,
                       shader_id: Id,
                       mesh_id: Id,
                       texture_ids: Vec<Id>,
                       transform: Matrix4<f32>,
                       shader_vars: Vec<ShaderVariable>);

  fn queue_cubemap_projection(&mut self,
                              equirect_shader_id: Id,
                              cube_mesh_id: Id,
                              projection_target: ProjectionTarget,
                              cubemap_id: Id,
                              projection_dimension: Rect2d,
                              original_dimension: Rect2d,
                              shader_variables: Vec<ShaderVariable>);

  fn queue_skybox(&mut self,
                  skybox_shader_id: Id,
                  cube_mesh_id: Id,
                  rendered_cubemap_id: Id,
                  shader_variables: Vec<ShaderVariable>);

  fn with_skybox(&mut self,
                 cubemap_id: Id);

  fn render(&mut self, context: &Context) -> Result<Vec<RenderResult>, RenderError>;
}

pub struct SimpleRenderTasks {
  renderables: Vec<Renderable>,

  // CubemapId
  skybox: Option<Id>,

  camera_render_info: Option<CameraRenderInfo>,
}

impl SimpleRenderTasks {
  pub fn new() -> SimpleRenderTasks {
    SimpleRenderTasks { renderables: vec![], skybox: None, camera_render_info: None {} }
  }
}

impl RenderTasks for SimpleRenderTasks {
  fn with_camera(&mut self, camera_render_info: CameraRenderInfo) {
    self.camera_render_info = Some(camera_render_info);
  }

  fn queue_static_mesh(&mut self,
                       shader_id: Id,
                       mesh_id: Id,
                       texture_ids: Vec<Id>,
                       transform: Matrix4<f32>,
                       shader_variables: Vec<ShaderVariable>)
  {
    self.renderables.push(Renderable::StaticMesh {
      shader_id,
      mesh_id,
      texture_ids,
      transform,
      shader_variables,
    });
  }

  fn queue_cubemap_projection(&mut self,
                              equirect_shader_id: Id,
                              cube_mesh_id: Id,
                              projection_target: ProjectionTarget,
                              cubemap_id: Id,
                              projection_dimension: Rect2d,
                              original_dimension: Rect2d,
                              shader_variables: Vec<ShaderVariable>)
  {
    self.renderables.push(Renderable::EquirectCubemapProjection {
      equirect_shader_id,
      cube_mesh_id,
      projection_target,
      cubemap_id,
      projection_dimension,
      original_dimension,
      shader_variables,
    });
  }

  fn queue_skybox(&mut self,
                  skybox_shader_id: Id,
                  cube_mesh_id: Id,
                  rendered_cubemap_id: Id,
                  shader_variables: Vec<ShaderVariable>)
  {
    self.renderables.push(Renderable::Skybox {
      skybox_shader_id,
      cube_mesh_id,
      rendered_cubemap_id,
      shader_variables,
    });
  }

  fn with_skybox(&mut self,
                 cubemap_id: Id)
  {
    self.skybox = Some(cubemap_id);
  }

  fn render(&mut self, context: &Context) -> Result<Vec<RenderResult>, RenderError> {

    // Clear the screen buffer
    unsafe { raw::clear_buffer(); }

    let mut result = vec!();
    for renderable in &self.renderables {
      match renderable {
        Renderable::StaticMesh { shader_id, mesh_id, texture_ids, transform, shader_variables } => {
          let mesh_draw_info = context.get_static_mesh(mesh_id).ok_or(UnregisteredMesh(*mesh_id))?;
          let shader_draw_info = context.get_shader(shader_id).ok_or(UnregisteredShader(*shader_id))?;
          let camera_render_info = self.camera_render_info.as_ref().ok_or(NoCameraSet)?;

          unsafe {
            // Bind shader
            raw::use_shader(shader_draw_info.shader);

            if let Some(cubemap_id) = &self.skybox {
              let irradiance_cubemap_draw_info = context.cubemap.get(cubemap_id).ok_or(UnregisteredCubemap(*cubemap_id))?;
              raw::uniform1i(shader_draw_info.shader, "irradianceMap", 0);
              raw::active_texture(0);
              raw::bind_cubemap(irradiance_cubemap_draw_info.cubemap);
            }

            // Bind textures here
            for i in 0..texture_ids.len() {
              let texture_draw_info = match context.get_texture(&texture_ids[i]) {
                None => continue,
                Some(x) => x,
              };

              raw::active_texture((i + 1) as u32);
              raw::bind_texture(texture_draw_info.texture);
            }

            // Pass shader specific uniforms
            for shader_variable in shader_variables {
              match shader_variable.variable_type {
                ShaderVariableType::F32_1(ff) => raw::uniform1f(shader_draw_info.shader, &shader_variable.name, ff),
                ShaderVariableType::F32_3(vec) => raw::uniform3f(shader_draw_info.shader, &shader_variable.name, vec.x, vec.y, vec.z),
                ShaderVariableType::F32_4(vec) => raw::uniform4f(shader_draw_info.shader, &shader_variable.name, vec.x, vec.y, vec.z, vec.w),
              }
            }

            let camera_position = camera_render_info.position;
            raw::uniform3f(shader_draw_info.shader, CAMERA_POSITION, camera_position.x, camera_position.y, camera_position.z);

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
        Renderable::EquirectCubemapProjection {
          equirect_shader_id, cube_mesh_id, projection_target,
          cubemap_id, projection_dimension, original_dimension, shader_variables
        } => {
          let cube_mesh_draw_info = context.get_static_mesh(cube_mesh_id).ok_or(UnregisteredMesh(*cube_mesh_id))?;
          let shader_draw_info = context.get_shader(equirect_shader_id).ok_or(UnregisteredShader(*equirect_shader_id))?;
          let cubemap_draw_info = context.get_cubemap(&cubemap_id).ok_or(UnregisteredCubemap(*cubemap_id))?;

          let projection = cgmath::perspective(Deg(90.0f32), 1.0f32, 0.1f32, 10.0f32);
          let views = vec!(
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(1.0f32, 0.0, 0.0), -Vector3::unit_y()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(-1.0f32, 0.0, 0.0), -Vector3::unit_y()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 1.0, 0.0), Vector3::unit_z()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, -1.0, 0.0), -Vector3::unit_z()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, 1.0), -Vector3::unit_y()),
            cgmath::Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, -1.0), -Vector3::unit_y()),
          );
          let equirect_shader = shader_draw_info.shader;

          unsafe {
            let (framebuffer, _) = raw::create_framebuffer(projection_dimension.get_height(), projection_dimension.get_width());

            raw::use_shader(equirect_shader);
            raw::uniform1i(equirect_shader, "equirectangularMaps", 0);
            raw::matrix4f(equirect_shader, PROJECTION, projection.as_ptr());

            raw::active_texture(0);
            match projection_target {
              ProjectionTarget::Cubemap(c) => {
                let glid = context.get_cubemap(&c).ok_or(UnregisteredCubemap(*c))?.cubemap;
                raw::bind_cubemap(glid);
              }
              ProjectionTarget::Texture2d(c) => {
                let glid = context.get_texture(&c).ok_or(UnregisteredTexture(*c))?.texture;
                raw::bind_texture(glid);
              }
            };

            raw::set_viewport(projection_dimension.get_x(), projection_dimension.get_y(),
                              projection_dimension.get_width(), projection_dimension.get_height());
            raw::bind_framebuffer(framebuffer);
            for i in 0..6 {
              raw::matrix4f(equirect_shader, VIEW, views[i].as_ptr());
              raw::framebuffer_texture2d(i as u32, cubemap_draw_info.cubemap, 0);
              raw::clear_buffer();

              raw::bind_vao(cube_mesh_draw_info.vao);
              raw::draw_arrays(0, cube_mesh_draw_info.draw_size);
            }

            raw::bind_vao(0);
            // unbind framebuffer
            raw::bind_framebuffer(0);
            raw::set_viewport(0, 0, original_dimension.get_width(), original_dimension.get_height());
          }
        }
        Renderable::Skybox { skybox_shader_id, cube_mesh_id, rendered_cubemap_id, shader_variables, } => {
          let mesh_draw_info = context.get_static_mesh(&cube_mesh_id).ok_or(UnregisteredMesh(*cube_mesh_id))?;
          let shader_draw_info = context.get_shader(&skybox_shader_id).ok_or(UnregisteredShader(*skybox_shader_id))?;
          let rendered_cubemap_draw_info = context.get_cubemap(&rendered_cubemap_id).ok_or(UnregisteredCubemap(*rendered_cubemap_id))?;
          let camera_render_info = self.camera_render_info.as_ref().ok_or(NoCameraSet)?;

          unsafe {
            raw::use_shader(shader_draw_info.shader);
            raw::uniform1i(shader_draw_info.shader, "environmentMap", 0);
            raw::matrix4f(shader_draw_info.shader, VIEW, camera_render_info.view.as_ptr());
            raw::matrix4f(shader_draw_info.shader, PROJECTION, camera_render_info.projection.as_ptr());
            raw::active_texture(0);
            raw::bind_cubemap(rendered_cubemap_draw_info.cubemap);


            raw::bind_vao(mesh_draw_info.vao);
            match mesh_draw_info.ebo {
              None => raw::draw_arrays(0, mesh_draw_info.draw_size),
              Some(_) => raw::draw_elements(mesh_draw_info.draw_size),
            }
          }
        }
        _ => {}
      }
    }
    Ok(result)
  }
}

pub enum ProjectionTarget {
  Cubemap(Id),
  Texture2d(Id),
}

#[derive(Debug)]
pub enum RenderError {
  NoCameraSet,
  UnregisteredMesh(Id),
  UnregisteredShader(Id),
  UnregisteredTexture(Id),
  UnregisteredFramebuffer(Id),
  UnregisteredCubemap(Id),
}

pub enum RenderResult {}