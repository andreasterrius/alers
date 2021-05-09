use rapier3d::dynamics::{IntegrationParameters, JointSet, RigidBodySet, RigidBodyBuilder, RigidBodyHandle};
use rapier3d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier3d::pipeline::PhysicsPipeline;
use rapier3d::geometry::{ColliderBuilder, SharedShape, ColliderHandle};
use crate::rapier3d::dynamics::CCDSolver;
use rapier3d::na::Vector3;

pub use rapier3d;

pub struct PhysicsContext {
  pub pipeline: PhysicsPipeline,
  pub gravity : Vector3<f32>,
  pub integration_parameters: IntegrationParameters,
  pub broad_phase: BroadPhase,
  pub narrow_phase: NarrowPhase,
  pub bodies: RigidBodySet,
  pub colliders: ColliderSet,
  pub ccd_solver : CCDSolver,
  pub joints: JointSet,
}

pub fn ale_physics_context_new() -> PhysicsContext {
  PhysicsContext {
    pipeline: PhysicsPipeline::new(),
    gravity: Vector3::new(0.0, -9.81, 0.0),
    integration_parameters: IntegrationParameters::default(),
    broad_phase: BroadPhase::new(),
    narrow_phase: NarrowPhase::new(),
    bodies: RigidBodySet::new(),
    colliders: ColliderSet::new(),
    ccd_solver: CCDSolver::new(),
    joints: JointSet::new(),
  }
}

pub fn ale_physics_context_collider_cuboid_new(physics_context : &mut PhysicsContext, v : ale_math::Vector3<f32>) -> (RigidBodyHandle, ColliderHandle) {
  let rigidbody = RigidBodyBuilder::new_dynamic().build();
  let rigidbody_handle = physics_context.bodies.insert(rigidbody);

  let collider = ColliderBuilder::cuboid(v.x, v.y, v.z).build();
  let collider_handle = physics_context
      .colliders
      .insert(collider, rigidbody_handle, &mut physics_context.bodies);

  (rigidbody_handle, collider_handle)
}

pub fn ale_physics_context_tick(physics_context : &mut PhysicsContext,
                                delta_time : f32){
  let physics_hooks = ();
  let event_handler = ();

  let integration_parameter = IntegrationParameters { dt: delta_time, ..Default::default() };

  physics_context.pipeline.step(
    &physics_context.gravity,
    &physics_context.integration_parameters,
    &mut physics_context.broad_phase,
    &mut physics_context.narrow_phase,
    &mut physics_context.bodies,
    &mut physics_context.colliders,
    &mut physics_context.joints,
    &mut physics_context.ccd_solver,
    &physics_hooks,
    &event_handler,
  )
}

pub fn ale_physics_context_update(physics_context: &mut PhysicsContext,
                                  objects: Vec<(&mut ale_math::transform::AleTransform, &RigidBodyHandle)>){
  for (t, rigidbody_handle) in objects {
    let rigidbody = physics_context.bodies.get(*rigidbody_handle)
        .unwrap();
    let position: [f32; 3] = rigidbody.position().translation.into();
    let rotation = rigidbody.position().rotation;

    t.set_position(ale_math::Vector3::new(position[0], position[1], position[2]));
    t.set_rotation(ale_math::Quaternion::new(rotation.w, rotation.i, rotation.j, rotation.k));
  }
}