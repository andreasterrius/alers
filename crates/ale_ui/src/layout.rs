use crate::element::{Element, Elements};
use crate::text::Text;
use crate::ui;
use ale_camera::CameraRenderInfo;
use ale_opengl::text::TextRenderer;
use ale_resources::font::Font;
use ale_resources::resources::Resources;

pub struct Layout<'a> {
  text_renderer: &'a mut TextRenderer,
  resources: &'a mut Resources,
  camera_render_info: CameraRenderInfo,
}

impl <'a> Layout<'a> {
  pub fn new(
    text_renderer: &'a mut TextRenderer,
    resources: &'a mut Resources,
    camera_render_info: CameraRenderInfo,
  ) -> Layout<'a> {
    Layout { text_renderer, resources, camera_render_info }
  }

  pub fn render(&mut self, root: &ui::Root) {
    self.render_elements(root.get_elements())
  }

  fn render_elements(&mut self, elements: &Elements) {
    for e in elements.0.iter() {
      match e {
        Element::Elements(ele) => self.render_elements(ele),
        Element::Text(text) => self.render_text(text),
      }
    }
  }

  fn render_text(&mut self, text: &Text) {
    let font = match self.resources.fonts.get_mut(text.get_font()) {
      None => { return; }
      Some(font) => {font}
    };

    self.text_renderer.render(
      &self.camera_render_info,
      font,
      text.get_font_size(),
      text.get_pos(),
      &text.get_text(),
      None,
    )
  }
}
