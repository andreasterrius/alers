use ale_camera::CameraRenderInfo;
use ale_console::{ale_console_variable_event_handle, ale_console_variable_register, Console};
use ale_math::{Array, Matrix, Transform};
use ale_math::transform::AleTransform;
use ale_resources::mesh::Mesh;
use ale_resources::shader::Shader;
use ale_variable::{to_variable, ToVariable};

use crate::mesh::OpenGLMesh;
use crate::raw;
use crate::shader::OpenGLShader;

pub struct OpenGLWireContext {
  pub bounding_box_mesh: OpenGLMesh,

  pub bounding_box_shader: OpenGLShader,

  pub wire_render_enable: bool,
  // from 0 to 1
  pub wire_thickness: f32,
}

pub fn ale_opengl_wire_context_new() -> OpenGLWireContext {
  let wire_shader = Shader::new(
    include_str!("../../../resources/shaders/wire.vert").to_owned(),
    include_str!("../../../resources/shaders/wire.frag").to_owned(),
  );

  OpenGLWireContext {
    bounding_box_mesh: OpenGLMesh::new(&ale_mesh_bounding_box_new()).unwrap(),
    bounding_box_shader: OpenGLShader::new(&wire_shader).unwrap(),
    wire_render_enable: true,
    wire_thickness: 0.01,
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
  meshes: Vec<(&mut AleTransform, &Mesh)>,
  camera_render_info: &CameraRenderInfo,
) {
  if !opengl_wire_context.wire_render_enable {
    return;
  }

  let shader = &opengl_wire_context.bounding_box_shader;
  shader.activate(&vec![to_variable!(opengl_wire_context.wire_thickness)]);
  unsafe {
    raw::matrix4f(shader.id, "view", camera_render_info.view.as_ptr());
    raw::matrix4f(shader.id, "projection", camera_render_info.projection.as_ptr());

    for (mut transform, mesh) in meshes {
      let mut bb_transform = ale_mesh_bounding_box_matrix(mesh.bounding_box);
      bb_transform = bb_transform.concat(&transform.matrix_cache());

      raw::matrix4f(shader.id, "model", bb_transform.as_ptr());
      raw::bind_vao(opengl_wire_context.bounding_box_mesh.vao);
      match opengl_wire_context.bounding_box_mesh.ebo {
        None => raw::draw_arrays(0, opengl_wire_context.bounding_box_mesh.draw_size),
        Some(_) => raw::draw_elements(opengl_wire_context.bounding_box_mesh.draw_size),
      }
    }
  }
}
