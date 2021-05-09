use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_app::{ale_app_resource_path, ale_app_run, App};
use ale_camera::flycamera::FlyCamera;
use ale_camera::Camera;
use ale_gltf::ale_gltf_load;
use ale_input::Input;
use ale_math::rect::Rect;
use ale_math::transform::AleTransform;
use ale_math::{ale_bounding_box_size, Array, Deg, Euler, Matrix4, Quaternion, Rotation3, Transform, Vector3, Zero};
use ale_mesh::{ale_mesh_cube_new, Mesh};
use ale_opengl::old::opengl::{RenderContext, SimpleRenderTasks};
use ale_opengl::pbr::{
  ale_opengl_pbr_context_new, ale_opengl_pbr_render, ale_opengl_pbr_render_envmap, OpenGLPBRContext,
};
use ale_opengl::{ale_opengl_clear_render, ale_opengl_depth_test_enable};
use ale_physics::rapier3d::dynamics::{RigidBodyBuilder, RigidBodyHandle};
use ale_physics::rapier3d::na::Isometry3;
use ale_physics::rapier3d::na::Vector;
use ale_physics::{
  ale_physics_context_cuboid_new, ale_physics_context_new, ale_physics_context_tick, ale_physics_context_update,
  PhysicsContext,
};
use ale_texture::ale_texture_load;

fn main() {
  ale_app_run(Pong, DisplayInfo::new(Rect::new(1024, 768)));
}

struct Pong;

struct Object {
  transform: AleTransform,
  rigidbody_handle: RigidBodyHandle,
  mesh: Mesh,
}

struct State {
  physics_context: PhysicsContext,
  opengl_pbr_context: OpenGLPBRContext,

  paddle: Object,
  floor: Object,
  fly_camera: FlyCamera,
}

impl App<State> for Pong {
  fn load(&mut self, context: &mut RenderContext, window: &Window) -> State {
    /*
     * Create the mesh
     */
    let paddle_mesh = ale_mesh_cube_new();
    let (_, ball_mesh) = ale_gltf_load(&ale_app_resource_path("gltf/bakso.gltf")).remove(0);
    let floor_mesh = ale_mesh_cube_new();

    /*
     * Physics setup
     */
    let mut physics_context = ale_physics_context_new();

    /*
     * Create Objects
     */
    let mut paddle_transform =
      AleTransform::from_position_scale(Vector3::new(0.0, 10.0, 0.0), Vector3::new(0.2, 0.2, 1.0));
    let mut paddle_bb = paddle_transform
      .scale_matrix()
      .transform_vector(ale_bounding_box_size(paddle_mesh.bounding_box));
    let (paddle_rigidbody_handle, _) = ale_physics_context_cuboid_new(
      &mut physics_context,
      paddle_transform.position,
      paddle_transform.lcl_rotation,
      paddle_bb / 2.0,
      true,
    );
    let mut paddle = Object {
      transform: paddle_transform,
      rigidbody_handle: paddle_rigidbody_handle,
      mesh: paddle_mesh,
    };

    // let mut floor_transform =
    //   AleTransform::from_position_scale(Vector3::new(0.0, -10.0, 0.0), Vector3::new(10.0, 2.0, 10.0));
    let mut floor_transform = AleTransform::from_all(
      Vector3::new(0.0, -10.0, 0.0),
      Quaternion::from_axis_angle(Vector3::unit_z(), Deg(30.0)),
      Vector3::new(10.0, 2.0, 10.0),
    );
    let mut floor_bb = floor_transform
      .scale_matrix()
      .transform_vector(ale_bounding_box_size(floor_mesh.bounding_box));
    let (floor_rigidbody_handle, _) = ale_physics_context_cuboid_new(
      &mut physics_context,
      floor_transform.position,
      floor_transform.lcl_rotation,
      floor_bb / 2.0, // extent is half
      false,
    );
    let mut floor = Object {
      transform: floor_transform,
      rigidbody_handle: floor_rigidbody_handle,
      mesh: floor_mesh,
    };

    /*
     * Camera Creation
     */
    let fly_camera = FlyCamera::new(Camera::new(
      Vector3::new(0.0, 0.0, 10.0),
      window.get_display_info().dimension.clone(),
      90.0,
    ));

    /*
     * Renderer
     */
    let hdr_texture = ale_texture_load(&ale_app_resource_path("hdr/GravelPlaza_Env.hdr")).unwrap();
    let opengl_pbr_context = ale_opengl_pbr_context_new(
      &hdr_texture,
      &window.get_display_info().dimension,
      vec![
        (&mut paddle.transform, &mut paddle.mesh),
        (&mut floor.transform, &mut floor.mesh),
      ],
    );

    ale_opengl_depth_test_enable();

    State {
      opengl_pbr_context,
      physics_context,
      fly_camera,
      paddle,
      floor,
    }
  }

  fn input(&mut self, s: &mut State, inputs: Vec<Input>) {
    s.fly_camera.input(&inputs);
  }

  fn tick(&mut self, s: &mut State, delta_time: f32) {
    s.fly_camera.tick(delta_time);

    ale_physics_context_tick(&mut s.physics_context, delta_time);
    ale_physics_context_update(
      &mut s.physics_context,
      vec![
        (&mut s.paddle.transform, &s.paddle.rigidbody_handle),
        (&mut s.floor.transform, &s.floor.rigidbody_handle),
      ],
    );
  }

  fn render(&mut self, s: &mut State, render_tasks: SimpleRenderTasks, render_context: &mut RenderContext) {
    ale_opengl_clear_render();

    let camera_render_info = s.fly_camera.get_camera_render_info();

    //ale_opengl_pbr_render_envmap(&s.opengl_pbr_context, &camera_render_info);
    ale_opengl_pbr_render(
      &s.opengl_pbr_context,
      vec![
        (&mut s.paddle.transform, &mut s.paddle.mesh),
        (&mut s.floor.transform, &mut s.floor.mesh),
      ],
      &camera_render_info,
      &vec![],
    );
  }
}
