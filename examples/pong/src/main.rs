use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_app::{ale_app_run, App};
use ale_camera::Camera;
use ale_input::Input;
use ale_math::rect::Rect;
use ale_math::{ale_bounding_box_size, Vector3, Zero};
use ale_mesh::ale_mesh_cube_new;
use ale_opengl::old::opengl::{RenderContext, SimpleRenderTasks};
use ale_physics::rapier3d::dynamics::RigidBodyBuilder;
use ale_physics::rapier3d::geometry::{ColliderBuilder, ColliderShape};
use ale_physics::rapier3d::na::Isometry3;
use ale_physics::{ale_physics_context_new, PhysicsContext};

fn main() {
  ale_app_run(Pong, DisplayInfo::new(Rect::new(1024, 768)));
}

struct Pong;

struct State {
  physics_context: PhysicsContext,
}

impl App<State> for Pong {
  fn load(&mut self, context: &mut RenderContext, window: &Window) -> State {
    let paddle = ale_mesh_cube_new();
    let paddle_size = ale_bounding_box_size(paddle.bounding_box) / 2.0;

    let mut physics_context = ale_physics_context_new();
    let mut camera = Camera::new(
      Vector3::new(0.0, 10.0, 0.0),
      window.get_display_info().dimension.clone(),
      90.0,
    );
    camera.look_at(Vector3::zero());

    let rigidbody = RigidBodyBuilder::new_dynamic().build();
    let rigidbody_handle = physics_context.bodies.insert(rigidbody);

    let collider = ColliderBuilder::new(ColliderShape::cuboid(paddle_size)).build();
    let collider_handle = physics_context
      .colliders
      .insert(collider, rigidbody_handle, &mut physics_context.bodies);

    State { physics_context }
  }

  fn input(&mut self, state: &mut State, inputs: Vec<Input>) {}

  fn tick(&mut self, s: &mut State, delta_time: f32) {}

  fn render(&mut self, s: &mut State, render_tasks: SimpleRenderTasks, render_context: &mut RenderContext) {}
}
