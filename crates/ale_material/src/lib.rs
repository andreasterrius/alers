use ale_camera::CameraRenderInfo;
use ale_math::transform::Transform;
use ale_math::{Array, Matrix, Matrix4, Vector3, Zero};
use ale_mesh::{Mesh, MeshId};
use ale_resource::{Resource, ResourcePile};
use ale_shader::{Shader, ShaderLoader};
use ale_texture::envmap::{Envmap, EnvmapLoader};
use ale_texture::{Texture, TextureLoader};
use ale_variable::Variable;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PBRMaterial {
  pub shader: Resource<Shader>,
  pub envmap: Resource<Envmap>,

  pub pbr_albedo_color: Vector3<f32>,
}

impl PBRMaterial {
  pub fn new(pbr_shader: Resource<Shader>, pbr_envmap: Resource<Envmap>) -> PBRMaterial {
    PBRMaterial {
      shader: pbr_shader,
      envmap: pbr_envmap,
      pbr_albedo_color: Vector3::from_value(1.0f32),
    }
  }

  pub fn load_default(resource_pile: &mut ResourcePile) -> PBRMaterial {
    let pbr_shader = resource_pile.load_shader("shader/pbr/pbr.vert", "shader/pbr/pbr.frag");
    let hdr_texture = resource_pile.load_texture("hdr_texture/GravelPlaza_Env.hdr");
    let equirect_shader = resource_pile.load_shader("shader/envmap/cubemap.vert", "shader/envmap/equirect.frag");
    let irradiance_shader = resource_pile.load_shader("shader/envmap/cubemap.vert", "shader/envmap/irradiance.frag");
    let skybox_shader = resource_pile.load_shader("shader/envmap/skybox.vert", "shader/envmap/skybox.frag");
    let cube_mesh = resource_pile.register(Mesh::new_cube());
    let pbr_envmap = resource_pile.load_envmap(Envmap::new(
      hdr_texture,
      equirect_shader,
      irradiance_shader,
      skybox_shader,
      cube_mesh,
    ));

    PBRMaterial {
      shader: pbr_shader,
      envmap: pbr_envmap,
      pbr_albedo_color: Vector3::from_value(1.0f32),
    }
  }
}
