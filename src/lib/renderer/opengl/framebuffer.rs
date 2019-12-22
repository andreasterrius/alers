use crate::renderer::opengl::raw;

pub struct FramebufferDrawInfo {
  pub framebuffer : u32,
}

impl FramebufferDrawInfo {
  pub fn new() -> Result<FramebufferDrawInfo, FramebufferError> {
    let (framebuffer, _) = unsafe { raw::create_framebuffer() };
    Ok(FramebufferDrawInfo { framebuffer })
  }
}

pub struct FramebufferError;