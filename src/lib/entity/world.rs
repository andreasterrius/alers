use crate::entity::camera::CameraEntity;
use crate::entity::pawn::PawnEntity;
use crate::entity::skybox::SkyboxEntity;
use crate::input::Input;
use crate::renderer::opengl::RenderTasks;

pub struct World {
  // List of pawns in the world
  pawns: Vec<PawnEntity>,

  // Active camera
  camera: CameraEntity,

  // Active skybox
  skybox: Option<SkyboxEntity>,
}

impl World {
  pub fn new() -> World {
    World {
      pawns: vec![],
      camera: CameraEntity::None,
      skybox: None,
    }
  }

  pub fn add_pawn(&mut self, pawn: PawnEntity) {
    self.pawns.push(pawn);
  }

  pub fn set_skybox(&mut self, skybox: SkyboxEntity) {
    self.skybox = Some(skybox);
  }

  pub fn set_camera(&mut self, camera: CameraEntity) {
    self.camera = camera;
  }

  pub fn input(&mut self, inputs: &Vec<Input>) {
    self.camera.input(inputs);
  }

  pub fn tick(&mut self, delta_time: f32) {
    self.camera.tick(delta_time);
  }

  pub fn render<T: RenderTasks>(&mut self, render_tasks: &mut dyn RenderTasks) {
    // No camera = nothing rendered
    let camera_render_info = match self.camera.get_camera_render_info() {
      None => {
        return;
      }
      Some(cri) => cri,
    };

    render_tasks.with_camera(camera_render_info.clone());

    for p in &mut self.pawns {
      render_tasks.queue_static_mesh(
        p.shader_id,
        p.static_mesh_id,
        p.textures.clone(),
        p.transform.matrix(),
        p.shader_variables.clone(),
      );
    }

    if let Some(skybox) = &self.skybox {
      render_tasks.queue_skybox(skybox.shader_id, skybox.static_mesh_id, skybox.rendered_cubemap_id, vec![]);

      render_tasks.with_skybox(skybox.irradiance_cubemap_id);
    }
  }
}
