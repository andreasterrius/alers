pub mod mesh;
pub mod raw;
pub mod shader;
pub mod text;
pub mod texture;

pub fn ale_opengl_clear_render() {
  // Clear the screen buffer
  unsafe {
    raw::clear_buffer();
  }
}
