use ale_camera::Camera;
use ale_camera::flycamera::FlyCamera;
use ale_data::alevec::Key;
use ale_ui::element;
use ale_ui::layout::{Layout, LayoutType, TableLayoutType};
use ale_world::components::{Input, OnSpawn, Tick};
use ale_world::engine::Engine;
use ale_world::viewport::ViewportDescriptor;
use ale_world::wire_component;
use ale_world::world::{EntityId, World};
use element::Panel;
use LayoutType::TableLayout;

pub struct MainFrame {
  key: EntityKey,
  ui: Panel,

  camera_id: EntityKey,
}

impl MainFrame {
  pub fn register_components(world: &mut World) {
    world.enable(&[
      wire_component!(dyn Tick, MainFrame),
      wire_component!(dyn Input, MainFrame),
      wire_component!(dyn OnSpawn, MainFrame),
    ])
  }

  pub fn new(engine: &mut Engine,
             editor_camera_key: EntityKey) -> MainFrame {
    let panel = Panel::new(
      TableLayout(TableLayoutType::new_divider(
        vec![vec![0.7, 0.3]], vec!(1.0),
      ))
    );

    MainFrame {
      key: EntityKey::empty(),
      ui: panel,
      camera_id: (),
    }
  }
}

impl Tick for MainFrame {
  fn fixed_tick(&mut self, delta_time: f32) {}

  fn tick(&mut self, delta_time: f32) {}
}

impl Input for MainFrame {
  fn input(&mut self, input: ale_input::Input) {}
}

impl OnSpawn for MainFrame {
  fn take_key(&mut self, key: EntityKey) {
    self.key = key;
  }
}
