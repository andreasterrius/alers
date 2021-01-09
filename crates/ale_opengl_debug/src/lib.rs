use ale_math::{Vector3, Zero};
use core::mem;
use gl::types::{GLfloat, GLsizeiptr};
use std::os::raw::c_void;

const BUFFER_SIZE: usize = 50000;

pub struct OpenGLLineDebugContext {
  vao: u32,
  vbo: u32,
  draw_size: u32,

  // Lines to render
  // start_pos, end_pos, color
  lines: Vec<(Vector3<f32>, Vector3<f32>, Vector3<f32>)>,
}

pub fn ale_opengl_line_debug_context_new() -> OpenGLLineDebugContext {
  let (vao, vbo, draw_size) = unsafe { create_buffer() };

  OpenGLLineDebugContext {
    vao,
    vbo,
    draw_size,
    lines: vec![],
  }
}

pub fn ale_opengl_line_debug_queue(
  opengl_line_debug_context: &mut OpenGLLineDebugContext,
  start_pos: Vector3<f32>,
  end_pos: Vector3<f32>,
  color: Vector3<f32>,
) {
  opengl_line_debug_context.lines.push((start_pos, end_pos, color));
}

pub fn ale_opengl_line_debug_clear(opengl_line_debug_context: &mut OpenGLLineDebugContext) {
  opengl_line_debug_context.lines.clear();
}

pub fn ale_opengl_line_debug_render(opengl_line_debug_context: &mut OpenGLLineDebugContext) {}

/**/

pub unsafe fn create_buffer() -> (u32, u32, u32) {
  let (mut vao, mut vbo) = (0, 0);
  gl::GenVertexArrays(1, &mut vao);
  gl::GenBuffers(1, &mut vbo);

  // Bind the VAO
  gl::BindVertexArray(vao);

  let mut draw_size = BUFFER_SIZE as u32;
  let arr = [(Vector3::<f32>::zero(), Vector3::<f32>::zero(), Vector3::<f32>::zero()); BUFFER_SIZE];

  // Bind VBO, Pass Data
  let float_size = mem::size_of::<GLfloat>() as i32;
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
  gl::BufferData(
    gl::ARRAY_BUFFER,
    9 * float_size as GLsizeiptr,
    arr.as_ptr() as *const c_void,
    gl::STATIC_DRAW,
  );

  // Pass start position
  gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * float_size, 0 as *const c_void);
  gl::EnableVertexAttribArray(0);

  // Pass end position
  gl::VertexAttribPointer(
    1,
    3,
    gl::FLOAT,
    gl::FALSE,
    3 * float_size,
    (3 * float_size) as *const c_void,
  );

  // Pass color
  // Pass end position
  gl::VertexAttribPointer(
    2,
    3,
    gl::FLOAT,
    gl::FALSE,
    3 * float_size,
    (6 * float_size) as *const c_void,
  );

  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  gl::BindVertexArray(0);

  (vao, vbo, draw_size)
}
