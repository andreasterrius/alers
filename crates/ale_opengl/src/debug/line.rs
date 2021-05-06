use crate::constant::{PROJECTION, VIEW};
use crate::raw;
use crate::shader::{ale_opengl_shader_activate, ale_opengl_shader_new, OpenGLShader};
use ale_camera::CameraRenderInfo;
use ale_math::{Matrix, Vector3, Zero};
use ale_shader::ale_shader_new;
use ale_variable::Variable;
use core::mem;
use gl::types::{GLfloat, GLsizeiptr};
use std::mem::size_of;
use std::os::raw::c_void;
use std::ptr::null;

const BUFFER_SIZE: usize = 200000;

pub struct OpenGLDebugContext {
  // shader
  shader: OpenGLShader,

  // mesh representation for lines
  line_vao: u32,
  line_vbo: u32,
  line_draw_size: u32,

  point_vao : u32,
  point_vbo : u32,
  point_draw_size: u32,

  // Lines to render
  // start_pos, color
  lines: Vec<[Vector3<f32>; 2]>,

  // Points to render
  // start_pos, color
  points: Vec<[Vector3<f32>; 2]>,
}

pub fn ale_opengl_debug_context_new() -> OpenGLDebugContext {
  let (line_vao, line_vbo, line_draw_size) = unsafe { create_line_buffer() };
  let (point_vao, point_vbo, point_draw_size) = unsafe{ create_point_buffer() };

  let shader = ale_opengl_shader_new(&ale_shader_new(
    include_str!("../../../../resources/shaders/debug/line.vert").to_owned(),
    include_str!("../../../../resources/shaders/debug/line.frag").to_owned(),
  ))
  .unwrap();

  OpenGLDebugContext {
    line_vao,
    line_vbo,
    line_draw_size,
    point_vao,
    point_vbo,
    point_draw_size,
    shader,
    lines: vec![],
    points: vec![],
  }
}

pub fn ale_opengl_debug_line_queue(
  opengl_debug_context: &mut OpenGLDebugContext,
  start_pos: Vector3<f32>,
  end_pos: Vector3<f32>,
  color: Vector3<f32>,
) {
  opengl_debug_context.lines.push([start_pos, color]);
  opengl_debug_context.lines.push([end_pos, color]);
}

pub fn ale_opengl_debug_line_clear(opengl_debug_context: &mut OpenGLDebugContext) {
  opengl_debug_context.lines.clear();
}

pub fn ale_opengl_debug_point_queue(
  opengl_debug_context: &mut OpenGLDebugContext,
  start_pos: Vector3<f32>,
  color: Vector3<f32>,
) {
  opengl_debug_context.points.push([start_pos, color]);
}

pub fn ale_opengl_debug_point_clear(opengl_debug_context: &mut OpenGLDebugContext) {
  opengl_debug_context.points.clear();
}

pub fn ale_opengl_debug_render(opengl_debug_context: &OpenGLDebugContext, camera: &CameraRenderInfo) {
  // Render lines
  unsafe {
    let shader = &opengl_debug_context.shader;

    ale_opengl_shader_activate(shader, &vec![]);

    raw::matrix4f(shader.id, VIEW, camera.view.as_ptr());
    raw::matrix4f(shader.id, PROJECTION, camera.projection.as_ptr());

    for chunk in opengl_debug_context.lines.chunks(BUFFER_SIZE) {
      gl::BindBuffer(gl::ARRAY_BUFFER, opengl_debug_context.line_vbo);
      gl::BufferSubData(
        gl::ARRAY_BUFFER,
        0,
        (size_of::<f32>() * 6 * chunk.len()) as isize,
        chunk.as_ptr() as *const c_void,
      );

      gl::BindVertexArray(opengl_debug_context.line_vao);
      gl::DrawArrays(gl::LINES, 0, chunk.len() as i32);
    }

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  }

  // Render points
  unsafe {
    let shader = &opengl_debug_context.shader;

    ale_opengl_shader_activate(shader, &vec![]);

    raw::matrix4f(shader.id, VIEW, camera.view.as_ptr());
    raw::matrix4f(shader.id, PROJECTION, camera.projection.as_ptr());
    gl::PointSize(10.0);

    for chunk in opengl_debug_context.points.chunks(BUFFER_SIZE) {
      gl::BindBuffer(gl::ARRAY_BUFFER, opengl_debug_context.point_vbo);
      gl::BufferSubData(
        gl::ARRAY_BUFFER,
        0,
        (size_of::<f32>() * 6 * chunk.len()) as isize,
        chunk.as_ptr() as *const c_void,
      );

      gl::BindVertexArray(opengl_debug_context.point_vao);
      gl::DrawArrays(gl::POINTS, 0, chunk.len() as i32);
    }

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  }
}

pub unsafe fn create_line_buffer() -> (u32, u32, u32) {
  let (mut vao, mut vbo) = (0, 0);
  gl::GenVertexArrays(1, &mut vao);
  gl::GenBuffers(1, &mut vbo);

  // Bind the VAO
  gl::BindVertexArray(vao);

  let mut draw_size = BUFFER_SIZE as u32;

  // Bind VBO, Pass Data
  let float_size = mem::size_of::<GLfloat>() as i32;
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
  gl::BufferData(
    gl::ARRAY_BUFFER,
    (BUFFER_SIZE * 6 * float_size as usize) as isize,
    null(),
    gl::DYNAMIC_DRAW,
  );

  // Pass start position
  gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * float_size, 0 as *const c_void);
  gl::EnableVertexAttribArray(0);

  // Pass color
  gl::VertexAttribPointer(
    1,
    3,
    gl::FLOAT,
    gl::FALSE,
    6 * float_size,
    (3 * float_size) as *const c_void,
  );
  gl::EnableVertexAttribArray(1);

  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  gl::BindVertexArray(0);

  (vao, vbo, draw_size)
}

pub unsafe fn create_point_buffer() -> (u32, u32, u32) {
  let (mut vao, mut vbo) = (0, 0);
  gl::GenVertexArrays(1, &mut vao);
  gl::GenBuffers(1, &mut vbo);

  // Bind the VAO
  gl::BindVertexArray(vao);

  let mut draw_size = BUFFER_SIZE as u32;

  // Bind VBO, Pass Data
  let float_size = mem::size_of::<GLfloat>() as i32;
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
  gl::BufferData(
    gl::ARRAY_BUFFER,
    (BUFFER_SIZE * 6 * float_size as usize) as isize,
    null(),
    gl::DYNAMIC_DRAW,
  );

  // Pass start position
  gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * float_size, 0 as *const c_void);
  gl::EnableVertexAttribArray(0);

  // Pass color
  gl::VertexAttribPointer(
    1,
    3,
    gl::FLOAT,
    gl::FALSE,
    6 * float_size,
    (3 * float_size) as *const c_void,
  );
  gl::EnableVertexAttribArray(1);

  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  gl::BindVertexArray(0);

  (vao, vbo, draw_size)
}
