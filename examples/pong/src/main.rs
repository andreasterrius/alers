use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_app::{ale_app_resource_path, ale_app_run, App};
use ale_camera::flycamera::FlyCamera;
use ale_camera::Camera;
use ale_gltf::ale_gltf_load;
use ale_input::Input;
use ale_math::rect::Rect;
use ale_math::transform::AleTransform;
use ale_math::{
  ale_bounding_box_size, ale_quaternion_look_at, Array, Deg, Euler, Matrix4, Quaternion, Rotation3, Transform, Vector3,
  Zero,
};
use ale_mesh::{ale_mesh_cube_new, Mesh};
use ale_opengl::old::opengl::{RenderContext, SimpleRenderTasks};
use ale_opengl::pbr::{
  ale_opengl_pbr_context_new, ale_opengl_pbr_render, ale_opengl_pbr_render_envmap, OpenGLPBRContext,
};
use ale_opengl::{ale_opengl_clear_render, ale_opengl_depth_test_enable};
use ale_physics::rapier3d::dynamics::{RigidBodyBuilder, RigidBodyHandle};
use ale_physics::{
  ale_physics_context_cuboid_new, ale_physics_context_new, ale_physics_context_tick, ale_physics_context_update,
  PhysicsContext, RigidBodyType,
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
  color: Vector3<f32>,
}

fn ale_create_pong_object(
  physics_context: &mut PhysicsContext,
  mut transform: AleTransform,
  mesh: Mesh,
  rigidbody_type: RigidBodyType,
  color: Vector3<f32>,
) -> Object {
  let mut bb = transform
    .scale_matrix()
    .transform_vector(ale_bounding_box_size(mesh.bounding_box));
  let (rigidbody_handle, _) = ale_physics_context_cuboid_new(
    physics_context,
    transform.position,
    transform.lcl_rotation,
    bb / 2.0,
    rigidbody_type,
    false,
  );
  Object {
    transform,
    rigidbody_handle,
    mesh,
    color,
  }
}

struct State {
  physics_context: PhysicsContext,
  opengl_pbr_context: OpenGLPBRContext,

  paddle_left: Object,
  paddle_right: Object,
  floor: Object,
  fly_camera: FlyCamera,
}

impl App<State> for Pong {
  fn load(&mut self, context: &mut RenderContext, window: &Window) -> State {
    /*
     * Create the mesh
     */
    let paddle_left_mesh = ale_mesh_cube_new();
    let paddle_right_mesh = ale_mesh_cube_new();
    let (_, ball_mesh) = ale_gltf_load(&ale_app_resource_path("gltf/bakso.gltf")).remove(0);
    let floor_mesh = ale_mesh_cube_new();

    /*
     * Physics setup
     */
    let mut physics_context = ale_physics_context_new();

    /*
     * Create Objects
     */
    let mut paddle_left = ale_create_pong_object(
      &mut physics_context,
      AleTransform::from_position_scale(Vector3::new(10.0, 0.0, 0.0), Vector3::new(0.2, 0.2, 1.0)),
      paddle_left_mesh,
      RigidBodyType::Kinematic,
      Vector3::new(0.0, 1.0, 0.0),
    );

    let mut paddle_right = ale_create_pong_object(
      &mut physics_context,
      AleTransform::from_position_scale(Vector3::new(-10.0, 0.0, 0.0), Vector3::new(0.2, 0.2, 1.0)),
      paddle_right_mesh,
      RigidBodyType::Kinematic,
      Vector3::new(1.0, 0.0, 0.0),
    );

    let mut floor = ale_create_pong_object(
      &mut physics_context,
      AleTransform::from_position_scale(Vector3::new(0.0, -3.0, 0.0), Vector3::new(10.02, 1.0, 10.0)),
      floor_mesh,
      RigidBodyType::Kinematic,
      Vector3::new(1.0, 1.0, 1.0),
    );

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
      vec![&mut paddle_left.mesh, &mut paddle_right.mesh, &mut floor.mesh],
    );

    ale_opengl_depth_test_enable();

    State {
      opengl_pbr_context,
      physics_context,
      fly_camera,
      paddle_left,
      paddle_right,
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
        (&mut s.paddle_left.transform, &s.paddle_left.rigidbody_handle),
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
        (
          &mut s.paddle_left.transform,
          &mut s.paddle_left.mesh,
          &s.paddle_left.color,
        ),
        (
          &mut s.paddle_right.transform,
          &mut s.paddle_right.mesh,
          &s.paddle_right.color,
        ),
        (&mut s.floor.transform, &mut s.floor.mesh, &s.floor.color),
      ],
      &camera_render_info,
      &vec![],
    );
  }
}
