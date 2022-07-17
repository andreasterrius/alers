use std::collections::HashMap;

use thiserror::Error;

use ale_camera::CameraRenderInfo;
use ale_math::color::Color;
use ale_math::Vector2;
use ale_resources::font::{Font, FontLayout, FontTextureKey};
use ale_resources::mesh::Mesh;
use ale_resources::resources::Resources;
use ale_resources::shader;
use ale_resources::shader::Shader;

use crate::mesh::OpenGLMesh;
use crate::raw;
use crate::shader::OpenGLShader;
use crate::texture::OpenGLTexture;

pub struct TextRenderer {
  text_shader: OpenGLShader,
  plane_mesh: OpenGLMesh,
  glyph_texture: HashMap<FontTextureKey, OpenGLTexture>,
}

impl TextRenderer {
  pub fn new_with_resources(resources: &mut Resources) -> Result<TextRenderer, TextRendererError> {
    let text_shader_key = resources.shaders.stash.load("shaders/text_2d")?.remove(0);
    let plane_mesh_key = resources.meshes.register(Mesh::new_plane());

    TextRenderer::new(
      resources.shaders.stash.get(text_shader_key).unwrap(),
      resources.meshes.get(plane_mesh_key).unwrap(),
    )
  }

  pub fn new(text_shader: &Shader, plane_mesh: &Mesh) -> Result<TextRenderer, TextRendererError> {
    let text_shader = OpenGLShader::new(text_shader)?;
    let plane_mesh = OpenGLMesh::new(plane_mesh)?;

    Ok(TextRenderer {
      text_shader,
      plane_mesh,
      glyph_texture: Default::default(),
    })
  }

  pub fn  render(
    &mut self,
    camera_render_info: &CameraRenderInfo,
    font: &mut Font,
    font_size: u32,
    origin: Vector2<f32>,
    text: &str,
    bounds: Option<Vector2<i32>>,
  ) {
    let layout = font.layout(font_size, text, bounds);
    self.render_layout(camera_render_info, &layout, font, origin);
  }

  pub fn render_layout(
    &mut self,
    camera_render_info: &CameraRenderInfo,
    font_layout: &FontLayout,
    font: &Font,
    origin: Vector2<f32>,
  ) {
    for l in &font_layout.glyphs {
      self.glyph_texture.entry(l.font_texture_key.clone()).or_insert_with(|| {
        let font_texture = match font.textures.get(&l.font_texture_key) {
          None => panic!("Unable to render, font glyph wasn't rasterized"),
          Some(ft) => ft,
        };

        unsafe { OpenGLTexture::new(&font_texture.texture).unwrap() }
      });

      let opengl_texture = self.glyph_texture.get(&l.font_texture_key).unwrap();

      opengl_texture.render(
        &self.plane_mesh,
        &self.text_shader,
        origin + Vector2::new(l.offset_min.x as f32, l.offset_min.y as f32),
        &Color::from_rgb(1.0, 1.0, 1.0),
        &camera_render_info,
      );
    }
  }
}

#[derive(Error, Debug)]
pub enum TextRendererError {
  #[error("(TextRendererError::ShaderLoadError) {}", .0)]
  ShaderLoadError(#[from] shader::LoadError),
  #[error("(TextRendererError::OpenGLShaderError) {}", .0)]
  OpenGLShaderError(#[from] crate::shader::OpenGLShaderError),
  #[error("(TextRendererError::OpenGLMeshError) {}", .0)]
  OpenGLMeshError(#[from] crate::mesh::OpenGLMeshError),
}
