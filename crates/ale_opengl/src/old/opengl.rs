use std::collections::HashMap;

use crate::constant::{CAMERA_POSITION, MODEL, PROJECTION, VIEW};
use crate::mesh::{OpenGLMesh, OpenGLMeshError};
use crate::old::cubemap::{Cubemap, CubemapId};
use crate::old::cubemap::{CubemapDrawInfo, CubemapError};
use crate::old::opengl::RenderError::{
  NoCameraSet, UnregisteredCubemap, UnregisteredMesh, UnregisteredShader, UnregisteredTexture,
};
use crate::raw;
use crate::shader::{ale_opengl_shader_activate, ale_opengl_shader_new, OpenGLShader, OpenGLShaderError};
use crate::texture::{ale_opengl_texture_new, OpenGLTexture, OpenGLTextureError};
use ale_camera::CameraRenderInfo;
use ale_font::Font;
use ale_math::rect::Rect;
use ale_math::{perspective, Deg, EuclideanSpace, Matrix, Matrix4, Point3, Vector3};
use ale_mesh::{Mesh, MeshId};
use ale_shader::{Shader, ShaderId};
use ale_texture::{Texture, TextureId};
use ale_variable::Variable;

pub struct RenderResources {
  static_meshes: HashMap<MeshId, OpenGLMesh>,
  shaders: HashMap<ShaderId, OpenGLShader>,
  textures: HashMap<TextureId, OpenGLTexture>,

  //internal context
  //framebuffers: HashMap<Id, FramebufferDrawInfo>,
  cubemap: HashMap<CubemapId, CubemapDrawInfo>,
}

impl RenderResources {
  pub fn new() -> RenderResources {
    RenderResources {
      static_meshes: HashMap::new(),
      shaders: HashMap::new(),
      textures: HashMap::new(),
      //framebuffers: HashMap::new(),
      cubemap: HashMap::new(),
    }
  }

  pub fn static_mesh(&mut self, mesh: &Mesh) -> Result<(), OpenGLMeshError> {
    self.static_meshes.insert(mesh.uid(), OpenGLMesh::new(mesh)?);
    Ok(())
  }

  pub fn shader(&mut self, shader: &Shader) -> Result<(), OpenGLShaderError> {
    self.shaders.insert(shader.uid(), ale_opengl_shader_new(shader)?);
    Ok(())
  }

  pub fn texture(&mut self, texture: &Texture) -> Result<(), OpenGLTextureError> {
    self.textures.insert(texture.uid(), ale_opengl_texture_new(texture)?);
    Ok(())
  }

  //  pub fn framebuffer(&mut self) -> Result<Id, FramebufferError> {
  //    //let id = Id::new();
  //    //self.framebuffers.insert(id, FramebufferDrawInfo::new()?);
  //    Ok(id)
  //  }

  pub fn cubemap(&mut self, cubemap: &Cubemap) -> Result<(), CubemapError> {
    self.cubemap.insert(cubemap.uid(), CubemapDrawInfo::new(&cubemap)?);
    Ok(())
  }

  pub fn get_static_mesh(&self, mesh_id: &MeshId) -> Option<&OpenGLMesh> {
    self.static_meshes.get(&mesh_id)
  }

  pub fn get_shader(&self, shader_id: &ShaderId) -> Option<&OpenGLShader> {
    self.shaders.get(&shader_id)
  }

  pub fn get_texture(&self, texture_id: &TextureId) -> Option<&OpenGLTexture> {
    self.textures.get(texture_id)
  }

  //  pub fn get_framebuffer(&self, framebuffer: &Id) -> Option<&FramebufferDrawInfo> {
  //    self.framebuffers.get(&framebuffer)
  //  }

  pub fn get_cubemap(&self, cubemap: &CubemapId) -> Option<&CubemapDrawInfo> {
    self.cubemap.get(&cubemap)
  }
}

enum Renderable {
  StaticMesh {
    shader_id: ShaderId,
    mesh_id: MeshId,
    texture_ids: Vec<TextureId>,
    transform: Matrix4<f32>,
    shader_variables: Vec<Variable>,
  },

  EquirectCubemapProjection {
    equirect_shader_id: ShaderId,
    cube_mesh_id: MeshId,
    projection_target: ProjectionTarget,
    cubemap_id: CubemapId,
    projection_dimension: Rect,
    original_dimension: Rect,

    #[allow(dead_code)]
    shader_variables: Vec<Variable>,
  },

  Skybox {
    skybox_shader_id: ShaderId,
    cube_mesh_id: MeshId,
    rendered_cubemap_id: CubemapId,

    #[allow(dead_code)]
    shader_variables: Vec<Variable>,
  },
}

pub trait RenderTasks {
  fn with_camera(&mut self, camera_render_info: CameraRenderInfo);

  // fn queue_ui(&mut self, ui_shader_id: ShaderId, plane_mesh_id: MeshId, ui_render_info: UIRenderInfo);

  fn queue_static_mesh(
    &mut self,
    shader_id: ShaderId,
    mesh_id: MeshId,
    texture_ids: Vec<TextureId>,
    transform: Matrix4<f32>,
    shader_vars: Vec<Variable>,
  );

  fn queue_cubemap_projection(
    &mut self,
    equirect_shader_id: ShaderId,
    cube_mesh_id: MeshId,
    projection_target: ProjectionTarget,
    cubemap_id: CubemapId,
    projection_dimension: Rect,
    original_dimension: Rect,
    shader_variables: Vec<Variable>,
  );

  fn queue_skybox(
    &mut self,
    skybox_shader_id: ShaderId,
    cube_mesh_id: MeshId,
    rendered_cubemap_id: CubemapId,
    shader_variables: Vec<Variable>,
  );

  fn with_skybox(&mut self, cubemap_id: CubemapId);

  fn render(&mut self, context: &RenderResources) -> Result<Vec<RenderResult>, RenderError>;
}

pub struct SimpleRenderTasks {
  renderables: Vec<Renderable>,

  skybox: Option<CubemapId>,

  camera_render_info: Option<CameraRenderInfo>,
}

impl SimpleRenderTasks {
  pub fn new() -> SimpleRenderTasks {
    SimpleRenderTasks {
      renderables: vec![],
      skybox: None,
      camera_render_info: None {},
    }
  }
}

impl RenderTasks for SimpleRenderTasks {
  fn with_camera(&mut self, camera_render_info: CameraRenderInfo) {
    self.camera_render_info = Some(camera_render_info);
  }

  // fn queue_ui(&mut self, ui_shader_id: ShaderId, plane_mesh_id: MeshId, ui_render_info: UIRenderInfo) {
  //   self.renderables.push(Renderable::UIElement {
  //     ui_shader_id,
  //     plane_mesh_id,
  //     ui_render_info,
  //   });
  // }

  fn queue_static_mesh(
    &mut self,
    shader_id: ShaderId,
    mesh_id: MeshId,
    texture_ids: Vec<TextureId>,
    transform: Matrix4<f32>,
    shader_variables: Vec<Variable>,
  ) {
    self.renderables.push(Renderable::StaticMesh {
      shader_id,
      mesh_id,
      texture_ids,
      transform,
      shader_variables,
    });
  }

  fn queue_cubemap_projection(
    &mut self,
    equirect_shader_id: ShaderId,
    cube_mesh_id: MeshId,
    projection_target: ProjectionTarget,
    cubemap_id: CubemapId,
    projection_dimension: Rect,
    original_dimension: Rect,
    shader_variables: Vec<Variable>,
  ) {
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

  fn queue_skybox(
    &mut self,
    skybox_shader_id: ShaderId,
    cube_mesh_id: MeshId,
    rendered_cubemap_id: CubemapId,
    shader_variables: Vec<Variable>,
  ) {
    self.renderables.push(Renderable::Skybox {
      skybox_shader_id,
      cube_mesh_id,
      rendered_cubemap_id,
      shader_variables,
    });
  }

  fn with_skybox(&mut self, cubemap_id: CubemapId) {
    self.skybox = Some(cubemap_id);
  }

  fn render(&mut self, context: &RenderResources) -> Result<Vec<RenderResult>, RenderError> {
    let result = vec![];
    for renderable in &self.renderables {
      match renderable {
        Renderable::StaticMesh {
          shader_id,
          mesh_id,
          texture_ids,
          transform,
          shader_variables,
        } => {
          let mesh_draw_info = context.get_static_mesh(mesh_id).ok_or(UnregisteredMesh(*mesh_id))?;
          let shader_draw_info = context.get_shader(shader_id).ok_or(UnregisteredShader(*shader_id))?;
          let camera_render_info = self.camera_render_info.as_ref().ok_or(NoCameraSet)?;

          unsafe {
            // Bind shader
            ale_opengl_shader_activate(shader_draw_info, shader_variables);

            if let Some(cubemap_id) = &self.skybox {
              let irradiance_cubemap_draw_info = context
                .cubemap
                .get(cubemap_id)
                .ok_or(UnregisteredCubemap(*cubemap_id))?;
              raw::uniform1i(shader_draw_info.id, "irradianceMap", 0);
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
              raw::bind_texture(texture_draw_info.id.0);
            }

            let camera_position = camera_render_info.position;
            raw::uniform3f(
              shader_draw_info.id,
              CAMERA_POSITION,
              camera_position.x,
              camera_position.y,
              camera_position.z,
            );

            // Pass uniforms
            raw::matrix4f(shader_draw_info.id, MODEL, transform.as_ptr());
            raw::matrix4f(shader_draw_info.id, VIEW, camera_render_info.view.as_ptr());
            raw::matrix4f(shader_draw_info.id, PROJECTION, camera_render_info.projection.as_ptr());

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
          equirect_shader_id,
          cube_mesh_id,
          projection_target,
          cubemap_id,
          projection_dimension,
          original_dimension,
          shader_variables: _,
        } => {
          let cube_mesh_draw_info = context
            .get_static_mesh(cube_mesh_id)
            .ok_or(UnregisteredMesh(*cube_mesh_id))?;
          let shader_draw_info = context
            .get_shader(equirect_shader_id)
            .ok_or(UnregisteredShader(*equirect_shader_id))?;
          let cubemap_draw_info = context
            .get_cubemap(&cubemap_id)
            .ok_or(UnregisteredCubemap(*cubemap_id))?;

          let projection = perspective(Deg(90.0f32), 1.0f32, 0.1f32, 10.0f32);
          let views = vec![
            Matrix4::look_at(Point3::origin(), Point3::new(1.0f32, 0.0, 0.0), -Vector3::unit_y()),
            Matrix4::look_at(Point3::origin(), Point3::new(-1.0f32, 0.0, 0.0), -Vector3::unit_y()),
            Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 1.0, 0.0), Vector3::unit_z()),
            Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, -1.0, 0.0), -Vector3::unit_z()),
            Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, 1.0), -Vector3::unit_y()),
            Matrix4::look_at(Point3::origin(), Point3::new(0.0f32, 0.0, -1.0), -Vector3::unit_y()),
          ];
          let equirect_shader = shader_draw_info.id;

          unsafe {
            let (framebuffer, _) =
              raw::create_framebuffer_cubemap(projection_dimension.get_height(), projection_dimension.get_width());

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
                let glid = context.get_texture(&c).ok_or(UnregisteredTexture(*c))?.id.0;
                raw::bind_texture(glid);
              }
            };

            raw::set_viewport(
              projection_dimension.get_x(),
              projection_dimension.get_y(),
              projection_dimension.get_width(),
              projection_dimension.get_height(),
            );
            raw::bind_framebuffer(framebuffer.0);
            for i in 0..6 {
              raw::matrix4f(equirect_shader, VIEW, views[i].as_ptr());
              raw::framebuffer_texture2d_cubemap(i as u32, cubemap_draw_info.cubemap, 0);
              raw::clear_buffer(0.2f32, 0.3f32, 0.3f32);

              raw::bind_vao(cube_mesh_draw_info.vao);
              raw::draw_arrays(0, cube_mesh_draw_info.draw_size);
            }

            raw::bind_vao(0);
            // unbind framebuffer
            raw::bind_framebuffer(0);
            raw::set_viewport(0, 0, original_dimension.get_width(), original_dimension.get_height());
          }
        }
        Renderable::Skybox {
          skybox_shader_id,
          cube_mesh_id,
          rendered_cubemap_id,
          shader_variables: _,
        } => {
          let mesh_draw_info = context
            .get_static_mesh(&cube_mesh_id)
            .ok_or(UnregisteredMesh(*cube_mesh_id))?;
          let shader_draw_info = context
            .get_shader(&skybox_shader_id)
            .ok_or(UnregisteredShader(*skybox_shader_id))?;
          let rendered_cubemap_draw_info = context
            .get_cubemap(&rendered_cubemap_id)
            .ok_or(UnregisteredCubemap(*rendered_cubemap_id))?;
          let camera_render_info = self.camera_render_info.as_ref().ok_or(NoCameraSet)?;

          unsafe {
            raw::use_shader(shader_draw_info.id);
            raw::uniform1i(shader_draw_info.id, "environmentMap", 0);
            raw::matrix4f(shader_draw_info.id, VIEW, camera_render_info.view.as_ptr());
            raw::matrix4f(shader_draw_info.id, PROJECTION, camera_render_info.projection.as_ptr());
            raw::active_texture(0);
            raw::bind_cubemap(rendered_cubemap_draw_info.cubemap);

            raw::bind_vao(mesh_draw_info.vao);
            match mesh_draw_info.ebo {
              None => raw::draw_arrays(0, mesh_draw_info.draw_size),
              Some(_) => raw::draw_elements(mesh_draw_info.draw_size),
            }
          }
        }
        // Renderable::UIElement {
        //   ui_shader_id,
        //   plane_mesh_id,
        //   ui_render_info,
        // } => {
        //   let shader_draw_info = context
        //     .get_shader(&ui_shader_id)
        //     .ok_or(UnregisteredShader(*ui_shader_id))?;
        //   let mesh_draw_info = context
        //     .get_static_mesh(&plane_mesh_id)
        //     .ok_or(UnregisteredMesh(*plane_mesh_id))?;
        //   let camera_render_info = self.camera_render_info.as_ref().ok_or(NoCameraSet)?;
        //
        //   unsafe {
        //     raw::bind_vao(mesh_draw_info.vao);
        //     raw::use_shader(shader_draw_info.id);
        //     raw::uniform4f(
        //       shader_draw_info.id,
        //       "possize",
        //       ui_render_info.rect.get_x() as f32,
        //       ui_render_info.rect.get_y() as f32,
        //       ui_render_info.rect.get_width() as f32,
        //       ui_render_info.rect.get_height() as f32,
        //     );
        //     let (r, g, b, a) = ui_render_info.color.get_rgba();
        //     raw::uniform4f(shader_draw_info.id, "color", r, g, b, a);
        //     raw::matrix4f(shader_draw_info.id, VIEW, camera_render_info.view.as_ptr());
        //     raw::matrix4f(
        //       shader_draw_info.id,
        //       PROJECTION,
        //       camera_render_info.orthographic.as_ptr(),
        //     );
        //     match mesh_draw_info.ebo {
        //       None => raw::draw_arrays(0, mesh_draw_info.draw_size),
        //       Some(_) => raw::draw_elements(mesh_draw_info.draw_size),
        //     }
        //   }
        // }
        _ => {}
      }
    }
    Ok(result)
  }
}

pub enum ProjectionTarget {
  Cubemap(CubemapId),
  Texture2d(TextureId),
}

#[derive(Debug)]
pub enum RenderError {
  NoCameraSet,
  UnregisteredMesh(MeshId),
  UnregisteredShader(ShaderId),
  UnregisteredTexture(TextureId),
  //UnregisteredFramebuffer(Id),
  UnregisteredCubemap(CubemapId),
}

pub enum RenderResult {}
