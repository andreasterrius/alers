use ale_autoid::ProcessUniqueId;
use ale_resource::{Resource, ResourcePile, ResourcePileObserver};
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub mod console;
pub mod mesh;
pub mod raw;
pub mod render_frame;
pub mod resource_pile;
pub mod shader;
pub mod text;
pub mod texture;

pub struct OpenGL;

impl OpenGL {
  pub fn clear_buffer() {
    // Clear the screen buffer
    unsafe {
      raw::clear_buffer();
    }
  }

  pub fn enable_blend() {
    unsafe {
      raw::enable_blend();
      raw::enable_blend_transparent();
    }
  }

  pub fn enable_depth_test() {
    unsafe {
      raw::enable_depth_test();
    }
  }
}
