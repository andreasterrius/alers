use ale_app::App;
use ale_camera::fly_camera::FlyCamera;
use ale_ecs::{EntityBuilder, World};
use ale_gltf::{glTF, glTFLoader};
use ale_math::{Vector2, Vector3};
use ale_mesh::Mesh;
use ale_opengl::mesh::{OpenGLMeshContext, OpenGLMeshLoader};
use ale_opengl::resource_pile::OpenGLResourcePile;
use ale_opengl::shader::OpenGLShaderLoader;
use ale_opengl::texture::OpenGLTextureLoader;
use ale_opengl_envmap::OpenGLEnvmap;
use ale_opengl_pbr::OpenGLPBRContext;
use ale_resource::ResourcePile;
use ale_shader::{Shader, ShaderLoader};
use ale_texture::{Texture, TextureLoader};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
  let window_size = Vector2::new(1024, 800);

  // Application
  let mut app = App::new(window_size);

  let mut opengl_resource_pile = Rc::new(RefCell::new(OpenGLResourcePile::new()));
  {
    opengl_resource_pile
      .borrow_mut()
      .add_loader::<OpenGLMeshLoader, Mesh>(OpenGLMeshLoader);
    opengl_resource_pile
      .borrow_mut()
      .add_loader::<OpenGLShaderLoader, Shader>(OpenGLShaderLoader);
    opengl_resource_pile
      .borrow_mut()
      .add_loader::<OpenGLTextureLoader, Texture>(OpenGLTextureLoader);
  }

  let mut resource_pile = ResourcePile::new();
  resource_pile.add_observer(Rc::downgrade(&opengl_resource_pile));

  // Resources
  let scene = resource_pile.load_gltf("gltf/scene.gltf");
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
