use ale_data::indexmap::Id;
use ale_window_winit::display::DisplaySetting;
use ale_window_winit::window::Window;

pub enum EngineCommand {
  CreateWindow(CreateWindowCommand)
}

pub struct CreateWindowCommand {
  pub(crate) id : Id<Window>,
  pub(crate) display_setting : DisplaySetting,
}

impl CreateWindowCommand {
  pub fn new(display_setting: DisplaySetting) -> (Id<Window>, CreateWindowCommand) {
    let id = Id::new();
    (id, CreateWindowCommand{ id , display_setting })
  }
}