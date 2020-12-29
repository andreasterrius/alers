use crate::Texture;
use ale_autoid::{struct_id, struct_id_impl, Identifiable, ProcessUniqueId};
use ale_mesh::Mesh;
use ale_resource::{Resource, ResourcePile, ResourceType};
use ale_shader::Shader;

pub struct Envmap {
  pub id: EnvmapId,
  pub texture: Resource<Texture>,
  pub equirect_shader: Resource<Shader>,
  pub irradiance_shader: Resource<Shader>,
  pub skybox_shader: Resource<Shader>,
  pub cube_mesh: Resource<Mesh>,
}

struct_id!(EnvmapId);
struct_id_impl!(EnvmapId, Envmap, id);

impl Envmap {
  pub fn new(
    texture: Resource<Texture>,
    equirect_shader: Resource<Shader>,
    irradiance_shader: Resource<Shader>,
    skybox_shader: Resource<Shader>,
    cube_mesh: Resource<Mesh>,
  ) -> Envmap {
    Envmap {
      id: Default::default(),
      texture,
      equirect_shader,
      irradiance_shader,
      skybox_shader,
      cube_mesh,
    }
  }
}

pub trait EnvmapLoader {
  fn load_envmap(&mut self, envmap: Envmap) -> Resource<Envmap>;
}

impl EnvmapLoader for ResourcePile {
  fn load_envmap(&mut self, envmap: Envmap) -> Resource<Envmap> {
    self.register(envmap)
  }
}

impl ResourceType for Envmap {}
