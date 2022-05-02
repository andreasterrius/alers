use crate::mesh::{OpenGLMesh};
use crate::raw;
use crate::raw::{bind_framebuffer, create_framebuffer_texcolor_rbodepth, OpenGLFramebufferId};
use crate::shader::{OpenGLShader};
use crate::texture::OpenGLTextureId;
use ale_math::Vector2;
use ale_mesh::Mesh;
use ale_variable::Variable;

pub struct OpenGLRenderFrameContext {
  framebuffer: OpenGLFramebufferId,

  texture: OpenGLTextureId,

  plane_mesh: OpenGLMesh,
}

pub fn ale_opengl_render_frame_new(screen_size: Vector2<u32>) -> OpenGLRenderFrameContext {
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

pub fn ale_opengl_render_frame_capture(opengl_render_frame_context: &OpenGLRenderFrameContext, mut func: impl FnMut()) {
  unsafe {
    bind_framebuffer(opengl_render_frame_context.framebuffer.0);

    func();

    bind_framebuffer(0);
  }
}

pub fn ale_opengl_render_frame_render(
  opengl_render_frame_context: &OpenGLRenderFrameContext,
  opengl_shader: &OpenGLShader,
  opengl_shader_variables: &Vec<Variable>,
) {
  unsafe {
    raw::clear_buffer(0.2f32, 0.3f32, 0.3f32);

    let opengl_shader_framebuffer = &opengl_shader;
    let opengl_ndc_mesh_plane = &opengl_render_frame_context.plane_mesh;

    opengl_shader_framebuffer.activate(opengl_shader_variables);

    raw::bind_vao(opengl_ndc_mesh_plane.vao);
    raw::active_texture(0);
    raw::bind_texture(opengl_render_frame_context.texture.0);

    match opengl_ndc_mesh_plane.ebo {
      None => raw::draw_arrays(0, opengl_ndc_mesh_plane.draw_size),
      Some(_) => raw::draw_elements(opengl_ndc_mesh_plane.draw_size),
    }

    bind_framebuffer(0);
  }
}
