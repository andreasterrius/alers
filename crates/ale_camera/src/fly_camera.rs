use crate::{
  ale_camera_new, ale_camera_render_info_calculate, ale_camera_set_rotation, ale_camera_translate, Camera,
  CameraRenderInfo,
};
use ale_input::{Action, Input, Key};
use ale_math::prelude::*;
use ale_math::{Deg, Quaternion, Vector2, Vector3, Zero};

pub const SMALL_NUMBERF32: f32 = 1e-12;

pub struct FlyCamera {
  camera: Camera,

  delta_rotate_input: Vector2<f32>,
  rotate_input: Vector2<f32>,
  move_input: Vector3<f32>,

  camera_speed: f32,
  camera_rotate_speed: f32,

  disable_input: bool,
}

pub fn ale_fly_camera_new(position: Vector3<f32>, viewport_size: Vector2<u32>, fov: f32) -> FlyCamera {
  FlyCamera {
    camera: ale_camera_new(position, viewport_size, fov),
    delta_rotate_input: Vector2::zero(),
    rotate_input: Vector2::zero(),
    move_input: Vector3::zero(),
    camera_speed: 10.0,
    camera_rotate_speed: 100.0,
    disable_input: false,
  }
}

fn intern_fly_camera_input(fly_camera: &mut FlyCamera, input: &Input) {
  if fly_camera.disable_input {
    return;
  }

  match input {
    // Handle movement
    Input::Key(Key::A, _, Action::Press, _) => fly_camera.move_input.x += -10.0f32,
    Input::Key(Key::D, _, Action::Press, _) => fly_camera.move_input.x += 10.0f32,
    Input::Key(Key::W, _, Action::Press, _) => fly_camera.move_input.z += 10.0f32,
    Input::Key(Key::S, _, Action::Press, _) => fly_camera.move_input.z += -10.0f32,
    Input::Key(Key::A, _, Action::Release, _) => fly_camera.move_input.x += 10.0f32,
    Input::Key(Key::D, _, Action::Release, _) => fly_camera.move_input.x += -10.0f32,
    Input::Key(Key::W, _, Action::Release, _) => fly_camera.move_input.z += -10.0f32,
    Input::Key(Key::S, _, Action::Release, _) => fly_camera.move_input.z += 10.0f32,
    Input::Key(Key::Z, _, Action::Press, _) => fly_camera.disable_input = !fly_camera.disable_input,

    Input::MouseMotion(x, y) => {
      fly_camera.rotate_input.x += *x;
      fly_camera.rotate_input.y += *y;
      fly_camera.delta_rotate_input.x = *x;
      fly_camera.delta_rotate_input.y = *y;
    }
    _ => {}
  }
}

pub fn ale_fly_camera_inputs(fly_camera: &mut FlyCamera, inputs: &Vec<Input>) {
  fly_camera.delta_rotate_input = Vector2::zero();
  for input in inputs {
    intern_fly_camera_input(fly_camera, &input);
  }

  if !fly_camera.delta_rotate_input.is_zero() {
    ale_camera_set_rotation(
      &mut fly_camera.camera,
      Quaternion::from_angle_y(-Deg(fly_camera.rotate_input.x * fly_camera.camera_rotate_speed))
        * Quaternion::from_angle_x(-Deg(fly_camera.rotate_input.y * fly_camera.camera_rotate_speed)),
    );
  }
}

pub fn ale_fly_camera_tick(fly_camera: &mut FlyCamera, delta_time: f64) {
  ale_camera_translate(
    &mut fly_camera.camera,
    fly_camera.move_input * fly_camera.camera_speed * (delta_time as f32),
  );
}

pub fn ale_fly_camera_render_info_calculate(fly_camera: &mut FlyCamera) -> CameraRenderInfo {
  ale_camera_render_info_calculate(&mut fly_camera.camera)
}
