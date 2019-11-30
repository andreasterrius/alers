use gl;
use gl::types::{GLfloat, GLsizeiptr, GLint, GLchar};
use std::{ptr, mem};
use data::id::Id;
use cgmath::{Matrix4};
use std::collections::HashMap;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use resource::static_mesh::StaticMesh;
use resource::shader::ShaderFile;
use data::id::Identifiable;
use data::buffer::Buffer;
use std::os::raw::c_void;
use std::convert::TryInto;
use std::ffi::CString;

pub struct Context {
  static_meshes: HashMap<Id, StaticMeshDrawInfo>,
  shaders: HashMap<Id, ShaderDrawInfo>
}

impl Context {
  pub fn new() -> Context {
    Context {
      static_meshes: HashMap::new(),
      shaders: HashMap::new(),
    }
  }

  pub fn static_mesh(&mut self, mesh: &StaticMesh) -> Result<(), StaticMeshError> {
    self.static_meshes.insert(mesh.uid(), StaticMeshDrawInfo::new(mesh)?);
    Ok(())
  }

  pub fn shader(&mut self, shader: &ShaderFile) -> Result<(), ShaderError> {
    self.shaders.insert(shader.uid(), ShaderDrawInfo::new(shader)?);
    Ok(())
  }
}

#[derive(Debug)]
pub enum StaticMeshError {
  CreateBufferError(CreateBufferError)
}

impl From<CreateBufferError> for StaticMeshError {
  fn from(error: CreateBufferError) -> Self {
    StaticMeshError::CreateBufferError(error)
  }
}

pub struct StaticMeshDrawInfo {
  vao: u32,
  vbo: u32,
  ebo: Option<u32>,
}

impl StaticMeshDrawInfo {
  pub fn new(mesh: &StaticMesh) -> Result<StaticMeshDrawInfo, StaticMeshError> {
    let (vao, vbo, ebo) = unsafe { create_buffer(&mesh.vertices, &mesh.indices)? };
    Ok(StaticMeshDrawInfo { vao, vbo, ebo })
  }
}

#[derive(Debug)]
pub enum ShaderError {
  CompilationError(CreateShaderError)
}

impl From<CreateShaderError> for ShaderError {
  fn from(error: CreateShaderError) -> Self {
    ShaderError::CompilationError(error)
  }
}

pub struct ShaderDrawInfo {
  shader: u32,
}

impl ShaderDrawInfo {
  pub fn new(shader: &ShaderFile) -> Result<ShaderDrawInfo, ShaderError> {
    let shader = unsafe { create_shader(&shader.vertex_shader, &shader.fragment_shader)? };
    Ok(ShaderDrawInfo { shader })
  }
}

enum Renderable {
  StaticMesh { shader_id: Id, mesh_id: Id, transform: Matrix4<f32> }
}

pub trait RenderTasks {
  fn queue_static_mesh(&mut self, shader: &ShaderFile, mesh: &StaticMesh, transform: Matrix4<f32>);

  fn render(&mut self, context: &Context);
}

pub struct SimpleRenderTasks {
  renderables: Vec<Renderable>
}

impl SimpleRenderTasks {
  pub fn new() -> SimpleRenderTasks {
    SimpleRenderTasks { renderables: vec![] }
  }
}

impl RenderTasks for SimpleRenderTasks {
  fn queue_static_mesh(&mut self, shader: &ShaderFile, mesh: &StaticMesh, transform: Matrix4<f32>) {
    self.renderables.push(Renderable::StaticMesh {
      shader_id: shader.uid(),
      mesh_id: mesh.uid(),
      transform
    });
  }

  fn render(&mut self, context: &Context) {}
}

#[derive(Debug)]
pub struct CreateBufferError {}

unsafe fn create_buffer(vertices: &Buffer<f32>,
                        indices: &Option<Buffer<i32>>) -> Result<(u32, u32, Option<u32>), CreateBufferError>
{
  let (mut vao, mut vbo) = (0, 0);
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
      gl::STATIC_DRAW
    );
    ebo = Some(ebo_ptr);
  }

  let mut count = 0;
  let mut start = 0;
  let total_row_size = (vertices.total_row_size() * mem::size_of::<GLfloat>()) as GLsizeiptr;
  for element in vertices.elements() {
    let stride = (start * mem::size_of::<GLfloat>()) as *const c_void;
    gl::VertexAttribPointer(count, element.size.try_into().unwrap(),
      gl::FLOAT, gl::FALSE, total_row_size.try_into().unwrap(), stride);
    gl::EnableVertexAttribArray(count);
    start += element.size;
    count += 1;
  }

  gl::BindBuffer(gl::ARRAY_BUFFER, 0);
  gl::BindVertexArray(0);

  Ok((vao, vbo, ebo))
}

#[derive(Debug)]
pub enum CreateShaderError {
  VertexShaderError(String),
  FragmentShaderError(String),
  LinkingShaderError(String),
}

unsafe fn create_shader(vertex_shader_source: &str,
                        fragment_shader_source: &str) -> Result<u32, CreateShaderError> {
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