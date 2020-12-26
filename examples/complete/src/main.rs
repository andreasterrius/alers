use ale_app::App;
use ale_camera::fly_camera::FlyCamera;
use ale_ecs::{EntityBuilder, World};
use ale_gltf::{glTF, glTFLoader};
use ale_math::{Vector2, Vector3};
use ale_opengl::mesh::OpenGLMeshContext;
use ale_opengl_envmap::OpenGLEnvmap;
use ale_opengl_pbr::OpenGLPBRContext;
use ale_resource::ResourcePile;
use ale_shader::ShaderLoader;
use ale_texture::{Texture, TextureLoader};

fn main() {
  let window_size = Vector2::new(1024, 800);

  // Application
  let mut app = App::new(window_size);
  let mut resource_pile = ResourcePile::new();
  let mut opengl_resource_pile = ResourcePile::new();

  // Resources
  let scene = resource_pile.load_gltf("gltf/scene.gltf");
  let mut mesh_context = OpenGLMeshContext::new();
  for (t, m) in &scene {
    mesh_context.register(&m.read());
  }

  let hdr_texture = resource_pile.load_texture("hdr_texture/GravelPlaza_Env.hdr");
  let envmap = { OpenGLEnvmap::new(&hdr_texture.read(), window_size, &mut resource_pile) };

  let pbr_context = OpenGLPBRContext::new(&mut resource_pile);

  // Entities
  let mut fly_camera = FlyCamera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32), window_size, 90.0f32);

  // World
  let mut world = World::new();
  for (t, m) in scene {
    world.spawn((t, m));
  }
}
