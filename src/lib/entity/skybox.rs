use crate::data::id::Id;

pub struct SkyboxEntity {
  pub static_mesh_id: Id,
  pub shader_id: Id,
  pub rendered_cubemap_id: Id,
  pub irradiance_cubemap_id: Id,
}
