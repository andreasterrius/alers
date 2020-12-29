use crate::envmap::OpenGLEnvmap;
use crate::mesh::OpenGLMesh;
use crate::raw;
use crate::shader::OpenGLShader;
use ale_camera::CameraRenderInfo;
use ale_material::PBRMaterial;
use ale_math::transform::Transform;
use ale_math::Matrix;

pub fn render_pbr(
  ogl_mesh: &OpenGLMesh,
  ogl_shader: &OpenGLShader,
  ogl_envmap: &OpenGLEnvmap,
  pbr_material: &PBRMaterial,
  transform: &mut Transform,
  camera_render_info: &CameraRenderInfo,
) {
  unsafe {
    //ogl_shader.activate(shader_variables);
    ogl_envmap.bind_to_shader(ogl_shader, "irradianceMap");

    // Bind textures here
    // for i in 0..texture_ids.len() {
    //   let texture_draw_info = match context.get_texture(&texture_ids[i]) {
    //     None => continue,
    //     Some(x) => x,
    //   };
    //
    //   raw::active_texture((i + 1) as u32);
    //   raw::bind_texture(texture_draw_info.id.0);
    // }

    let camera_position = camera_render_info.position;
    raw::uniform3f(
      ogl_shader.id,
      "camPos",
      camera_position.x,
      camera_position.y,
      camera_position.z,
    );

    // Pass uniforms
    raw::matrix4f(ogl_shader.id, "model", transform.matrix().as_ptr());
    raw::matrix4f(ogl_shader.id, "view", camera_render_info.view.as_ptr());
    raw::matrix4f(ogl_shader.id, "projection", camera_render_info.projection.as_ptr());

    // Bind Array Buffer
    ogl_mesh.render();
  }
}
