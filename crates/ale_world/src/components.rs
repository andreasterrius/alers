use traitcast_core::TraitcastFrom as Component;


pub trait Tick: Component {
  fn fixed_tick(&mut self, delta_time: f32);
  fn tick(&mut self, delta_time: f32);
}

pub trait Input: Component {
  fn input(&mut self, input: ale_input::Input);
}

pub trait Id: Component {
  fn id(&mut self, id: u32);
}

pub trait Render: Component {
  fn render(&mut self) {}
}

pub trait Camera: Component {}