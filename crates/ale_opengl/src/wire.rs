use crate::mesh::{ale_opengl_mesh_new, OpenGLMesh};
use crate::shader::{ale_opengl_shader_new, OpenGLShader};
use ale_console::{ale_console_variable_event_handle, ale_console_variable_register, Console};
use ale_math::transform::Transform;
use ale_mesh::{ale_mesh_bounding_box_new, Mesh};
use ale_shader::ale_shader_new;
use ale_variable::{to_variable, ToVariable};

pub struct OpenGLWireContext {
  pub bounding_box_mesh: OpenGLMesh,

  pub bounding_box_shader: OpenGLShader,

  pub wire_render_enable: bool,
  pub wire_thickness: f32,
}

pub fn ale_opengl_wire_context_new() -> OpenGLWireContext {
  let wire_shader = ale_shader_new(
    include_str!("../../../shaders/wire.vert").to_owned(),
    include_str!("../../../shaders/wire.frag").to_owned(),
  );

  OpenGLWireContext {
    bounding_box_mesh: ale_opengl_mesh_new(&ale_mesh_bounding_box_new()).unwrap(),
    bounding_box_shader: ale_opengl_shader_new(&wire_shader).unwrap(),
    wire_render_enable: false,
    wire_thickness: 0.0,
  }
}

pub fn ale_opengl_wire_console_variable_register(opengl_wire_context: &OpenGLWireContext, console: &mut Console) {
  ale_console_variable_register(console, to_variable!(opengl_wire_context.wire_render_enable));
  ale_console_variable_register(console, to_variable!(opengl_wire_context.wire_thickness));
}

pub fn ale_opengl_wire_console_variable_refresh(opengl_wire_context: &mut OpenGLWireContext, console: &mut Console) {
  opengl_wire_context.wire_render_enable =
    ale_console_variable_event_handle(console, to_variable!(opengl_wire_context.wire_render_enable));
  opengl_wire_context.wire_thickness =
    ale_console_variable_event_handle(console, to_variable!(opengl_wire_context.wire_thickness));
}

pub fn ale_opengl_wire_boundingbox_render(
  opengl_wire_context: &mut OpenGLWireContext,
  meshes: &Vec<(Transform, Mesh)>,
) {
  if !opengl_wire_context.wire_render_enable {
    return;
  }

  //activate shader
  //pass transform
  //render box
}
