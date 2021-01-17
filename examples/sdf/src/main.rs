use ale_app::display_info::DisplayInfo;
use ale_app::window::Window;
use ale_app::{ale_app_resource_path, ale_app_run, App};
use ale_camera::flycamera::FlyCamera;
use ale_camera::Camera;
use ale_gltf::ale_gltf_load;
use ale_input::{Input, Key};
use ale_math::rect::Rect;
use ale_math::transform::Transform;
use ale_math::{ale_closest_point_to_box, Array, Vector3, Zero};
use ale_mesh::sdf::{ale_mesh_sdf_new, MeshSDF};
use ale_mesh::{ale_mesh_cube_new, Mesh};
use ale_opengl::debug::line::{
  ale_opengl_line_debug_clear, ale_opengl_line_debug_context_new, ale_opengl_line_debug_queue,
  ale_opengl_line_debug_render, OpenGLLineDebugContext,
};
use ale_opengl::old::opengl::{RenderContext, SimpleRenderTasks};
use ale_opengl::pbr::{
  ale_opengl_pbr_context_new, ale_opengl_pbr_render, ale_opengl_pbr_render_debug, ale_opengl_pbr_render_envmap,
  OpenGLPBRContext,
};
use ale_opengl::raymarch::{ale_opengl_raymarch_context_new, ale_opengl_raymarch_render, OpenGLRaymarchContext};
use ale_opengl::wire::{ale_opengl_wire_boundingbox_render, ale_opengl_wire_context_new, OpenGLWireContext};
use ale_opengl::{ale_opengl_blend_enable, ale_opengl_clear_render, ale_opengl_depth_test_enable};
use ale_raymarch::ale_raymarch_sdf_single;
use ale_texture::ale_texture_load;

fn main() {
  ale_app_run(SDFDemo, DisplayInfo::new(Rect::new(1024, 800)));
}

struct State {
  fly_camera: FlyCamera,

  sphere: Vec<(Transform, Mesh)>,
  sphere_sdf: MeshSDF,

  opengl_wire_context: OpenGLWireContext,
  opengl_pbr_context: OpenGLPBRContext,
  opengl_line_debug_context: OpenGLLineDebugContext,
}

struct SDFDemo;

impl App<State> for SDFDemo {
  fn load(&mut self, context: &mut RenderContext, window: &Window) -> State {
    let mut sphere = ale_gltf_load(&ale_app_resource_path("gltf/bakso.gltf"));
    //let mut sphere = vec![(Transform::new(), ale_mesh_cube_new())];

    let fly_camera = FlyCamera::new(Camera::new(
      Vector3::from_value(0.0),
      window.get_display_info().dimension.clone(),
      90.0,
    ));
    let sphere_sdf = ale_mesh_sdf_new(&sphere[0].1, 10);
    let opengl_wire_context = ale_opengl_wire_context_new();

    let hdr_texture = ale_texture_load(&ale_app_resource_path("hdr/GravelPlaza_Env.hdr")).unwrap();
    let opengl_pbr_context =
      ale_opengl_pbr_context_new(&hdr_texture, &window.get_display_info().dimension, &mut sphere);

    let opengl_line_debug_context = ale_opengl_line_debug_context_new();

    ale_opengl_blend_enable();
    ale_opengl_depth_test_enable();

    State {
      fly_camera,
      sphere,
      sphere_sdf,
      opengl_wire_context,
      opengl_pbr_context,
      opengl_line_debug_context,
    }
  }

  fn input(&mut self, state: &mut State, inputs: Vec<Input>) {
    state.fly_camera.input(&inputs);

    for input in &inputs {
      match input {
        Input::Key(Key::K, _, _, _) => {
          let points = ale_raymarch_sdf_single(state.fly_camera.camera(), &state.sphere_sdf);
          for (start, end) in points {
            ale_opengl_line_debug_queue(
              &mut state.opengl_line_debug_context,
              start,
              end,
              Vector3::new(1.0, 1.0, 1.0),
            )
          }
        }
        Input::Key(Key::R, _, _, _) => {
          let point = state.fly_camera.camera().position();
          let closest = ale_closest_point_to_box(point, state.sphere[0].1.bounding_box);
          match closest {
            None => {
              println!("Point is inside box");
            }
            Some(close) => {
              ale_opengl_line_debug_queue(
                &mut state.opengl_line_debug_context,
                point,
                close,
                Vector3::new(1.0, 1.0, 1.0),
              );
            }
          }
        }
        _ => {}
      }
    }
  }

  fn tick(&mut self, state: &mut State, delta_time: f32) {
    state.fly_camera.tick(delta_time);
  }

  fn render(&mut self, state: &mut State, render_tasks: SimpleRenderTasks, render_context: &mut RenderContext) {
    ale_opengl_clear_render();

    let camera_render_info = state.fly_camera.get_camera_render_info();

    ale_opengl_pbr_render_envmap(&state.opengl_pbr_context, &camera_render_info);
    ale_opengl_pbr_render(
      &state.opengl_pbr_context,
      &mut state.sphere,
      &camera_render_info,
      &vec![],
    );

    // Render SDF points for given mesh
    // for (from, to, dist) in &state.sphere_sdf.points {
    //   let color = if *dist < 0.0 {
    //     Vector3::new(1.0, 0.0, 0.0)
    //   } else {
    //     Vector3::new(0.0, 1.0, 0.0)
    //   };
    //   ale_opengl_pbr_render_debug(
    //     &state.opengl_pbr_context,
    //     from.clone(),
    //     0.01f32,
    //     color,
    //     &camera_render_info,
    //   );
    //   ale_opengl_line_debug_queue(&mut state.opengl_line_debug_context, from.clone(), to.clone(), color);
    // }

    // Debug camera
    ale_opengl_line_debug_render(&state.opengl_line_debug_context, &camera_render_info);
    //ale_opengl_line_debug_clear(&mut state.opengl_line_debug_context);

    ale_opengl_wire_boundingbox_render(&mut state.opengl_wire_context, &mut state.sphere, &camera_render_info);
  }
}
