use ale_console::{
  ale_console_print_output, ale_console_variable_event_handle, ale_console_variable_register, Console, ConsoleEvent,
};
use ale_opengl::ale_opengl_clear_render;
use ale_opengl::mesh::{ale_opengl_mesh_context_new, OpenGLMeshContext};
use ale_opengl::render_frame::{ale_opengl_render_frame_render, OpenGLRenderFrameContext};
use ale_opengl::shader::{ale_opengl_shader_context_new, OpenGLShaderContext};
use ale_variable::{to_variable, ToVariable, Variable};
use std::collections::hash_map::Entry;

pub struct OpenGLFxaaContext {
  pub(crate) opengl_mesh_context: OpenGLMeshContext,
  pub(crate) opengl_shader_context: OpenGLShaderContext,

  // Variables for shaders
  pub(crate) fxaa_relative_threshold: f32,
  pub(crate) fxaa_contrast_threshold: f32,
  pub(crate) fxaa_subpixel_blending: f32,
  pub(crate) fxaa_is_enabled: bool,
}

pub fn ale_opengl_fxaa_new() -> OpenGLFxaaContext {
  let opengl_mesh_context = ale_opengl_mesh_context_new();
  let opengl_shader_context = ale_opengl_shader_context_new();

  OpenGLFxaaContext {
    opengl_mesh_context,
    opengl_shader_context,
    fxaa_relative_threshold: 0.0312,
    fxaa_contrast_threshold: 0.063,
    fxaa_subpixel_blending: 1.0,
    fxaa_is_enabled: true,
  }
}

pub fn ale_opengl_fxaa_console_variable_register(opengl_fxaa_context: &OpenGLFxaaContext, console: &mut Console) {
  ale_console_variable_register(console, to_variable!(opengl_fxaa_context.fxaa_relative_threshold));
  ale_console_variable_register(console, to_variable!(opengl_fxaa_context.fxaa_contrast_threshold));
  ale_console_variable_register(console, to_variable!(opengl_fxaa_context.fxaa_subpixel_blending));
  ale_console_variable_register(console, to_variable!(opengl_fxaa_context.fxaa_is_enabled));
}

pub fn ale_opengl_fxaa_console_variable_refresh(opengl_fxaa_context: &mut OpenGLFxaaContext, console: &mut Console) {
  opengl_fxaa_context.fxaa_relative_threshold =
    ale_console_variable_event_handle(console, to_variable!(opengl_fxaa_context.fxaa_relative_threshold));
  opengl_fxaa_context.fxaa_contrast_threshold =
    ale_console_variable_event_handle(console, to_variable!(opengl_fxaa_context.fxaa_contrast_threshold));
  opengl_fxaa_context.fxaa_subpixel_blending =
    ale_console_variable_event_handle(console, to_variable!(opengl_fxaa_context.fxaa_subpixel_blending));
  opengl_fxaa_context.fxaa_is_enabled =
    ale_console_variable_event_handle(console, to_variable!(opengl_fxaa_context.fxaa_is_enabled));
}

pub fn ale_opengl_fxaa_render(
  opengl_fxaa_context: &OpenGLFxaaContext,
  opengl_render_frame_context: &OpenGLRenderFrameContext,
) {
  ale_opengl_clear_render();
  ale_opengl_render_frame_render(
    &opengl_render_frame_context,
    &opengl_fxaa_context.opengl_shader_context,
    &opengl_fxaa_context.opengl_mesh_context,
    &vec![
      to_variable!(opengl_fxaa_context.fxaa_contrast_threshold),
      to_variable!(opengl_fxaa_context.fxaa_relative_threshold),
      to_variable!(opengl_fxaa_context.fxaa_subpixel_blending),
      to_variable!(opengl_fxaa_context.fxaa_is_enabled),
    ],
  );
}
