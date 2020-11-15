use crate::mesh::{OpenGLMesh, OpenGLMeshContext};
use crate::raw;
use crate::raw::{create_texture, CreateTextureError};
use crate::shader::{OpenGLShader, OpenGLShaderContext, OpenGLShaderId};
use ale_camera::{Camera, CameraRenderInfo};
use ale_font::{ale_font_layout, Font, FontTextureKey};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::Matrix;
use ale_texture::Texture;
use std::collections::HashMap;

pub struct OpenGLTextureId(pub u32);

pub struct OpenGLTexture {
  pub id: OpenGLTextureId,
}

pub fn ale_opengl_texture_new(texture: &Texture) -> Result<OpenGLTexture, OpenGLTextureError> {
  unsafe {
    create_texture(texture)
      .map(|id| OpenGLTexture { id: id })
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
  rect: &Rect,
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
      rect.get_x() as f32,
      rect.get_y() as f32,
      rect.get_width() as f32,
      rect.get_height() as f32,
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
