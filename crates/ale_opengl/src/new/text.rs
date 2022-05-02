use std::collections::HashMap;
use ale_camera::CameraRenderInfo;
use ale_font::{Font, FontTextureKey};
use ale_math::Vector2;
use ale_mesh::Mesh;
use ale_shader::Shader;
use crate::mesh::OpenGLMesh;
use crate::shader::OpenGLShader;
use crate::texture::OpenGLTexture;

pub struct TextRenderer {
  text_2d_shader: OpenGLShader,
  plane_opengl_mesh: OpenGLMesh,
  glyph_texture: HashMap<FontTextureKey, OpenGLTexture>,
}

impl TextRenderer {
  pub fn new() -> TextRenderer {
    let text_2d_shader = OpenGLShader::new(&Shader::new(
      include_str!("../../../../resources/shaders/text_2d.vert").to_owned(),
      include_str!("../../../../resources/shaders/text_2d.frag").to_owned(),
    ))
      .unwrap();

    let plane_opengl_mesh = OpenGLMesh::new(&Mesh::new_plane()).unwrap();

    TextRenderer {
      text_2d_shader,
      plane_opengl_mesh,
      glyph_texture: Default::default(),
    }
  }

  // pub fn render(
  //   &mut self,
  //   camera_render_info: &CameraRenderInfo,
  //   font: &mut Font,
  //   font_size: i32,
  //   origin: Vector2<f32>,
  //   text: &str,
  //   bounds: Option<Vector2<i32>>,
  // ) {
  //   let layout = ale_font_layout(font, font_size, text, bounds);
  //   ale_opengl_text_render_layout(opengl_text_font_context, camera_render_info, &layout, font, origin);
  // }
}