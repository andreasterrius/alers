use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_app::{ale_app_resource_path, ale_app_run, App};
use ale_camera::flycamera::FlyCamera;
use ale_camera::Camera;
use ale_input::Input;
use ale_math::rect::Rect;
use ale_math::transform::Transform;
use ale_math::{ale_bounding_box_size, Array, Vector3, Zero};
use ale_mesh::{ale_mesh_cube_new, Mesh};
use ale_opengl::old::opengl::{RenderContext, SimpleRenderTasks};
use ale_opengl::pbr::{
  ale_opengl_pbr_context_new, ale_opengl_pbr_render, ale_opengl_pbr_render_envmap, OpenGLPBRContext,
};
use ale_opengl::{ale_opengl_clear_render, ale_opengl_depth_test_enable};
use ale_physics::rapier3d::dynamics::RigidBodyBuilder;
use ale_physics::rapier3d::geometry::{ColliderBuilder, ColliderShape};
use ale_physics::rapier3d::na::Isometry3;
use ale_physics::rapier3d::ncollide::na::Vector;
use ale_physics::{ale_physics_context_new, PhysicsContext};
use ale_texture::ale_texture_load;

fn main() {
  ale_app_run(Pong, DisplayInfo::new(Rect::new(1024, 768)));
}

struct Pong;

struct State {
  opengl_pbr_context: OpenGLPBRContext,

  paddle0: (Transform, Mesh),
  physics_context: PhysicsContext,

  fly_camera: FlyCamera,
}

impl App<State> for Pong {
  fn load(&mut self, context: &mut RenderContext, window: &Window) -> State {
    /*
     * Create the paddles
     */
    let paddle_mesh = ale_mesh_cube_new();
    let paddle_mesh_size = ale_bounding_box_size(paddle_mesh.bounding_box) / 2.0;
    let mut paddle0 = (Transform::new(), paddle_mesh);

    /*
     * Physics setup
     */
    let mut physics_context = ale_physics_context_new();

    let rigidbody = RigidBodyBuilder::new_dynamic().build();
    let rigidbody_handle = physics_context.bodies.insert(rigidbody);

    let k = ale_physics::rapier3d::na::Vector3::new(paddle_mesh_size.x, paddle_mesh_size.y, paddle_mesh_size.z);

    let collider = ColliderBuilder::new(ColliderShape::cuboid(k)).build();
    let collider_handle = physics_context
      .colliders
      .insert(collider, rigidbody_handle, &mut physics_context.bodies);

    /*
     * Camera Creation
     */
    let fly_camera = FlyCamera::new(Camera::new(
      Vector3::from_value(0.0),
      window.get_display_info().dimension.clone(),
      90.0,
    ));

    /*
     * Renderer
     */
    let hdr_texture = ale_texture_load(&ale_app_resource_path("hdr/GravelPlaza_Env.hdr")).unwrap();
    let opengl_pbr_context =
      ale_opengl_pbr_context_new(&hdr_texture, &window.get_display_info().dimension, vec![&mut paddle0]);

    ale_opengl_depth_test_enable();

    State {
      paddle0,
      opengl_pbr_context,
      physics_context,
      fly_camera,
    }
  }

  fn input(&mut self, s: &mut State, inputs: Vec<Input>) {
    s.fly_camera.input(&inputs);
  }

  fn tick(&mut self, s: &mut State, delta_time: f32) {
    s.fly_camera.tick(delta_time);
  }

  fn render(&mut self, s: &mut State, render_tasks: SimpleRenderTasks, render_context: &mut RenderContext) {
    ale_opengl_clear_render();

    let camera_render_info = s.fly_camera.get_camera_render_info();

    ale_opengl_pbr_render_envmap(&s.opengl_pbr_context, &camera_render_info);
    ale_opengl_pbr_render(
      &s.opengl_pbr_context,
      vec![&mut s.paddle0],
      &camera_render_info,
      &vec![],
    );
  }
}
