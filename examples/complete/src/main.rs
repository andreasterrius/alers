use ale_app::{ale_app_new, ale_app_run};
use ale_camera::fly_camera::FlyCamera;
use ale_gltf::ale_gltf_load;
use ale_math::{Vector2, Vector3};
use ale_opengl::mesh::OpenGLMeshContext;
use ale_opengl_envmap::{ale_opengl_envmap_new, ale_opengl_envmap_render};
use ale_opengl_pbr::OpenGLPBRContext;
use ale_texture::ale_texture_load;

fn main() {
  let window_size = Vector2::new(1024, 800);
  let mut app = ale_app_new(window_size);
  let mut gltf = ale_gltf_load("/home/alether/Codes/Graphics/alers/examples/complete/resources/scene.gltf");
  let mut hdr_texture =
    ale_texture_load("/home/alether/Codes/Graphics/alers/examples/shared_resources/hdr_texture/GravelPlaza_REF.hdr")
      .unwrap();
  let envmap = ale_opengl_envmap_new(&hdr_texture, window_size);
  let mut fly_camera = FlyCamera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32), window_size, 90.0f32);

  let pbr_context = OpenGLPBRContext::new();
  let mut mesh_context = OpenGLMeshContext::new();

  //
  // ale_app_run(
  //   &mut app,
  //   &mut |inputs| {
  //     //ale_fly_camera_inputs(&mut fly_camera, &inputs);
  //   },
  //   &mut |delta_time| {},
  //   &mut || {
  //     let camera_render_info = ale_fly_camera_render_info_calculate(&mut fly_camera);
  //
  //     ale_opengl_envmap_render(&envmap, &camera_render_info);
  //     for (transform, mesh) in &mut gltf {
  //       ale_opengl_pbr_render(
  //         &pbr_context,
  //         &mut mesh_context,
  //         Some(&envmap),
  //         transform,
  //         &mesh,
  //         &camera_render_info,
  //         &vec![
  //           Variable::F32_3("albedo".to_owned(), Vector3::new(0.7f32, 0.7, 0.7)),
  //           Variable::F32_1("metallic".to_owned(), 0.0f32),
  //           Variable::F32_1("roughness".to_owned(), 0.5f32),
  //           Variable::F32_1("ao".to_owned(), 0.5f32),
  //         ],
  //       );
  //     }
  //   },
  // );
}
