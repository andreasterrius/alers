use crate::data::color::Color;
use crate::math::rect::Rect;
use crate::ui::{UIRenderInfo, UI};

pub struct Panel {
  rect: Rect,
  background_color: Color,

  is_hidden: bool,
}

impl Panel {
  pub fn new(rect: Rect, background_color: Color) -> Panel {
    Panel {
      rect,
      background_color,
      is_hidden: false,
    }
  }

  //  pub fn get_ui_render_info(&self) -> Vec<UIRenderInfo> {
  //    let mut render_infos = vec![];
  //
  //    if !self.is_hidden {
  //      for child in self.childs {
  //        render_infos.append(&mut child.get_ui_render_info());
  //      }
  //      render_infos.push(UIRenderInfo {
  //        rect: self.rect.clone(),
  //        color: self.background_color,
  //      })
  //    }
  //
  //    return render_infos;
  //  }
}
