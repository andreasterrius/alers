use ale_camera::CameraRenderInfo;
use ale_console::{ale_console_variable_event_handle, ale_console_variable_register, Console};
use ale_math::transform::AleTransform;
use ale_math::{Array, Matrix, Transform};
use ale_resources::mesh::Mesh;
use ale_resources::resources::Resources;
use ale_resources::shader;
use ale_resources::shader::GLSLShader;
use ale_variable::{to_variable, ToVariable};
use thiserror::Error;

use crate::mesh::{OpenGLMesh, OpenGLMeshError};
use crate::raw;
use crate::shader::{OpenGLShader, OpenGLShaderError};

pub struct MeshWireRenderer {
  pub bounding_box_mesh: OpenGLMesh,

  pub bounding_box_shader: OpenGLShader,

  pub wire_render_enable: bool,
  // from 0 to 1
  pub wire_thickness: f32,
}

impl MeshWireRenderer {
  pub fn new_with_resource(resources: &mut Resources) -> Result<MeshWireRenderer, MeshWireRendererError> {
    let wire = resources.shaders.glsl_stash.load("shaders/wire")?.remove(0);
    MeshWireRenderer::new(resources.shaders.glsl_stash.get(wire).unwrap())
  }

  pub fn new(wire_shader: &GLSLShader) -> Result<MeshWireRenderer, MeshWireRendererError> {
    Ok(MeshWireRenderer {
      bounding_box_mesh: OpenGLMesh::new(&Mesh::new_bounding_box())?,
      bounding_box_shader: OpenGLShader::new(&wire_shader)?,
      wire_render_enable: true,
      wire_thickness: 0.01,
    })
  }

  pub fn register_console_variable(&self, console: &mut Console) {
    ale_console_variable_register(console, to_variable!(self.wire_render_enable));
    ale_console_variable_register(console, to_variable!(self.wire_thickness));
  }

  pub fn refresh_console_variable(&mut self, console: &mut Console) {
    self.wire_render_enable =
        ale_console_variable_event_handle(console, to_variable!(self.wire_render_enable));
    self.wire_thickness =
        ale_console_variable_event_handle(console, to_variable!(self.wire_thickness));
  }

  pub fn render_bounding_box(
    &mut self,
    meshes: Vec<(&mut AleTransform, &Mesh)>,
    camera_render_info: &CameraRenderInfo,
  ) {
    if !self.wire_render_enable {
      return;
    }

    let shader = &self.bounding_box_shader;
    shader.activate(&vec![to_variable!(self.wire_thickness)]);
    unsafe {
      raw::matrix4f(shader.id, "view", camera_render_info.view.as_ptr());
      raw::matrix4f(shader.id, "projection", camera_render_info.projection.as_ptr());

      for (mut transform, mesh) in meshes {
        let mut bb_transform = mesh.bounding_box_matrix();
        bb_transform = bb_transform.concat(&transform.matrix_cache());

        raw::matrix4f(shader.id, "model", bb_transform.as_ptr());
        raw::bind_vao(self.bounding_box_mesh.vao);
        match self.bounding_box_mesh.ebo {
          None => raw::draw_arrays(0, self.bounding_box_mesh.draw_size),
          Some(_) => raw::draw_elements(self.bounding_box_mesh.draw_size),
        }
      }
    }
  }
}


#[derive(Debug, Error)]
pub enum MeshWireRendererError {
  #[error("(MeshWireRendererError::OpenGLMeshError), {}", .0)]
  OpenGLMeshError(#[from] OpenGLMeshError),
  #[error("(MeshWireRendererError::OpenGLShaderError), {}", .0)]
  OpenGLShaderError(#[from] OpenGLShaderError),
  #[error("(MeshWireRendererError::LoadShaderError), {}", .0)]
  ShaderLoadError(#[from] shader::LoadError),
}
