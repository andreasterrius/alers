use ale_opengl::raw;

pub struct FramebufferDrawInfo {
  pub framebuffer: u32,
}

impl FramebufferDrawInfo {
  pub fn new(w: u32, h: u32) -> Result<FramebufferDrawInfo, FramebufferError> {
    let (framebuffer, _) = unsafe { raw::create_framebuffer(w, h) };
    Ok(FramebufferDrawInfo { framebuffer })
  }
}

pub struct FramebufferError;
