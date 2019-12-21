use core::{mem, ptr};
use std::convert::TryInto;
use std::ffi::{c_void, CString};
use std::ptr::null;

use cgmath::{Point3, Vector3};
use cgmath::prelude::*;
use gl::types::{GLchar, GLfloat, GLint, GLsizeiptr};

use crate::data::buffer::Buffer;
use crate::resource::texture::{Texture, TextureMagnificationType, TexturePixel, TextureWrapType};

#[derive(Debug)]
pub struct CreateBufferError {}

pub unsafe fn create_buffer(vertices: &Buffer<f32>,
                            indices: &Option<Buffer<i32>>) -> Result<(u32, u32, Option<u32>, u32), CreateBufferError>
{
  let (mut vao, mut vbo) = (0, 0);
  let mut draw_size = vertices.len() as u32;
  gl::GenVertexArrays(1, &mut vao);
  gl::GenBuffers(1, &mut vbo);

  // Bind the VAO
  gl::BindVertexArray(vao);

  // Bind VBO, Pass Data
  gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
  gl::BufferData(
    gl::ARRAY_BUFFER,
    (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
    vertices.as_ptr() as *const c_void,
    gl::STATIC_DRAW,
  );

  // If we have indices then create the EBO
  let mut ebo = None;
  if let Some(buffer) = indices {
    let mut ebo_ptr = 0;
    gl::GenBuffers(1, &mut ebo_ptr);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo_ptr);
    gl::BufferData(
      gl::ELEMENT_ARRAY_BUFFER,
      (buffer.len() * mem::size_of::<GLint>()) as GLsizeiptr,
      buffer.as_ptr() as *const c_void,
      gl::STATIC_DRAW,
    );
    ebo = Some(ebo_ptr);
    draw_size = buffer.len() as u32;
  }

  let mut count = 0;
  let mut start = 0;
  let total_row_size = (vertices.total_row_size() * mem::size_of::<GLfloat>()) as GLsizeiptr;
  for element in vertices.elements() {
    //println!("{:?} {:?}", start, count);
    let stride = (start * mem::size_of::<GLfloat>()) as *const c_void;
    gl::VertexAttribPointer(count, element.size.try_into().unwrap(),
      gl::FLOAT, gl::FALSE, total_row_size.try_into().unwrap(), stride);
    gl::EnableVertexAttribArray(count);
    start += element.size;
    count += 1;
  }

  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  gl::BindVertexArray(0);

  //println!("{:?} {:?} {:?} {:?}", vao, vbo, ebo, draw_size);
  //println!("vertices {:?}", vertices);
  //println!("indices {:?}", indices);

  Ok((vao, vbo, ebo, draw_size))
}

#[derive(Debug)]
pub enum CreateShaderError {
  VertexShaderError(String),
  FragmentShaderError(String),

  LinkingShaderError(String),
}

pub unsafe fn create_shader(vertex_shader_source: &str,
                            fragment_shader_source: &str) -> Result<u32, CreateShaderError>
{
  // vertex shader
  let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
  let c_str_vert = CString::new(vertex_shader_source.as_bytes()).unwrap();
  gl::ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
  gl::CompileShader(vertex_shader);

  // check for shader compile errors
  let mut success = gl::FALSE as GLint;
  let mut info_log = Vec::with_capacity(512);
  info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
  gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
  if success != gl::TRUE as GLint {
    gl::GetShaderInfoLog(
      vertex_shader,
      512,
      ptr::null_mut(),
      info_log.as_mut_ptr() as *mut GLchar,
    );
    return Err(CreateShaderError::VertexShaderError(format!("Vertex Shader compilation failed: {}",
      String::from_utf8_lossy(&info_log))));
  }

  // fragment shader
  let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
  let c_str_frag = CString::new(fragment_shader_source.as_bytes()).unwrap();
  gl::ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
  gl::CompileShader(fragment_shader);
  // check for shader compile errors
  gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
  if success != gl::TRUE as GLint {
    gl::GetShaderInfoLog(
      fragment_shader,
      512,
      ptr::null_mut(),
      info_log.as_mut_ptr() as *mut GLchar,
    );
    return Err(CreateShaderError::FragmentShaderError(format!("Fragment shader compilation failed: {}",
      String::from_utf8_lossy(&info_log))));
  }

  // link shaders
  let shader_program = gl::CreateProgram();
  gl::AttachShader(shader_program, vertex_shader);
  gl::AttachShader(shader_program, fragment_shader);
  gl::LinkProgram(shader_program);
  // check for linking errors
  gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
  if success != gl::TRUE as GLint {
    gl::GetProgramInfoLog(
      shader_program,
      512,
      ptr::null_mut(),
      info_log.as_mut_ptr() as *mut GLchar,
    );
    return Err(CreateShaderError::LinkingShaderError(format!("Linking shader failed: {}",
      String::from_utf8_lossy(&info_log))));
  }
  gl::DeleteShader(vertex_shader);
  gl::DeleteShader(fragment_shader);

  Ok(shader_program)
}

#[derive(Debug)]
pub struct CreateTextureError {}

pub unsafe fn create_texture(texture: &Texture) -> Result<u32, CreateTextureError>
{
  let mut gl_texture = 0;
  gl::GenTextures(1, &mut gl_texture);
  gl::BindTexture(gl::TEXTURE_2D, gl_texture);

  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, texture_wrap_to_gl(&texture.get_wrap().x));
  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, texture_wrap_to_gl(&texture.get_wrap().y));

  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, texture_mag_to_gl(&texture.get_magnification().min));
  gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, texture_mag_to_gl(&texture.get_magnification().max));

  let byte = texture.get_data();
  let (internal_format, pixel_format, ptr) = match byte {
    TexturePixel::RgbF8(v) => { (gl::RGB as i32, gl::UNSIGNED_BYTE, v.as_ptr() as *const c_void) },
    TexturePixel::RgbF32(v) => { (gl::RGB32F as i32, gl::FLOAT, v.as_ptr() as *const c_void) },
  };

  gl::TexImage2D(
    gl::TEXTURE_2D,
    0,
    internal_format,
    texture.width() as i32,
    texture.height() as i32,
    0,
    gl::RGB,
    pixel_format,
    ptr
  );

  return Ok(gl_texture);
}

fn texture_wrap_to_gl(wrap: &TextureWrapType) -> i32
{
  match wrap {
    TextureWrapType::ClampToEdge => gl::CLAMP_TO_EDGE as i32,
    TextureWrapType::MirroredRepeat => gl::MIRRORED_REPEAT as i32,
    TextureWrapType::Repeat => gl::REPEAT as i32,
  }
}

fn texture_mag_to_gl(mag: &TextureMagnificationType) -> i32
{
  match mag {
    TextureMagnificationType::Nearest => gl::NEAREST as i32,
    TextureMagnificationType::Linear => gl::LINEAR as i32,
  }
}

pub unsafe fn create_renderbuffer() -> (u32, u32)
{
  let mut fbo = 0;
  gl::GenFramebuffers(1, &mut fbo);

  let mut rbo = 0;
  gl::GenRenderbuffers(1, &mut rbo);

  gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
  gl::BindRenderbuffer(gl::RENDERBUFFER, rbo);

  gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT24, 512, 512);
  gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, rbo);

  return (fbo, rbo)
}

pub unsafe fn create_cubemap() -> u32
{
  let mut cube_map = 0;
  gl::GenTextures(1, &mut cube_map);
  gl::BindTexture(gl::TEXTURE_CUBE_MAP, cube_map);
  for i in 0..6 {
    gl::TexImage2D(gl::TEXTURE_CUBE_MAP_POSITIVE_X + i, 0, gl::RGB32F as i32, 512, 512, 0, gl::RGB, gl::FLOAT, null());
  }
  gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
  gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
  gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, gl::CLAMP_TO_EDGE as i32);
  gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
  gl::TexParameteri(gl::TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

  return cube_map;
}

pub unsafe fn project_cubemap(renderbuffer: u32, cubemap: u32) {
  //let projection = cgmath::perspective();
  let views = vec!(
    cgmath::Matrix4::look_at(Point3::origin(), Point3::new(1.0f32, 0.0, 0.0), -Vector3::unit_y()),
  );
}

pub unsafe fn enable_depth_test() {
  gl::Enable(gl::DEPTH_TEST)
}