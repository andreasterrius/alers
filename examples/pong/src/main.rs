use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_app::{ale_app_resource_path, ale_app_run, App};
use ale_camera::flycamera::FlyCamera;
use ale_camera::Camera;
use ale_input::Input;
use ale_math::rect::Rect;
use ale_math::transform::AleTransform;
use ale_math::{ale_bounding_box_size, Array, Vector3, Zero, Transform};
use ale_mesh::{ale_mesh_cube_new, Mesh};
use ale_opengl::old::opengl::{RenderContext, SimpleRenderTasks};
use ale_opengl::pbr::{
  ale_opengl_pbr_context_new, ale_opengl_pbr_render, ale_opengl_pbr_render_envmap, OpenGLPBRContext,
};
use ale_opengl::{ale_opengl_clear_render, ale_opengl_depth_test_enable};
use ale_physics::rapier3d::dynamics::{RigidBodyBuilder, RigidBodyHandle};
use ale_physics::rapier3d::na::Isometry3;
use ale_physics::rapier3d::na::Vector;
use ale_physics::{ale_physics_context_new, PhysicsContext, ale_physics_context_tick, ale_physics_context_collider_cuboid_new, ale_physics_context_update};
use ale_texture::ale_texture_load;
use ale_gltf::ale_gltf_load;

fn main() {
  ale_app_run(Pong, DisplayInfo::new(Rect::new(1024, 768)));
}

struct Pong;

struct Object {
  transform : AleTransform,
  rigidbody_handle : RigidBodyHandle,
  mesh : Mesh,
}

struct State {
  physics_context: PhysicsContext,
  opengl_pbr_context: OpenGLPBRContext,

  paddle: Object,
  fly_camera: FlyCamera,
}

impl App<State> for Pong {
  fn load(&mut self, context: &mut RenderContext, window: &Window) -> State {

    /*
     * Create the mesh
     */
    let paddle_mesh = ale_mesh_cube_new();
    let (_, ball_mesh) = ale_gltf_load(&ale_app_resource_path("gltf/bakso.gltf")).remove(0);

    /*
     * Physics setup
     */
    let mut physics_context = ale_physics_context_new();

    /*
     * Create Objects
     */
    let mut paddle_transform = AleTransform::from_scale(Vector3::new(0.2, 0.2, 1.0));
    let mut paddle_bb_half = (paddle_transform.matrix().transform_vector(ale_bounding_box_size(paddle_mesh.bounding_box))) / 2.0;
    let (rigidbody_handle, _) = ale_physics_context_collider_cuboid_new(&mut physics_context, paddle_bb_half);
    let mut paddle = Object {
      transform: paddle_transform,
      rigidbody_handle,
      mesh: paddle_mesh
    };

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
      ale_opengl_pbr_context_new(&hdr_texture, &window.get_display_info().dimension, vec![(&mut paddle.transform, &mut paddle.mesh)]);

    ale_opengl_depth_test_enable();

    State {
      opengl_pbr_context,
      physics_context,
      fly_camera,
      paddle
    }
  }

  fn input(&mut self, s: &mut State, inputs: Vec<Input>) {
    s.fly_camera.input(&inputs);
  }

  fn tick(&mut self, s: &mut State, delta_time: f32) {
    s.fly_camera.tick(delta_time);

    ale_physics_context_tick(&mut s.physics_context, delta_time);
    ale_physics_context_update(&mut s.physics_context, vec![(&mut s.paddle.transform, &s.paddle.rigidbody_handle)]);
  }

  fn render(&mut self, s: &mut State, render_tasks: SimpleRenderTasks, render_context: &mut RenderContext) {
    ale_opengl_clear_render();

    let camera_render_info = s.fly_camera.get_camera_render_info();

    ale_opengl_pbr_render_envmap(&s.opengl_pbr_context, &camera_render_info);
    ale_opengl_pbr_render(
      &s.opengl_pbr_context,
      vec![(&mut s.paddle.transform, &mut s.paddle.mesh)],
      &camera_render_info,
      &vec![],
    );
  }
}
