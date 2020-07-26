use crate::resource::cubemap::CubemapId;
use crate::resource::mesh::MeshId;
use crate::resource::shader::ShaderFileId;

pub struct SkyboxEntity {
  pub static_mesh_id: MeshId,
  pub shader_id: ShaderFileId,
  pub rendered_cubemap_id: CubemapId,
  pub irradiance_cubemap_id: CubemapId,
}
