use crate::mesh::OpenGLMeshContext;
use crate::raw;
use crate::raw::{bind_framebuffer, create_framebuffer_texcolor_rbodepth, create_texture, OpenGLFramebufferId};
use crate::shader::{ale_opengl_shader_activate, OpenGLShaderContext};
use crate::texture::OpenGLTextureId;
use ale_math::Vector2;
use ale_texture::{ale_texture_new, TexturePixel};
use ale_variable::Variable;
use std::any::Any;
use std::ptr::null;

pub struct OpenGLRenderFrameContext {
  framebuffer: OpenGLFramebufferId,

  texture: OpenGLTextureId,
}

pub fn ale_opengl_render_frame_new(screen_size: Vector2<u32>) -> OpenGLRenderFrameContext {
  unsafe {
    let (fbo, rbo, texture) = create_framebuffer_texcolor_rbodepth(screen_size.x, screen_size.y).unwrap();

    OpenGLRenderFrameContext {
      framebuffer: fbo,
      texture,
    }
  }
}

pub fn ale_opengl_render_frame_capture(opengl_render_frame_context: &OpenGLRenderFrameContext, mut func: impl FnMut()) {
  unsafe {
    bind_framebuffer(opengl_render_frame_context.framebuffer.0);

    func();

    bind_framebuffer(0);
  }
}

pub fn ale_opengl_render_frame_render(
  opengl_render_frame_context: &OpenGLRenderFrameContext,
  opengl_shader_context: &OpenGLShaderContext,
  opengl_mesh_context: &OpenGLMeshContext,
  opengl_shader_variables: &Vec<Variable>,
) {
  unsafe {
    raw::clear_buffer();

    let opengl_shader_framebuffer = &opengl_shader_context.render_frame_shader;
    let opengl_ndc_mesh_plane = &opengl_mesh_context.plane_opengl_mesh;

    ale_opengl_shader_activate(opengl_shader_framebuffer, opengl_shader_variables);

    raw::bind_vao(opengl_ndc_mesh_plane.vao);
    raw::active_texture(0);
    raw::bind_texture(opengl_render_frame_context.texture.0);

    match opengl_ndc_mesh_plane.ebo {
      None => raw::draw_arrays(0, opengl_ndc_mesh_plane.draw_size),
      Some(_) => raw::draw_elements(opengl_ndc_mesh_plane.draw_size),
    }

    //bind_framebuffer(0);
  }
}
