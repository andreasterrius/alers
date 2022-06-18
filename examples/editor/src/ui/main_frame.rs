use ale_ui::element;
use ale_ui::layout::{Layout, LayoutType, TableLayoutType};
use ale_world::components::{Id, Input, Tick};
use ale_world::wire_component;
use ale_world::world::World;
use element::Panel;
use LayoutType::TableLayout;

pub struct MainFrame {
  id: u32,
  ui : Panel,
}

impl MainFrame {
  pub fn register_components(world: &mut World) {
    world.enable(&[
      wire_component!(dyn Tick, MainFrame),
      wire_component!(dyn Input, MainFrame),
      wire_component!(dyn Id, MainFrame),
    ])
  }

  pub fn new() -> MainFrame {
    let panel = Panel::new(
      TableLayout(TableLayoutType::new_divider(
        vec![vec![0.7, 0.3]], vec!(1.0),
      ))
    );

    MainFrame {
      id: 0,
      ui: panel
    }
  }
}

impl Tick for MainFrame {
  fn fixed_tick(&mut self, delta_time: f32) {

  }

  fn tick(&mut self, delta_time: f32) {

  }
}

impl Input for MainFrame {
  fn input(&mut self, input: ale_input::Input) {}
}

impl Id for MainFrame {
  fn id(&mut self, id: u32) {
    self.id = id;
  }
}
