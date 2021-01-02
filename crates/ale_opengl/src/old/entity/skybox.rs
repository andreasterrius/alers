use crate::old::cubemap::CubemapId;
use ale_mesh::MeshId;
use ale_shader::ShaderId;

pub struct SkyboxEntity {
  pub static_mesh_id: MeshId,
  pub shader_id: ShaderId,
  pub rendered_cubemap_id: CubemapId,
  pub irradiance_cubemap_id: CubemapId,
}
