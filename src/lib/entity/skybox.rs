use crate::resource::shader::ShaderFile;
use crate::resource::static_mesh::StaticMesh;
use crate::resource::cubemap::Cubemap;
use crate::data::id::{Identifiable, Id};

pub struct SkyboxEntity {
  pub static_mesh_id : Id,
  pub shader_id : Id,
  pub cubemap_id : Id,
}
