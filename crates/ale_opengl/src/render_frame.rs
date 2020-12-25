use crate::mesh::{OpenGLMesh, OpenGLMeshContext};
use crate::raw;
use crate::raw::{bind_framebuffer, create_framebuffer_texcolor_rbodepth, create_texture, OpenGLFramebufferId};
use crate::shader::{OpenGLShader, OpenGLShaderContext};
use crate::texture::OpenGLTextureId;
use ale_math::Vector2;
use ale_mesh::Mesh;
use ale_texture::TexturePixel;
use ale_variable::Variable;
use std::any::Any;
use std::ptr::null;

pub struct OpenGLRenderFrameContext {
  framebuffer: OpenGLFramebufferId,

  texture: OpenGLTextureId,

  plane_mesh: OpenGLMesh,
}

impl OpenGLRenderFrameContext {
  pub fn new(screen_size: Vector2<u32>) -> OpenGLRenderFrameContext {
    unsafe {
      let (fbo, rbo, texture) = create_framebuffer_texcolor_rbodepth(screen_size.x, screen_size.y).unwrap();
      let plane_mesh = OpenGLMesh::new(&Mesh::new_plane()).unwrap();

      OpenGLRenderFrameContext {
        framebuffer: fbo,
        texture,
        plane_mesh,
      }
    }
  }

  pub fn capture(&self, mut func: impl FnMut()) {
    unsafe {
      bind_framebuffer(self.framebuffer.0);

      func();

      bind_framebuffer(0);
    }
  }

  pub fn render(&self, opengl_shader: &OpenGLShader, opengl_shader_variables: &Vec<Variable>) {
    unsafe {
      raw::clear_buffer();

      let opengl_shader_framebuffer = &opengl_shader;
      let opengl_ndc_mesh_plane = &self.plane_mesh;

      opengl_shader.activate(opengl_shader_variables);

      raw::bind_vao(opengl_ndc_mesh_plane.vao);
      raw::active_texture(0);
      raw::bind_texture(self.texture.0);

      match opengl_ndc_mesh_plane.ebo {
        None => raw::draw_arrays(0, opengl_ndc_mesh_plane.draw_size),
        Some(_) => raw::draw_elements(opengl_ndc_mesh_plane.draw_size),
      }

      bind_framebuffer(0);
    }
  }
}
