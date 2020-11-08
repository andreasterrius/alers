use crate::resource::shader::ShaderFileId;
use crate::ui::UI;
use ale_mesh::MeshId;

pub struct UIEntity {
  pub ui: UI,
  pub mesh_id: MeshId,
  pub shader_id: ShaderFileId,
}
