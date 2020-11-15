use crate::ui::UI;
use ale_mesh::MeshId;
use ale_shader::ShaderId;

pub struct UIEntity {
  pub ui: UI,
  pub mesh_id: MeshId,
  pub shader_id: ShaderId,
}
