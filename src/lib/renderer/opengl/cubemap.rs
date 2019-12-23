use crate::resource::cubemap::Cubemap;
use crate::renderer::opengl::raw;

pub struct CubemapDrawInfo {
  pub cubemap : u32,
}

impl CubemapDrawInfo {
  pub fn new(cubemap : &Cubemap) -> Result<CubemapDrawInfo, CubemapError> {
    let cubemap_internal = unsafe { raw::create_cubemap() };
    Ok(CubemapDrawInfo {
      cubemap: cubemap_internal
    })
  }
}

#[derive(Debug)]
pub struct CubemapError;