use ale_console::{ale_console_variable_register, Console};
use ale_opengl::mesh::OpenGLMeshContext;
use ale_opengl::shader::OpenGLShaderContext;
use ale_variable::Variable;

static FXAA_RELATIVE_THRESHOLD: &str = "fxaaRelativeThreshold";
static FXAA_CONTRAST_THRESHOLD: &str = "fxaaContrastThreshold";

pub struct OpenGLFxaaContext {
  opengl_mesh_context: OpenGLMeshContext,
  opengl_shader_context: OpenGLShaderContext,
}

pub fn ale_opengl_fxaa_console_variable_register(console: &mut Console) {
  ale_console_variable_register(console, Variable::F32_1(FXAA_RELATIVE_THRESHOLD.to_owned(), 0.0));
  ale_console_variable_register(console, Variable::F32_1(FXAA_CONTRAST_THRESHOLD.to_owned(), 0.0));
}
