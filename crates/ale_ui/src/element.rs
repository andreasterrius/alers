use crate::button::Button;
use crate::text::Text;
use ale_camera::CameraRenderInfo;
use ale_data::alevec::AleVec;
use ale_input::Input;
use ale_opengl::renderer::sprite::SpriteRenderer;
use ale_opengl::renderer::text::TextRenderer;
use ale_resources::resources::Resources;

pub enum Element {
  Elements(Elements),
  Button(Button),
  Text(Text),
}

pub struct Elements {
  pub (crate) childs: AleVec<Element>,
}

impl Elements {
  pub fn new() -> Elements {
    return Elements { childs: AleVec::new() };
  }

  pub fn add(&mut self, element: Element) {
    self.childs.push(element);
  }

  pub fn input(&mut self, input: &Input) {
    for child in self.childs.iter_mut() {
      match child {
        Element::Elements(ele) => ele.input(input),
        Element::Button(btn) => btn.input(input),
        Element::Text(txt) => {}
      }
    }
  }

  pub fn render_with(&mut self, render_resources: &mut RenderResources) {
    for e in self.childs.iter_mut() {
      match e {
        Element::Elements(ele) => ele.render_with(render_resources),
        Element::Text(text) => text.render_with(render_resources),
        Element::Button(button) => button.render_with(render_resources),
      }
    }
  }
}


pub struct RenderResources<'a> {
  pub text_renderer: &'a mut TextRenderer,
  pub sprite_renderer: &'a mut SpriteRenderer,
  pub resources: &'a mut Resources,
  pub camera_render_info: CameraRenderInfo,
}

impl<'a> RenderResources<'a> {
  pub fn new(
    text_renderer: &'a mut TextRenderer,
    sprite_renderer: &'a mut SpriteRenderer,
    resources: &'a mut Resources,
    camera_render_info: CameraRenderInfo,
  ) -> RenderResources<'a> {
    RenderResources {
      text_renderer,
      sprite_renderer,
      resources,
      camera_render_info,
    }
  }
}
