use rand::random;

use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_app::{ale_app_resource_path, ale_app_run, App, AppError};
use ale_camera::Camera;
use ale_input::{Action, Input, Key};
use ale_math::color::Color;
use ale_math::rect::Rect;
use ale_math::transform::AleTransform;
use ale_math::{ale_bounding_box_size, Transform, Vector2, Vector3, Zero};
use ale_opengl::pbr::{
  ale_opengl_pbr_context_new, ale_opengl_pbr_render, ale_opengl_pbr_render_envmap, OpenGLPBRContext,
};
use ale_opengl::{ale_opengl_clear_render_color, ale_opengl_depth_test_enable};
use ale_physics::rapier3d::dynamics::RigidBodyHandle;
use ale_physics::rapier3d::geometry::ColliderHandle;
use ale_physics::{
  ale_physics_context_new, ale_physics_context_tick, ale_physics_context_update,
  ale_physics_object_linear_velocity_get, ale_physics_object_linear_velocity_set, ale_physics_object_new,
  ale_physics_object_position_set, PhysicsContext, RigidBodyShape, RigidBodyType,
};
use ale_resources::gltf;
use ale_resources::mesh::{Mesh, MeshId};
use ale_resources::texture::Texture;

use crate::Shape::{Cube, Sphere};

fn main() {
  ale_app_run(
    Pong,
    DisplayInfo::new(Rect {
      position: Vector2::zero(),
      size: Vector2::new(800, 600),
    }),
  );
}

struct Pong;

pub enum Shape {
  Cube,
  Sphere,
}

struct Object {
  transform: AleTransform,
  rigidbody_handle: RigidBodyHandle,
  collider_handle: ColliderHandle,
  mesh_id: MeshId,
  color: Vector3<f32>,
}

fn ale_create_pong_object(
  physics_context: &mut PhysicsContext,
  mut transform: AleTransform,
  mesh: &Mesh,
  shape: Shape,
  rigidbody_type: RigidBodyType,
  color: Vector3<f32>,
  gravity_enable: bool,
  is_sensor: bool,
) -> Object {
  let mut bb = transform
    .scale_matrix()
    .transform_vector(ale_bounding_box_size(mesh.bounding_box));

  let rigidbody_shape = match shape {
    Shape::Cube => RigidBodyShape::Cube(bb / 2.0),
    Shape::Sphere => RigidBodyShape::Sphere(bb.x / 2.0), // radius
  };

  let (rigidbody_handle, collider_handle) = ale_physics_object_new(
    physics_context,
    transform.position,
    transform.lcl_rotation,
    rigidbody_shape,
    rigidbody_type,
    gravity_enable,
    is_sensor,
  );

  Object {
    transform,
    rigidbody_handle,
    collider_handle,
    mesh_id: mesh.uid(),
    color,
  }
}

struct State {
  physics_context: PhysicsContext,
  opengl_pbr_context: OpenGLPBRContext,

  ball: Object,
  paddle_left: Object,
  paddle_right: Object,
  arena: Vec<Object>,

  //fly_camera: FlyCamera,
  camera: Camera,

  paddle_left_velocity: Vector3<f32>,
  paddle_right_velocity: Vector3<f32>,

  // Game state
  should_init: bool,
  score_left: i32,
  score_right: i32,
}

impl App<State> for Pong {
  fn load(&mut self, window: &Window) -> Result<State, AppError> {
    /*
     * Create the mesh
     */
    let cube_mesh = Mesh::new_cube();
    let ball_mesh = gltf::load(&ale_app_resource_path("gltf/bakso.gltf")).remove(0);

    /*
     * Physics setup
     */
    let mut physics_context = ale_physics_context_new();

    /*
     * Create Objects
     */
    let mut paddle_left = ale_create_pong_object(
      &mut physics_context,
      AleTransform::from_position_scale(Vector3::new(-10.0, 0.0, 0.0), Vector3::new(0.2, 0.2, 1.0)),
      &cube_mesh,
      Cube,
      RigidBodyType::Kinematic,
      Vector3::new(0.0, 1.0, 0.0),
      false,
      true,
    );

    let mut paddle_right = ale_create_pong_object(
      &mut physics_context,
      AleTransform::from_position_scale(Vector3::new(10.0, 0.0, 0.0), Vector3::new(0.2, 0.2, 1.0)),
      &cube_mesh,
      Cube,
      RigidBodyType::Kinematic,
      Vector3::new(1.0, 0.0, 0.0),
      true,
      true,
    );

    let mut arena = vec![
      // floor
      ale_create_pong_object(
        &mut physics_context,
        AleTransform::from_position_scale(Vector3::new(0.0, -1.1, 0.0), Vector3::new(20.00, 1.0, 20.0)),
        &cube_mesh,
        Cube,
        RigidBodyType::Static,
        Vector3::new(0.0, 0.0, 0.0),
        false,
        false,
      ),
    ];

    let mut ball = ale_create_pong_object(
      &mut physics_context,
      AleTransform::from_position_scale(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.4, 0.4, 0.4)),
      &ball_mesh,
      Sphere,
      RigidBodyType::Dynamic,
      Vector3::new(1.0, 1.0, 1.0),
      true,
      false,
    );

    /*
     * Camera Creation
     */
    // let mut fly_camera = FlyCamera::new(Camera::new(
    //   Vector3::new(0.0, 10.0, 0.0),
    //   window.get_display_info().dimension.clone(),
    //   90.0,
    // ));
    // fly_camera.camera_mut().look_at(Vector3::new(0.0, 0.0, 0.0));

    let mut camera = Camera::new(
      Vector3::new(0.0, 10.0, 0.0),
      window.get_display_info().dimension.clone(),
      90.0,
    );
    camera.look_at(Vector3::zero());

    /*
     * Renderer
     */
    let hdr_texture = Texture::load(&ale_app_resource_path("hdr/GravelPlaza_Env.hdr")).unwrap();
    let opengl_pbr_context = ale_opengl_pbr_context_new(
      &hdr_texture,
      &window.get_display_info().dimension,
      vec![&cube_mesh, &ball_mesh],
    );

    ale_opengl_depth_test_enable();

    Ok(State {
      opengl_pbr_context,
      physics_context,
      //fly_camera,
      camera,
      ball,
      paddle_left,
      paddle_right,
      arena,
      should_init: true,
      score_left: 0,
      score_right: 0,
      paddle_left_velocity: Vector3::zero(),
      paddle_right_velocity: Vector3::zero(),
    })
  }

  fn input(&mut self, s: &mut State, inputs: Vec<Input>) {
    for input in &inputs {
      match input {
        // Handle movement
        Input::Key(Key::W, _, Action::Press, _) => {
          s.paddle_left_velocity += Vector3::new(0.0, 0.0, -1.0);
        }
        Input::Key(Key::S, _, Action::Press, _) => {
          s.paddle_left_velocity += Vector3::new(0.0, 0.0, 1.0);
        }
        Input::Key(Key::W, _, Action::Release, _) => {
          s.paddle_left_velocity -= Vector3::new(0.0, 0.0, -1.0);
        }
        Input::Key(Key::S, _, Action::Release, _) => {
          s.paddle_left_velocity -= Vector3::new(0.0, 0.0, 1.0);
        }
        _ => {}
      }
    }

    // AI Input
    if s.paddle_right.transform.position.z < s.ball.transform.position.z {
      s.paddle_right_velocity = Vector3::new(0.0, 0.0, 1.0);
    } else {
      s.paddle_right_velocity = Vector3::new(0.0, 0.0, -1.0);
    }
  }

  fn fixed_tick(&mut self, s: &mut State, delta_time: f32) {
    // Move paddles
    let paddle_speed = 12.0f32;
    ale_physics_object_position_set(
      &mut s.physics_context,
      s.paddle_left.rigidbody_handle,
      s.paddle_left.transform.position + (s.paddle_left_velocity * paddle_speed * delta_time),
    );

    ale_physics_object_position_set(
      &mut s.physics_context,
      s.paddle_right.rigidbody_handle,
      s.paddle_right.transform.position + (s.paddle_right_velocity * paddle_speed * delta_time),
    );

    let mut objects = vec![
      (&mut s.paddle_left.transform, &s.paddle_left.rigidbody_handle),
      (&mut s.paddle_right.transform, &s.paddle_right.rigidbody_handle),
      (&mut s.ball.transform, &s.ball.rigidbody_handle),
    ];

    // Put arena objects here
    for object in &mut s.arena {
      objects.push((&mut object.transform, &mut object.rigidbody_handle));
    }

    ale_physics_context_tick(&mut s.physics_context, delta_time);
    ale_physics_context_update(&mut s.physics_context, objects);

    //resolve isect
    while let Ok(isect) = s.physics_context.isect_recv.try_recv() {
      if !isect.intersecting {
        continue;
      }

      let mut ball_collider = None;
      let mut paddle_collider = None;
      if isect.collider1 == s.ball.collider_handle {
        ball_collider = Some(isect.collider1);
        paddle_collider = Some(isect.collider2);
      } else if isect.collider2 == s.ball.collider_handle {
        ball_collider = Some(isect.collider2);
        paddle_collider = Some(isect.collider1);
      }

      // Now we determine which paddle collider this is
      // Create rudimentary bounce, nothing fancy at all
      match paddle_collider {
        None => {}
        Some(pc) => {
          if pc == s.paddle_left.collider_handle {
            let linvel = ale_physics_object_linear_velocity_get(&mut s.physics_context, s.ball.rigidbody_handle);
            ale_physics_object_linear_velocity_set(
              &mut s.physics_context,
              s.ball.rigidbody_handle,
              Vector3::new(f32::abs(linvel.x * 1.3f32), linvel.y, linvel.z),
            );
          } else if pc == s.paddle_right.collider_handle {
            let linvel = ale_physics_object_linear_velocity_get(&mut s.physics_context, s.ball.rigidbody_handle);
            ale_physics_object_linear_velocity_set(
              &mut s.physics_context,
              s.ball.rigidbody_handle,
              Vector3::new(-f32::abs(linvel.x * 1.3f32), linvel.y, linvel.z),
            );
          }
        }
      }
    }
  }

  fn tick(&mut self, s: &mut State) {
    // Pull back pong paddle if it goes too far
    if s.paddle_left.transform.position.z < -10.0f32 {
      s.paddle_left.transform.position.z = -10.0f32;
      ale_physics_object_position_set(
        &mut s.physics_context,
        s.paddle_left.rigidbody_handle,
        s.paddle_left.transform.position,
      );
    }

    if s.paddle_left.transform.position.z > 10.0f32 {
      s.paddle_left.transform.position.z = 10.0f32;
      ale_physics_object_position_set(
        &mut s.physics_context,
        s.paddle_left.rigidbody_handle,
        s.paddle_left.transform.position,
      );
    }

    // Check if ball passes through left goal or right goal
    let linvel = ale_physics_object_linear_velocity_get(&mut s.physics_context, s.ball.rigidbody_handle);

    // bounce bottom
    if s.ball.transform.position.z < -10.0f32 {
      ale_physics_object_linear_velocity_set(
        &mut s.physics_context,
        s.ball.rigidbody_handle,
        Vector3::new(linvel.x * 1.4f32, linvel.y, f32::abs(linvel.z)),
      );
    }
    // bounce top
    if s.ball.transform.position.z > 10.0f32 {
      ale_physics_object_linear_velocity_set(
        &mut s.physics_context,
        s.ball.rigidbody_handle,
        Vector3::new(linvel.x * 1.4f32, linvel.y, -f32::abs(linvel.z)),
      );
    }
    // left bound check
    if s.ball.transform.position.x < -10.0f32 {
      s.should_init = true;
      s.score_left += 1;
    }
    // right bound check
    if s.ball.transform.position.x > 10.0f32 {
      s.should_init = true;
      s.score_right += 1;
    }

    if s.should_init {
      // Since the transform will take from rigidbody,
      // just set the rigibody position
      ale_physics_object_position_set(
        &mut s.physics_context,
        s.paddle_left.rigidbody_handle,
        Vector3::new(-10.0, 0.0, 0.0),
      );
      ale_physics_object_position_set(
        &mut s.physics_context,
        s.paddle_right.rigidbody_handle,
        Vector3::new(10.0, 0.0, 0.0),
      );
      ale_physics_object_position_set(
        &mut s.physics_context,
        s.ball.rigidbody_handle,
        Vector3::new(0.0, 0.0, 0.0),
      );

      // calculate the initial velocity for the ball
      let speed = 20.0f32;
      let initial_velocity = Vector3::new(0.4f32, 0.0, 0.4);
      let random_velocity_base = Vector3::new(0.1f32, 0.0, 0.1);
      let random_mult: Vector3<f32> = Vector3::new(random(), 0.0, random());
      let mut velocity = initial_velocity
        + Vector3::new(
          random_mult.x * random_velocity_base.x,
          random_mult.y * random_velocity_base.y,
          random_mult.z * random_velocity_base.z,
        );

      let go_top: bool = random();
      let go_left: bool = random();
      if go_left {
        velocity.x = velocity.x * -1.0f32;
      }
      if go_top {
        velocity.z = velocity.z * -1.0f32;
      }
      velocity *= speed;

      ale_physics_object_linear_velocity_set(&mut s.physics_context, s.ball.rigidbody_handle, velocity);

      s.should_init = false;
    }
  }

  fn render(&mut self, s: &mut State) {
    ale_opengl_clear_render_color(Color::from_rgb(0.123f32, 0.54, 0.514));

    let camera_render_info = s.camera.camera_render_info();

    ale_opengl_pbr_render_envmap(&s.opengl_pbr_context, &camera_render_info);

    let mut renderables = vec![
      (
        &mut s.paddle_left.transform,
        &mut s.paddle_left.mesh_id,
        &s.paddle_left.color,
      ),
      (
        &mut s.paddle_right.transform,
        &mut s.paddle_right.mesh_id,
        &s.paddle_right.color,
      ),
      (&mut s.ball.transform, &mut s.ball.mesh_id, &s.ball.color),
    ];

    for object in &mut s.arena {
      renderables.push((&mut object.transform, &mut object.mesh_id, &object.color));
    }

    ale_opengl_pbr_render(&s.opengl_pbr_context, renderables, &camera_render_info, &vec![]);
  }
}
