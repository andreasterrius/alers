use std::collections::HashMap;

use ale_app::engine::Engine;
use ale_app::AppError;
use ale_camera::flycamera::FlyCamera;
use ale_camera::Camera;
use ale_data::alevec::{AleVec, Key};
use ale_data::indexmap;
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::{Vector2, Zero};
use ale_ui::button::Button;
use ale_ui::element;
use ale_ui::element::Element;
use ale_ui::empty::Empty;
use ale_ui::layout::{Layout, LayoutType, TableLayoutType};
use ale_window::display::{DisplaySetting, TargetMonitor};
use ale_window::window::Window;
use ale_world::components::{Inputable, OnSpawn, Tickable};
use ale_world::wire_component;
use ale_world::world::{BoxEntity, World};
use element::Panel;
use LayoutType::TableLayout;

pub struct Viewport {
  main_window_key: Key<Window>,
  sub_window_key: Key<Window>,

  game_panel_key: Key<Element>,
}

impl Viewport {
  pub fn new(engine: &mut Engine) -> Result<Viewport, AppError> {
    let main_window_key = engine.windows.add(DisplaySetting {
      dimension: Rect {
        position: Vector2::zero(),
        size: Vector2::new(800, 600),
      },
      initial_target: TargetMonitor::PRIMARY,
      is_hidden: false
    });
    let sub_window_key = engine.windows.add(DisplaySetting {
      dimension: Rect {
        position: Vector2::zero(),
        size: Vector2::new(400, 300),
      },
      initial_target: TargetMonitor::PRIMARY,
      is_hidden: false
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
    let game_panel_key = panel.push(Element::Empty(Empty::new()));
    panel.push(Element::Button(Button::new_basic(Color::red())));
    panel.push(Element::Button(Button::new_basic(Color::green())));
    panel.push(Element::Button(Button::new_basic(Color::blue())));
    panel.push(Element::Button(Button::new_basic(Color::yellow())));
    panel.refresh_layout()?;

    let panel_key = engine.panels.push(panel);
    match engine.windows.get_mut(main_window_key) {
      None => {}
      Some(w) => {
        w.attach_panel(panel_key);
      }
    }

    Ok(Viewport {
      main_window_key,
      sub_window_key,
      game_panel_key,
    })
  }
}
