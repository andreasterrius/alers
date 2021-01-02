use crate::constant::{CAMERA_POSITION, ENVIRONMENT_MAP, MODEL, PROJECTION, VIEW};
use crate::mesh::OpenGLMesh;
use crate::old::cubemap::Cubemap;
use crate::raw;
use crate::shader::{ale_opengl_shader_activate, OpenGLShader};
use crate::texture::OpenGLTexture;
use ale_camera::CameraRenderInfo;
use ale_math::transform::Transform;
use ale_math::Matrix;
use ale_mesh::Mesh;
use ale_shader::Shader;

pub struct OpenGLPBRContext {
  pub cube_mesh: OpenGLMesh,

  pub pbr_shader: OpenGLShader,
  pub skybox_shader: OpenGLShader,

  pub irradiance_cubemap: u32, // OpenGLCubemap
  pub convoluted_cubemap: u32, // OpenGLCubemap
}

pub fn ale_opengl_pbr_context_new() {}

pub fn ale_opengl_pbr_render(
  opengl_pbr_context: &OpenGLPBRContext,
  mesh: &mut Vec<(Transform, OpenGLMesh)>,
  camera_render_info: &CameraRenderInfo,
  textures: &Vec<OpenGLTexture>,
) {
  unsafe {
    let pbr_shader = &opengl_pbr_context.pbr_shader;
    let cubemap = opengl_pbr_context.irradiance_cubemap;
    // Bind shader
    ale_opengl_shader_activate(pbr_shader, &vec![]);

    raw::uniform1i(pbr_shader.id, "irradianceMap", 0);
    raw::active_texture(0);
    raw::bind_cubemap(cubemap);

    // Bind textures here
    for i in 0..textures.len() {
      raw::active_texture((i + 1) as u32);
      raw::bind_texture(textures[i].id.0);
    }

    let camera_position = camera_render_info.position;
    raw::uniform3f(
      pbr_shader.id,
      CAMERA_POSITION,
      camera_position.x,
      camera_position.y,
      camera_position.z,
    );

    raw::matrix4f(pbr_shader.id, VIEW, camera_render_info.view.as_ptr());
    raw::matrix4f(pbr_shader.id, PROJECTION, camera_render_info.projection.as_ptr());

    for (t, m) in mesh {
      // Pass uniforms
      raw::matrix4f(pbr_shader.id, MODEL, t.matrix().as_ptr());

      // Bind Array Buffer
      raw::bind_vao(m.vao);

      // Draw according to EBO
      match m.ebo {
        None => raw::draw_arrays(0, m.draw_size),
        Some(_) => raw::draw_elements(m.draw_size),
      }
    }
  }
}

pub fn ale_opengl_pbr_render_envmap(opengl_pbr_context: &OpenGLPBRContext, camera_render_info: &CameraRenderInfo) {
  unsafe {
    let skybox_shader = &opengl_pbr_context.pbr_shader;
    let convoluted_cubemap = opengl_pbr_context.convoluted_cubemap;
    let cube_mesh = &opengl_pbr_context.cube_mesh;

    raw::use_shader(skybox_shader.id);
    raw::uniform1i(skybox_shader.id, ENVIRONMENT_MAP, 0);
    raw::matrix4f(skybox_shader.id, VIEW, camera_render_info.view.as_ptr());
    raw::matrix4f(skybox_shader.id, PROJECTION, camera_render_info.projection.as_ptr());
    raw::active_texture(0);
    raw::bind_cubemap(convoluted_cubemap);

    raw::bind_vao(cube_mesh.vao);
    match cube_mesh.ebo {
      None => raw::draw_arrays(0, cube_mesh.draw_size),
      Some(_) => raw::draw_elements(cube_mesh.draw_size),
    }
  }
}
