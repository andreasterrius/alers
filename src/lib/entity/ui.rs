use crate::ui::UI;

pub struct UIEntity {
  id: UIEntityId,
  ui: UI,
}

struct_id!(UIEntityId);
struct_id_impl!(UIEntityId, UIEntity, id);

impl UIEntity {}
