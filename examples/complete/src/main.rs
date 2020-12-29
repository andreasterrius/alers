use ale_app::App;
use ale_camera::fly_camera::FlyCamera;
use ale_ecs::{EntityBuilder, World};
use ale_gltf::{glTF, glTFLoader};
use ale_material::PBRMaterial;
use ale_math::{Vector2, Vector3};
use ale_mesh::Mesh;
use ale_opengl::mesh::{OpenGLMeshContext, OpenGLMeshLoader};
use ale_opengl::resource_pile::OpenGLResourcePile;
use ale_opengl::shader::OpenGLShaderLoader;
use ale_opengl::texture::OpenGLTextureLoader;
use ale_resource::ResourcePile;
use ale_shader::{Shader, ShaderLoader};
use ale_texture::{Texture, TextureLoader};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
  let window_size = Vector2::new(1024, 800);

  // Application
  let mut app = App::new(window_size);

  let mut resource_pile = app.get_mut_resource_pile();

  // Resources
  let scene = resource_pile.load_gltf("gltf/scene.gltf");
  let pbr_material = PBRMaterial::load_default(&mut resource_pile);
  let mut fly_camera = FlyCamera::new(Vector3::new(0.0f32, 0.0f32, -10.0f32), window_size, 90.0f32);

  // World
  let mut world = World::new();
  for (transform, material) in scene {
    world.spawn((transform, material, pbr_material.clone()));
  }

  app.run(world, &mut fly_camera);
}
