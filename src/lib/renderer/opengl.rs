use std::{mem, ptr};
use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::CString;
use std::os::raw::c_void;

use cgmath::{Matrix, Matrix4, Vector3, Vector4};
use gl::types::{GLchar, GLfloat, GLint, GLsizeiptr};

use crate::camera::CameraRenderInfo;
use crate::data::buffer::Buffer;
use crate::data::id::Id;
use crate::data::id::Identifiable;
use crate::resource::shader::ShaderFile;
use crate::resource::static_mesh::StaticMesh;
use crate::resource::texture::{Texture, TextureMagnificationType, TextureWrapType, TexturePixel};

pub struct Context {
  static_meshes: HashMap<Id, StaticMeshDrawInfo>,
  shaders: HashMap<Id, ShaderDrawInfo>,
  textures: HashMap<Id, TextureDrawInfo>,
}

impl Context {
  pub fn new() -> Context {
    Context {
      static_meshes: HashMap::new(),
      shaders: HashMap::new(),
      textures: HashMap::new(),
    }
  }

  pub fn setup(&mut self) {
    unsafe {
      gl::Enable(gl::DEPTH_TEST)
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

  pub fn texture(&mut self, texture: &Texture) -> Result<(), TextureError> {
    self.textures.insert(texture.uid(), TextureDrawInfo::new(texture)?);
    Ok(())
  }

  pub fn get_static_mesh(&self, mesh_id: &Id) -> Option<&StaticMeshDrawInfo> {
    self.static_meshes.get(&mesh_id)
  }

  pub fn get_shader(&self, shader_id: &Id) -> Option<&ShaderDrawInfo> {
    self.shaders.get(&shader_id)
  }

  pub fn get_texture(&self, texture_id: &Id) -> Option<&TextureDrawInfo> {
    self.textures.get(&texture_id)
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
  _vbo: u32,
  ebo: Option<u32>,
  draw_size: u32, //indices size, or vertex size
}

impl StaticMeshDrawInfo {
  pub fn new(mesh: &StaticMesh) -> Result<StaticMeshDrawInfo, StaticMeshError> {
    let (vao, vbo, ebo, draw_size) = unsafe { create_buffer(&mesh.vertices, &mesh.indices)? };
    Ok(StaticMeshDrawInfo { vao, _vbo: vbo, ebo, draw_size })
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

#[derive(Debug)]
pub enum TextureError {
  CreateTextureError(CreateTextureError),
}

impl From<CreateTextureError> for TextureError {
  fn from(e: CreateTextureError) -> Self {
    TextureError::CreateTextureError(e)
  }
}

pub struct TextureDrawInfo {
  texture: u32,
}

impl TextureDrawInfo {
  pub fn new(texture: &Texture) -> Result<TextureDrawInfo, TextureError> {
    let texture = unsafe { create_texture(texture)? };
    Ok(TextureDrawInfo { texture })
  }
}

enum Renderable {
  StaticMesh {
    shader_id: Id,
    mesh_id: Id,
    texture_ids: Vec<Id>,
    transform: Matrix4<f32>,
    shader_variables: Vec<ShaderVariable>,
  }
}

pub trait RenderTasks {
  fn queue_static_mesh(&mut self,
                       shader: &ShaderFile,
                       mesh: &StaticMesh,
                       textures: Vec<&Texture>,
                       transform: Matrix4<f32>,
                       shader_vars: Vec<ShaderVariable>);

  fn render(&mut self, context: &Context, camera: &mut CameraRenderInfo);
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
  fn queue_static_mesh(&mut self,
                       shader: &ShaderFile,
                       mesh: &StaticMesh,
                       textures: Vec<&Texture>,
                       transform: Matrix4<f32>,
                       shader_vars: Vec<ShaderVariable>)
  {
    self.renderables.push(Renderable::StaticMesh {
      shader_id: shader.uid(),
      mesh_id: mesh.uid(),
      texture_ids: textures.into_iter().map(|x| x.uid()).collect(),
      transform,
      shader_variables: shader_vars,
    });
  }

  fn render(&mut self, context: &Context, camera: &mut CameraRenderInfo) {

    // Clear screen
    unsafe {
      gl::ClearColor(0.2f32, 0.3f32, 0.3f32, 1.0f32);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    }

    for renderable in &self.renderables {
      match renderable {
        Renderable::StaticMesh { shader_id, mesh_id, texture_ids, transform, shader_variables } => {
          let mesh_draw_info = match context.get_static_mesh(mesh_id) {
            None => continue,
            Some(x) => x
          };
          let shader_draw_info = match context.get_shader(shader_id) {
            None => continue,
            Some(x) => x,
          };

          unsafe {
            // Bind shader
            gl::UseProgram(shader_draw_info.shader);

            // Bind textures here
            for i in 0..texture_ids.len() {
              let texture_draw_info = match context.get_texture(&texture_ids[i]) {
                None => continue,
                Some(x) => x,
              };

              gl::ActiveTexture(gl::TEXTURE0 + i as u32);
              gl::BindTexture(gl::TEXTURE_2D, texture_draw_info.texture);
            }

            // Pass shader specific uniforms
            for shader_variable in shader_variables {
              let location = gl::GetUniformLocation(shader_draw_info.shader, CString::new(shader_variable.name.clone()).unwrap().as_ptr() as *const i8);
              match shader_variable.variable_type {
                ShaderVariableType::F32_3(vec) => gl::Uniform3f(location, vec.x, vec.y, vec.z),
                ShaderVariableType::F32_4(vec) => gl::Uniform4f(location, vec.x, vec.y, vec.z, vec.w),
              }
            }

            // Pass uniforms
            gl::UniformMatrix4fv(gl::GetUniformLocation(shader_draw_info.shader, CString::new("model").unwrap().as_ptr()), 1, gl::FALSE, transform.as_ptr());
            gl::UniformMatrix4fv(gl::GetUniformLocation(shader_draw_info.shader, CString::new("view").unwrap().as_ptr() as *const i8), 1, gl::FALSE, camera.view.as_ptr());
            gl::UniformMatrix4fv(gl::GetUniformLocation(shader_draw_info.shader, CString::new("projection").unwrap().as_ptr() as *const i8), 1, gl::FALSE, camera.projection.as_ptr());

            // Bind Array Buffer
            gl::BindVertexArray(mesh_draw_info.vao);

            // Draw according to EBO
            match mesh_draw_info.ebo {
              None => gl::DrawArrays(gl::TRIANGLES, 0, mesh_draw_info.draw_size as i32),
              Some(_) => gl::DrawElements(gl::TRIANGLES, mesh_draw_info.draw_size as i32, gl::UNSIGNED_INT, ptr::null()),
            }
          }
        }
      }
    }
  }
}

pub struct ShaderVariable {
  pub name: String,
  pub variable_type: ShaderVariableType,
}

impl ShaderVariable {
  pub fn new(name: String, variable_type: ShaderVariableType) -> ShaderVariable {
    ShaderVariable {
      name,
      variable_type,
    }
  }
}

pub enum ShaderVariableType {
  F32_3(Vector3<f32>),
  F32_4(Vector4<f32>),
}

#[derive(Debug)]
pub struct CreateBufferError {}

unsafe fn create_buffer(vertices: &Buffer<f32>,
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

unsafe fn create_shader(vertex_shader_source: &str,
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

unsafe fn create_texture(texture: &Texture) -> Result<u32, CreateTextureError>
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

fn texture_wrap_to_gl(wrap: &TextureWrapType) -> i32 {
  match wrap {
    TextureWrapType::ClampToEdge => gl::CLAMP_TO_EDGE as i32,
    TextureWrapType::MirroredRepeat => gl::MIRRORED_REPEAT as i32,
    TextureWrapType::Repeat => gl::REPEAT as i32,
  }
}

fn texture_mag_to_gl(mag: &TextureMagnificationType) -> i32 {
  match mag {
    TextureMagnificationType::Nearest => gl::NEAREST as i32,
    TextureMagnificationType::Linear => gl::LINEAR as i32,
  }
}
