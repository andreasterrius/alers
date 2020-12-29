use crate::mesh::{OpenGLMesh, OpenGLMeshContext};
use crate::raw;
use crate::raw::{create_texture, CreateTextureError};
use crate::resource_pile::{OpenGLResourceLoader, OpenGLResourcePile, OpenGLResourceType};
use crate::route_loader;
use crate::shader::{OpenGLShader, OpenGLShaderContext, OpenGLShaderId};
use ale_camera::{Camera, CameraRenderInfo};
use ale_font::{Font, FontTextureKey};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::{Matrix, Vector2};
use ale_texture::Texture;
use std::collections::HashMap;

pub struct OpenGLTextureId(pub u32);

pub struct OpenGLTexture {
  pub id: OpenGLTextureId,
  pub width: u32,
  pub height: u32,
}

impl OpenGLTexture {
  pub fn new(texture: &Texture) -> Result<OpenGLTexture, OpenGLTextureError> {
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

  pub fn render(
    &self,
    opengl_mesh_plane: &OpenGLMesh,
    opengl_shader_sprite: &OpenGLShader,
    position: Vector2<f32>,
    color: &Color,
    camera: &CameraRenderInfo,
  ) {
    unsafe {
      raw::bind_vao(opengl_mesh_plane.vao);

      raw::use_shader(opengl_shader_sprite.id);
      raw::active_texture(0);
      raw::bind_texture(self.id.0);
      raw::uniform4f(
        opengl_shader_sprite.id,
        "possize",
        position.x,
        position.y,
        self.width as f32,
        self.height as f32,
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

impl OpenGLResourceType for OpenGLTexture {}

pub struct OpenGLTextureLoader;

impl OpenGLResourceLoader<Texture, OpenGLTexture> for OpenGLTextureLoader {
  fn create(&self, opengl_resource_pile: &OpenGLResourcePile, before: &Texture) -> OpenGLTexture {
    OpenGLTexture::new(before).unwrap()
  }
}
route_loader!(OpenGLTextureLoader, Texture);
