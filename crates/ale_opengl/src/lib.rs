use ale_math::color::Color;

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
pub mod renderer;
pub mod texture;
pub mod wire;

pub fn ale_opengl_clear_render() {
  // Clear the screen buffer
  unsafe {
    raw::clear_buffer(0.2f32, 0.3f32, 0.3f32);
  }
}

pub fn ale_opengl_clear_render_color(color: Color) {
  unsafe {
    let (r, g, b) = color.get_rgb();
    raw::clear_buffer(r, g, b);
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
