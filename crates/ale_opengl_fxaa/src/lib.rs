use ale_console::Console;
use ale_opengl::mesh::OpenGLMeshContext;
use ale_opengl::render_frame::OpenGLRenderFrameContext;
use ale_opengl::resource_pile::OpenGLResource;
use ale_opengl::shader::{OpenGLShader, OpenGLShaderContext};
use ale_opengl::OpenGLRenderer;
use ale_resource::Resource;
use ale_shader::Shader;
use ale_variable::{to_variable, ToVariable, Variable};
use std::collections::hash_map::Entry;

pub struct OpenGLFXAA {
  // The main fxaa shader
  pub(crate) fxaa_shader: OpenGLResource<OpenGLShader>,

  // Variables for shaders
  pub(crate) fxaa_relative_threshold: f32,
  pub(crate) fxaa_contrast_threshold: f32,
  pub(crate) fxaa_subpixel_blending: f32,
  pub(crate) fxaa_is_enabled: bool,
}

impl OpenGLFXAA {
  pub fn new(fxaa_shader: OpenGLResource<OpenGLShader>) -> OpenGLFXAA {
    OpenGLFXAA {
      fxaa_shader,
      fxaa_relative_threshold: 0.0312,
      fxaa_contrast_threshold: 0.063,
      fxaa_subpixel_blending: 1.0,
      fxaa_is_enabled: true,
    }
  }

  pub fn variable_register(&self, console: &mut Console) {
    console.variable_register(to_variable!(self.fxaa_relative_threshold));
    console.variable_register(to_variable!(self.fxaa_contrast_threshold));
    console.variable_register(to_variable!(self.fxaa_subpixel_blending));
    console.variable_register(to_variable!(self.fxaa_is_enabled));
  }

  pub fn variable_refresh(&mut self, console: &mut Console) {
    self.fxaa_relative_threshold = console.variable_handle_event(to_variable!(self.fxaa_relative_threshold));
    self.fxaa_contrast_threshold = console.variable_handle_event(to_variable!(self.fxaa_contrast_threshold));
    self.fxaa_subpixel_blending = console.variable_handle_event(to_variable!(self.fxaa_subpixel_blending));
    self.fxaa_is_enabled = console.variable_handle_event(to_variable!(self.fxaa_is_enabled));
  }

  pub fn render(&self, opengl_render_frame_context: &OpenGLRenderFrameContext) {
    OpenGLRenderer::clear_buffer();
    opengl_render_frame_context.render(
      &self.fxaa_shader.read(),
      &vec![
        to_variable!(self.fxaa_contrast_threshold),
        to_variable!(self.fxaa_relative_threshold),
        to_variable!(self.fxaa_subpixel_blending),
        to_variable!(self.fxaa_is_enabled),
      ],
    );
  }
}
