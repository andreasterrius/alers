pub mod console;
pub mod constant;
pub mod mesh;
pub mod old;
pub mod pbr;
pub mod raw;
pub mod render_frame;
pub mod shader;
pub mod text;
pub mod texture;
pub mod wire;
pub mod raymarch;
pub mod debug;

pub fn ale_opengl_clear_render() {
  // Clear the screen buffer
  unsafe {
    raw::clear_buffer();
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
