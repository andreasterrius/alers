use thiserror::Error;

use crate::constant::{COLOR, POSITION_SIZE, PROJECTION};
use ale_math::color::Color;
use ale_math::{Array, Matrix4, Vector2, Vector4};
use ale_resources::mesh::Mesh;
use ale_resources::resources::Resources;
use ale_resources::shader;
use ale_resources::shader::GLSLShader;
use ale_variable::Variable;

use crate::mesh::{OpenGLMesh, OpenGLMeshError};
use crate::raw;
use crate::raw::print_error;
use crate::shader::{OpenGLShader, OpenGLShaderError};

pub struct SpriteRenderer {
  plane_mesh: OpenGLMesh,
  flat_shader: OpenGLShader,
}

impl SpriteRenderer {
  pub fn new_with_resource(resources: &mut Resources) -> Result<SpriteRenderer, SpriteRendererError> {
    let plane_mesh_key = resources.meshes.register(Mesh::new_plane());
    let flat_shader_key = resources.shaders.glsl_stash.load("shaders/ui")?.remove(0);


    SpriteRenderer::new(
      resources.meshes.get(plane_mesh_key).unwrap(),
      resources.shaders.glsl_stash.get(flat_shader_key).unwrap(),
    )
  }

  pub fn new(plane_mesh: &Mesh, flat_shader: &GLSLShader) -> Result<SpriteRenderer, SpriteRendererError> {
    let plane_mesh = OpenGLMesh::new(plane_mesh)?;
    let flat_shader = OpenGLShader::new(flat_shader)?;

    Ok(SpriteRenderer {
      plane_mesh,
      flat_shader,
    })
  }

  pub fn render_flat_box(&self, position: Vector2<f32>, size: Vector2<f32>, color: Color, ortho: Matrix4<f32>) {
    self.plane_mesh.activate();
    raw::print_error("test");
    self.flat_shader.activate(&vec![
      Variable::F32_4(
        POSITION_SIZE.to_owned(),
        Vector4::new(position.x, position.y, size.x, size.y),
      ),
      Variable::F32_4(COLOR.to_owned(), Vector4::new(color.r, color.g, color.b, color.a)),
      Variable::F32_4_4(PROJECTION.to_owned(), ortho),
    ]);
    self.plane_mesh.draw();
  }

  pub fn render_flat_box_border(
    &self,
    position: Vector2<f32>,
    size: Vector2<f32>,
    inside_color: Color,
    thickness: f32,
    border_color: Color,
    ortho: Matrix4<f32>,
  ) {
    self.render_flat_box(position, size, border_color, ortho);

    let half_thickness = thickness / 2.0;
    let inside_pos = position + Vector2::from_value(half_thickness);
    let inside_size = size - Vector2::from_value(thickness);

    self.render_flat_box(inside_pos, inside_size, inside_color, ortho);
  }
}

#[derive(Error, Debug)]
pub enum SpriteRendererError {
  #[error("(SpriteRendererError::MeshError) {}", .0)]
  MeshError(#[from] OpenGLMeshError),
  #[error("(SpriteRendererError::ShaderError) {}", .0)]
  ShaderError(#[from] OpenGLShaderError),
  #[error("(SpriteRendererError::ShaderLoadError) {}", .0)]
  ShaderLoadError(#[from] shader::LoadError),
}
