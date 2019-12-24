use crate::renderer::opengl::raw;
use crate::resource::cubemap::Cubemap;

pub struct CubemapDrawInfo {
  pub cubemap: u32,
}

impl CubemapDrawInfo {
  pub fn new(cubemap: &Cubemap) -> Result<CubemapDrawInfo, CubemapError> {
    let cubemap_internal = unsafe {
      raw::create_cubemap(cubemap.get_dimension().get_width(),
                          cubemap.get_dimension().get_height())
    };
    Ok(CubemapDrawInfo {
      cubemap: cubemap_internal
    })
  }
}

#[derive(Debug)]
pub struct CubemapError;