use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_app::{ale_app_run, App, AppError};
use ale_camera::Camera;
use ale_data::alevec::Key;
use ale_input::{Input};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::{Array, Vector2, Vector3, Zero};
use ale_math::transform::AleTransform;
use ale_opengl::renderer::sprite::SpriteRenderer;
use ale_opengl::renderer::text::TextRenderer;
use ale_opengl::{ale_opengl_blend_enable, ale_opengl_clear_render_color, ale_opengl_depth_test_enable};
use ale_opengl::wire::MeshWireRenderer;
use ale_resources::font::Font;
use ale_resources::mesh::Mesh;
use ale_resources::path::ResourcePath;
use ale_resources::resources::Resources;
use ale_ui::button::Button;
use ale_ui::element;
use ale_ui::element::{Element, RenderResources};
use ale_ui::layout::{LayoutType, TableLayoutType};
use ale_ui::text::Text;

struct UIApp;
struct UIState {
  resources: Resources,
  text_renderer: TextRenderer,
  sprite_renderer: SpriteRenderer,
  mesh_wire_renderer: MeshWireRenderer,
  ui_elements: element::Panel,
  bakso : Key<Mesh>,
  camera: Camera,
}

fn main() {
  ale_app_run(
    UIApp,
    DisplayInfo::new(Rect {
      position: Vector2::zero(),
      size: Vector2::new(800, 600),
    }),
  );
}

impl App<UIState> for UIApp {
  fn load(&mut self, window: &Window) -> Result<UIState, AppError> {
    let mut resources = Resources::new();
    let font = resources
      .fonts
      .load(&ResourcePath::find("font/Inconsolata-Regular.ttf"))
      .unwrap()
      .remove(0);

    let bakso = resources.meshes.load("gltf/bakso.gltf")?.remove(0);

    let mut ui_elements = element::Panel::new_root(
      LayoutType::TableLayout(TableLayoutType::new_divider(
        vec![vec![0.7, 0.3], vec![0.2, 0.3, 0.5]],
        vec![0.7, 0.3],
      )),
      window.get_display_info().dimension.size,
    );
    ui_elements.add(Element::Text(Text::new(
      Vector2::new(300, 300),
      String::from("some label asdadsadas"),
      font,
      12,
    )));
    // ui_elements.add(Element::Button(Button::new_basic(Color::red())));
    // ui_elements.add(Element::Button(Button::new_basic(Color::green())));
    // ui_elements.add(Element::Button(Button::new_basic(Color::blue())));
    // ui_elements.add(Element::Button(Button::new_basic(Color::yellow())));
    ui_elements.refresh_layout()?;

    let text_renderer = TextRenderer::new_with_resources(&mut resources)?;
    let sprite_renderer = SpriteRenderer::new_with_resource(&mut resources)?;
    let mesh_wire_renderer = MeshWireRenderer::new_with_resource(&mut resources)?;

    let mut camera = Camera::new(Vector3::from_value(5.0f32), window.get_display_info().dimension.clone(), 90.0);
    camera.look_at(Vector3::zero());

    ale_opengl_depth_test_enable();
    ale_opengl_blend_enable();

    Ok(UIState {
      resources,
      text_renderer,
      sprite_renderer,
      mesh_wire_renderer,
      ui_elements,
      camera,
      bakso,
    })
  }

  fn input(&mut self, s: &mut UIState, inputs: Vec<Input>) {
    for input in &inputs {
      s.ui_elements.input(input)
    }
  }

  fn fixed_tick(&mut self, s: &mut UIState, delta_time: f32) {}

  fn tick(&mut self, s: &mut UIState) {}

  fn render(&mut self, s: &mut UIState) {
    ale_opengl_clear_render_color(Color::light_blue());

    // Render UI
    {
      let mut render_resources = RenderResources::new(
        &mut s.text_renderer,
        &mut s.sprite_renderer,
        &mut s.resources,
        s.camera.camera_render_info(),
      );
      s.ui_elements.render_with(&mut render_resources);
    }

    // Render Game Window
    {
      let bakso = s.resources.meshes.get(s.bakso).unwrap();
      s.mesh_wire_renderer.render_bounding_box(
        vec![(&mut AleTransform::new(), bakso)], &s.camera.camera_render_info());
    }
  }
}
