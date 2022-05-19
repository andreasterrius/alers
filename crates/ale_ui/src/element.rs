use ale_camera::CameraRenderInfo;
use ale_data::alevec::AleVec;
use ale_input::Input;
use ale_math::rect::Rect;
use ale_math::{abs_diff_ne, Vector2, Vector3, Zero};
use ale_opengl::renderer::sprite::SpriteRenderer;
use ale_opengl::renderer::text::TextRenderer;
use ale_resources::resources::Resources;

use crate::button::Button;
use crate::layout::Layout;
use crate::text::Text;

pub enum Element {
  Panel(Panel),
  Button(Button),
  Text(Text),
}

pub struct Panel {
  pub (crate) layout_position : Vector2<i32>,
  pub (crate) layout_size : Vector2<u32>,
  pub (crate) layout : Layout,
  pub (crate) childs: AleVec<Element>,
  pub (crate) is_scrollable : bool,
}

impl Panel {
  pub fn new(layout : Layout) -> Panel {
    return Panel {
      layout_position: Vector2::zero(),
      layout_size: Vector2::zero(),
      layout,
      childs: AleVec::new(),
      is_scrollable: false
    };
  }

  pub fn add(&mut self, element: Element) {
    self.childs.push(element);
  }

  pub fn refresh_layout(&mut self, area : &Rect){

  }

  fn refresh_layout_intern(&mut self, area : &Rect) {
    for child in self.childs.iter_mut() {
      match child {
        Element::Panel(ele) => {}
        Element::Button(btn) => {}
        Element::Text(txt) => {}
      }
    }
  }

  pub fn input(&mut self, input: &Input) {
    for child in self.childs.iter_mut() {
      match child {
        Element::Panel(ele) => ele.input(input),
        Element::Button(btn) => btn.input(input),
        Element::Text(txt) => {}
      }
    }
  }

  pub fn render_with(&mut self, render_resources: &mut RenderResources) {
    for e in self.childs.iter_mut() {
      match e {
        Element::Panel(ele) => {
          ele.render_with(render_resources)
        },
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
