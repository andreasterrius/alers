use ale_app::{ale_app_run, App};
use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_input::Input;
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_opengl::ale_opengl_clear_render_color;
use ale_ui::ui;

struct UIApp;
struct UIState {
    ui_root : ui::Root
}


fn main() {
    ale_app_run(UIApp, DisplayInfo::new(Rect::new(800, 600)));
}

impl App<UIState> for UIApp {
    fn load(&mut self, window: &Window) -> UIState {
        UIState{
            ui_root: ui::Root::new()
        }
    }

    fn input(&mut self, s: &mut UIState, inputs: Vec<Input>) {
    }

    fn fixed_tick(&mut self, s: &mut UIState, delta_time: f32) {}

    fn tick(&mut self, s: &mut UIState) {}

    fn render(&mut self, s: &mut UIState) {
        ale_opengl_clear_render_color(Color::from_rgb(1.0, 1.0, 1.0))
    }
}

