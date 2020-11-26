use crate::raw::{bind_framebuffer, create_framebuffer_postprocess, create_texture, framebuffer_texture2d, OpenGLFramebufferId};
use ale_math::Vector2;
use ale_texture::{ale_texture_new, TexturePixel};
use std::ptr::null;

pub struct OpenGLPostProcessContext {
  framebuffer: OpenGLFramebufferId,
}

pub fn ale_opengl_postprocess_new(screen_size: Vector2<u32>) -> OpenGLPostProcessContext {
  unsafe {
    let (fbo, rbo) = create_framebuffer_postprocess(screen_size.x, screen_size.y);
    let texture = ale_texture_new(TexturePixel::RgbU8Null, screen_size.x, screen_size.y, 3);
    let gl_texture = create_texture(&texture).unwrap();

    bind_framebuffer(fbo.0);
    framebuffer_texture2d(gl_texture.0);
  }

  OpenGLPostProcessContext { framebuffer: fbo }
}
