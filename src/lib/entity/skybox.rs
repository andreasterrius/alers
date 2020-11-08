use crate::resource::cubemap::CubemapId;
use crate::resource::shader::ShaderFileId;
use ale_mesh::MeshId;

pub struct SkyboxEntity {
  pub static_mesh_id: MeshId,
  pub shader_id: ShaderFileId,
  pub rendered_cubemap_id: CubemapId,
  pub irradiance_cubemap_id: CubemapId,
}
