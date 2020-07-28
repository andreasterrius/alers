use crate::resource::mesh::MeshId;
use crate::resource::shader::ShaderFileId;
use crate::ui::UI;

pub struct UIEntity {
  pub ui: UI,
  pub mesh_id: MeshId,
  pub shader_id: ShaderFileId,
}
