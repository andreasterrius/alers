use ale_camera::CameraRenderInfo;
use ale_data::alevec::AleVec;
use ale_input::Input;
use ale_math::rect::Rect;
use ale_math::Vector2;
use ale_opengl::renderer::sprite::SpriteRenderer;
use ale_opengl::renderer::text::TextRenderer;
use ale_resources::resources::Resources;

use crate::button::Button;
use crate::layout::{Layout, LayoutError, LayoutType};
use crate::text::Text;

pub enum Element {
  Panel(Panel),
  Button(Button),
  Text(Text),
}

pub struct Panel {
  pub(crate) layout: Layout,
  pub(crate) layout_type: LayoutType,
  pub(crate) childs: AleVec<Element>,
}

impl Panel {
  pub fn new(layout_type: LayoutType) -> Panel {
    return Panel {
      layout: Layout::new(),
      layout_type,
      childs: AleVec::new(),
    };
  }

  pub fn new_layout(layout_type: LayoutType, local_position: Vector2<i32>, local_size: Vector2<u32>) -> Panel {
    return Panel {
      layout: Layout::new_local(local_position, local_size),
      layout_type,
      childs: AleVec::new(),
    };
  }

  pub fn add(&mut self, element: Element) {
    self.childs.push(element);
  }

  pub fn resize(&mut self, new_size: Vector2<u32>) {
    self.layout.size = new_size;
    self.refresh_layout();
  }

  pub fn refresh_layout(&mut self) -> Result<(), LayoutError> {
    let mut child_layouts = vec![];
    for child in self.childs.iter_mut() {
      match child {
        Element::Panel(pnl) => {
          child_layouts.push(&mut pnl.layout);
        }
        Element::Button(btn) => child_layouts.push(&mut btn.layout),
        Element::Text(txt) => child_layouts.push(&mut txt.layout),
      }
    }

    self.layout_type.arrange(&self.layout, child_layouts)
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
        Element::Panel(ele) => ele.render_with(render_resources),
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
