use std::collections::HashMap;
use ale_camera::CameraRenderInfo;
use ale_data::alevec::AleVec;
use ale_input::Input;
use ale_math::rect::Rect;
use ale_math::{Vector2, Zero};
use ale_opengl::renderer::sprite::SpriteRenderer;
use ale_opengl::renderer::text::TextRenderer;
use ale_resources::resources::Resources;

use crate::button::Button;
use crate::empty::Empty;
use crate::layout::{Layout, LayoutError, LayoutType};
use crate::text::Text;

pub enum Element {
  Empty(Empty),
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

  pub fn new_root(layout_type: LayoutType, local_size: Vector2<u32>) -> Panel {
    return Panel {
      layout: Layout::new_local(Vector2::zero(), local_size),
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
        Element::Empty(empty) => child_layouts.push(&mut empty.layout),
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
        Element::Empty(empty) => {}
      }
    }
  }

  pub fn render_with(&mut self, render_resources: &mut RenderResources) {
    for e in self.childs.iter_mut() {
      match e {
        Element::Panel(ele) => ele.render_with(render_resources),
        Element::Text(text) => text.render_with(render_resources),
        Element::Button(button) => button.render_with(render_resources),
        Element::Empty(_) => {}
      }
    }
  }

  pub fn get_empty_layouts(&self) -> HashMap<String, &Empty> {
    let mut empties = HashMap::new();
    for e in self.childs.iter() {
      match e {
        Element::Panel(ele) => empties.extend(ele.get_empty_layouts()),
        Element::Empty(empty) => {
          empties.insert(empty.name.clone(), empty);
        }
        _ => {}
      }
    }
    return empties
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
