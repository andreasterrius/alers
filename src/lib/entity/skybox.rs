use crate::data::id::{Id, Identifiable};
use crate::resource::cubemap::Cubemap;
use crate::resource::shader::ShaderFile;
use crate::resource::static_mesh::StaticMesh;

pub struct SkyboxEntity {
  pub static_mesh_id: Id,
  pub shader_id: Id,
  pub rendered_cubemap_id: Id,
  pub irradiance_cubemap_id: Id,
}
