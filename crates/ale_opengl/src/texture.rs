use crate::mesh::{OpenGLMesh, OpenGLMeshContext};
use crate::raw;
use crate::raw::{create_texture, CreateTextureError};
use crate::shader::{OpenGLShader, OpenGLShaderContext, OpenGLShaderId};
use ale_camera::CameraRenderInfo;
use ale_font::{ale_font_layout, Font, FontTextureKey};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::Matrix;
use ale_texture::Texture;
use std::collections::HashMap;

pub struct OpenGLTextureId(pub u32);

pub struct OpenGLTexture {
  pub opengl_texture_id: OpenGLTextureId,
}

pub fn ale_opengl_texture_new(texture: &Texture) -> Result<OpenGLTexture, OpenGLTextureError> {
  unsafe {
    create_texture(texture)
      .map(|id| OpenGLTexture { opengl_texture_id: id })
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

pub fn ale_opengl_text_render(
  opengl_texture_context: &mut OpenGLTextureContext,
  opengl_mesh_context: &OpenGLMeshContext,
  opengl_shader_context: &OpenGLShaderContext,
  font: &mut Font,
  font_size: i32,
  text: &str,
) {
  let layouts = ale_font_layout(font, font_size, text);

  for l in &layouts {
    opengl_texture_context
      .glyph_texture
      .entry(l.font_texture_key.clone())
      .or_insert_with(|| {
        let font_texture = match font.textures.get(&l.font_texture_key) {
          None => panic!("Unable to render, font glyph wasn't rasterized"),
          Some(ft) => ft,
        };

        unsafe { ale_opengl_texture_new(&font_texture.texture).unwrap() }
      });
  }
}
//
// pub fn ale_opengl_render_texture() {
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
//     raw::use_shader(shader_draw_info.shader);
//     raw::uniform4f(
//       shader_draw_info.shader,
//       "possize",
//       ui_render_info.rect.get_x() as f32,
//       ui_render_info.rect.get_y() as f32,
//       ui_render_info.rect.get_width() as f32,
//       ui_render_info.rect.get_height() as f32,
//     );
//     let (r, g, b, a) = ui_render_info.color.get_rgba();
//     raw::uniform4f(shader_draw_info.shader, "color", r, g, b, a);
//     raw::matrix4f(shader_draw_info.shader, VIEW, camera_render_info.view.as_ptr());
//     raw::matrix4f(
//       shader_draw_info.shader,
//       PROJECTION,
//       camera_render_info.orthographic.as_ptr(),
//     );
//     match mesh_draw_info.ebo {
//       None => raw::draw_arrays(0, mesh_draw_info.draw_size),
//       Some(_) => raw::draw_elements(mesh_draw_info.draw_size),
//     }
//   }
// }

fn intern_opengl_texture_render(
  opengl_mesh_plane: &OpenGLMesh,
  opengl_shader_sprite: &OpenGLShader,
  rect: &Rect,
  color: &Color,
  camera: &CameraRenderInfo,
) {
  unsafe {
    raw::bind_vao(opengl_mesh_plane.vao);
    raw::use_shader(opengl_shader_sprite.id);
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
