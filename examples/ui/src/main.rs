use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_app::{ale_app_run, App, AppError};
use ale_camera::Camera;
use ale_input::{Input, Key};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::{Array, Vector2, Vector3, Zero};
use ale_opengl::{ale_opengl_blend_enable, ale_opengl_clear_render_color, ale_opengl_depth_test_enable};
use ale_opengl::renderer::sprite::SpriteRenderer;
use ale_opengl::renderer::text::TextRenderer;
use ale_resources::font::Font;
use ale_resources::path::ResourcePath;
use ale_resources::resources::Resources;
use ale_ui::button::Button;
use ale_ui::element::Element;
use ale_ui::layout::Layout;
use ale_ui::text::Text;
use ale_ui::ui;

struct UIApp;
struct UIState {
  resources: Resources,
  text_renderer: TextRenderer,
  sprite_renderer: SpriteRenderer,
  ui_root: ui::Root,
  camera: Camera,

}

fn main() {
  ale_app_run(UIApp, DisplayInfo::new(Rect::new(800, 600)));
}

impl App<UIState> for UIApp {
  fn load(&mut self, window: &Window) -> Result<UIState, AppError> {
    let mut resources = Resources::new();
    let font = resources.fonts.load(
      &ResourcePath::find("font/Inconsolata-Regular.ttf")).unwrap().remove(0);

    let mut ui_root = ui::Root::new();
    ui_root.add_element(Element::Text(Text::new(
      Vector2::zero(),
      String::from("some label"),
      font,
      12,
    )));
    ui_root.add_element(Element::Button(Button::new(
      Vector2::new(100.0, 100.0),
      Vector2::new(20.0, 30.0),
      Color::from_rgba(1.0, 0.0, 0.0, 1.0),
      Color::from_rgba(0.0, 1.0, 0.0, 1.0),
      Color::from_rgba(0.0, 0.0, 1.0, 1.0),
    )));

    let text_renderer = TextRenderer::new_with_resources(&mut resources)?;
    let sprite_renderer = SpriteRenderer::new_with_resource(&mut resources)?;

    let mut camera = Camera::new(Vector3::zero(), window.get_display_info().dimension.clone(), 90.0);
    camera.look_at(Vector3::zero());

    ale_opengl_depth_test_enable();
    ale_opengl_blend_enable();

    Ok(UIState {
      resources,
      text_renderer,
      sprite_renderer,
      ui_root,
      camera,
    })
  }

  fn input(&mut self, s: &mut UIState, inputs: Vec<Input>) {}

  fn fixed_tick(&mut self, s: &mut UIState, delta_time: f32) {}

  fn tick(&mut self, s: &mut UIState) {}

  fn render(&mut self, s: &mut UIState) {
    ale_opengl_clear_render_color(Color::from_rgb(0.0, 0.0, 0.0));

    {
      let mut layout = Layout::new(&mut s.text_renderer, &mut s.sprite_renderer,
                                   &mut s.resources, s.camera.camera_render_info());
      layout.render(&s.ui_root);
    }
  }
}
