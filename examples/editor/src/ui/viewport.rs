use std::collections::HashMap;

use ale_app::AppError;
use ale_camera::Camera;
use ale_camera::flycamera::FlyCamera;
use ale_data::alevec::{AleVec, Key};
use ale_math::{Vector2, Zero};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_ui::button::Button;
use ale_ui::element;
use ale_ui::element::Element;
use ale_ui::empty::Empty;
use ale_ui::layout::{Layout, LayoutType, TableLayoutType};
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_world::components::{Input, OnSpawn, Tick};
use ale_world::engine::Engine;
use ale_world::viewport::ViewportDescriptor;
use ale_world::wire_component;
use ale_world::world::{EntityKey, World};
use element::Panel;
use LayoutType::TableLayout;

const GAME_RENDER: &str = "game_render";

pub struct Viewport {
  main_viewport_key: Key<ViewportDescriptor>,
  test_viewport_key: Key<ViewportDescriptor>,
}

impl Viewport {
  pub fn new(engine: &mut Engine,
             editor_camera_key: EntityKey) -> Result<Viewport, AppError> {
    let main_window_key = engine.windows.add(
      DisplaySetting {
        dimension: Rect {
          position: Vector2::zero(),
          size: Vector2::new(800, 600),
        },
        initial_target: TargetMonitor::PRIMARY,
      });
    let sub_window_key = engine.windows.add(DisplaySetting {
      dimension: Rect {
        position: Vector2::zero(),
        size: Vector2::new(400, 300),
      },
      initial_target: TargetMonitor::PRIMARY,
    });

    let mut panel = element::Panel::new_root(
      LayoutType::TableLayout(TableLayoutType::new_divider(
        vec![vec![0.7, 0.3], vec![0.2, 0.3, 0.5]],
        vec![0.7, 0.3],
      )),
      Vector2::new(800, 600),
    );
    // ui_elements.add(Element::Text(Text::new(
    //   Vector2::new(300, 300),
    //   String::from("some label asdadsadas"),
    //   font,
    //   12,
    // )));
    panel.add(Element::Empty(Empty::new("game_render".to_owned())));
    panel.add(Element::Button(Button::new_basic(Color::red())));
    panel.add(Element::Button(Button::new_basic(Color::green())));
    panel.add(Element::Button(Button::new_basic(Color::blue())));
    panel.add(Element::Button(Button::new_basic(Color::yellow())));
    panel.refresh_layout()?;

    let panel_key = engine.panels.push(panel);

    let main_viewport_key = engine.viewport_descriptor.push(ViewportDescriptor::new(editor_camera_key, main_window_key));
    let test_viewport_key = engine.viewport_descriptor.push(ViewportDescriptor::with_panel_multi(
      HashMap::from([(GAME_RENDER.to_owned(), editor_camera_key)]), panel_key, sub_window_key));

    Ok(Viewport {
      main_viewport_key,
      test_viewport_key,
    })
  }
}

