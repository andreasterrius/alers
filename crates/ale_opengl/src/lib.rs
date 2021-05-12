use ale_math::Vector3;

pub mod console;
pub mod constant;
pub mod debug;
pub mod mesh;
pub mod old;
pub mod pbr;
pub mod raw;
pub mod raymarch;
pub mod render_frame;
pub mod shader;
pub mod text;
pub mod texture;
pub mod wire;

pub fn ale_opengl_clear_render() {
  // Clear the screen buffer
  unsafe {
    raw::clear_buffer(0.2f32, 0.3f32, 0.3f32);
  }
}

pub fn ale_opengl_clear_render_color(color: Vector3<f32>) {
  unsafe {
    raw::clear_buffer(color.x, color.y, color.z);
  }
}

pub fn ale_opengl_blend_enable() {
  unsafe {
    raw::enable_blend();
    raw::enable_blend_transparent();
  }
}

pub fn ale_opengl_depth_test_enable() {
  unsafe {
    raw::enable_depth_test();
  }
}
