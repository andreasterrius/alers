use ale_camera::CameraRenderInfo;
use ale_opengl::renderer::sprite::SpriteRenderer;
use ale_opengl::renderer::text::TextRenderer;

use ale_resources::resources::Resources;

use crate::button::Button;
use crate::element::{Element, Elements};
use crate::text::Text;
use crate::ui;

pub struct Layout<'a> {
  text_renderer: &'a mut TextRenderer,
  sprite_renderer: &'a mut SpriteRenderer,
  resources: &'a mut Resources,
  camera_render_info: CameraRenderInfo,
}

impl<'a> Layout<'a> {
  pub fn new(
    text_renderer: &'a mut TextRenderer,
    sprite_renderer: &'a mut SpriteRenderer,
    resources: &'a mut Resources,
    camera_render_info: CameraRenderInfo,
  ) -> Layout<'a> {
    Layout {
      text_renderer,
      sprite_renderer,
      resources,
      camera_render_info,
    }
  }

  pub fn render(&mut self, root: &ui::Root) {
    self.render_elements(root.get_elements())
  }

  fn render_elements(&mut self, elements: &Elements) {
    for e in elements.0.iter() {
      match e {
        Element::Elements(ele) => self.render_elements(ele),
        Element::Text(text) => self.render_text(text),
        Element::Button(button) => self.render_button(button),
      }
    }
  }

  fn render_text(&mut self, text: &Text) {
    let font = match self.resources.fonts.get_mut(text.get_font()) {
      None => {
        return;
      }
      Some(font) => font,
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

  fn render_button(&mut self, button: &Button) {
    button.render_with(self.sprite_renderer, self.camera_render_info.orthographic);
  }
}
