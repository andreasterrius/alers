use crate::raw;

use ale_autoid::*;
use ale_math::rect::Rect;

pub struct Cubemap {
  id: CubemapId,
  dimension: Rect,
}

impl Cubemap {
  pub fn new(dimension: Rect) -> Cubemap {
    Cubemap {
      id: CubemapId::new(),
      dimension,
    }
  }

  pub fn get_dimension(&self) -> &Rect {
    &self.dimension
  }
}

struct_id!(CubemapId);
struct_id_impl!(CubemapId, Cubemap, id);

pub struct CubemapDrawInfo {
  pub cubemap: u32,
}

impl CubemapDrawInfo {
  pub fn new(cubemap: &Cubemap) -> Result<CubemapDrawInfo, CubemapError> {
    let cubemap_internal = unsafe {
      raw::create_cubemap(
        cubemap.get_dimension().get_width(),
        cubemap.get_dimension().get_height(),
      )
    };
    Ok(CubemapDrawInfo {
      cubemap: cubemap_internal,
    })
  }
}

#[derive(Debug)]
pub struct CubemapError;
