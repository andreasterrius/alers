use crate::mesh::{OpenGLMesh};
use crate::raw;
use crate::raw::{create_texture, CreateTextureError};
use crate::shader::{OpenGLShader};
use ale_camera::{CameraRenderInfo};
use ale_font::{FontTextureKey};
use ale_math::color::Color;
use ale_math::{Matrix, Vector2};
use ale_texture::Texture;
use std::collections::HashMap;

pub struct OpenGLTextureId(pub u32);

pub struct OpenGLTexture {
  pub id: OpenGLTextureId,
  pub width: u32,
  pub height: u32,
}

pub fn ale_opengl_texture_new(texture: &Texture) -> Result<OpenGLTexture, OpenGLTextureError> {
  unsafe {
    create_texture(texture)
      .map(|id| OpenGLTexture {
        id,
        width: texture.width,
        height: texture.height,
      })
      .map_err(|err| OpenGLTextureError::from(err))
  }
}

pub struct OpenGLTextureContext {
  pub(crate) glyph_texture: HashMap<FontTextureKey, OpenGLTexture>,
}

pub fn ale_opengl_texture_context_new() -> OpenGLTextureContext {
  OpenGLTextureContext {
    glyph_texture: HashMap::new(),
  }
}

pub fn ale_opengl_texture_render(
  opengl_mesh_plane: &OpenGLMesh,
  opengl_shader_sprite: &OpenGLShader,
  opengl_texture: &OpenGLTexture,
  position: Vector2<f32>,
  color: &Color,
  camera: &CameraRenderInfo,
) {
  unsafe {
    raw::bind_vao(opengl_mesh_plane.vao);

    raw::use_shader(opengl_shader_sprite.id);
    raw::active_texture(0);
    raw::bind_texture(opengl_texture.id.0);
    raw::uniform4f(
      opengl_shader_sprite.id,
      "possize",
      position.x,
      position.y,
      opengl_texture.width as f32,
      opengl_texture.height as f32,
    );

    let (r, g, b, a) = color.get_rgba();
    raw::matrix4f(opengl_shader_sprite.id, "view", camera.view.as_ptr());
    raw::matrix4f(opengl_shader_sprite.id, "projection", camera.orthographic.as_ptr());

    match opengl_mesh_plane.ebo {
      None => raw::draw_arrays(0, opengl_mesh_plane.draw_size),
      Some(_) => raw::draw_elements(opengl_mesh_plane.draw_size),
    }
  }
}

#[derive(Debug)]
pub enum OpenGLTextureError {
  CreateTextureError(CreateTextureError),
}

impl From<CreateTextureError> for OpenGLTextureError {
  fn from(e: CreateTextureError) -> Self {
    OpenGLTextureError::CreateTextureError(e)
  }
}
