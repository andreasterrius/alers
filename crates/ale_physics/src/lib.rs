use crate::rapier3d::dynamics::CCDSolver;
use rapier3d::dynamics::{
  BodyStatus, IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};
use rapier3d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier3d::geometry::{ColliderBuilder, ColliderHandle, SharedShape};
use rapier3d::na::{Quaternion, UnitQuaternion, Vector3};
use rapier3d::pipeline::PhysicsPipeline;

use ale_math::{Euler, InnerSpace};
pub use rapier3d;
use rapier3d::math::{AngVector, Isometry};

pub enum RigidBodyType {
  Kinematic,
  Dynamic,
  Static,
}

// Single collider only
pub enum RigidBodyShape {
  Cube(ale_math::Vector3<f32>), //extent
  Sphere(f32),                  // radius
}

pub struct PhysicsContext {
  pub pipeline: PhysicsPipeline,
  pub gravity: Vector3<f32>,
  pub broad_phase: BroadPhase,
  pub narrow_phase: NarrowPhase,
  pub bodies: RigidBodySet,
  pub colliders: ColliderSet,
  pub ccd_solver: CCDSolver,
  pub joints: JointSet,
}

pub fn ale_physics_context_new() -> PhysicsContext {
  PhysicsContext {
    pipeline: PhysicsPipeline::new(),
    gravity: Vector3::new(0.0, -9.81, 0.0),
    broad_phase: BroadPhase::new(),
    narrow_phase: NarrowPhase::new(),
    bodies: RigidBodySet::new(),
    colliders: ColliderSet::new(),
    ccd_solver: CCDSolver::new(),
    joints: JointSet::new(),
  }
}

pub fn ale_physics_object_new(
  physics_context: &mut PhysicsContext,
  position: ale_math::Vector3<f32>,
  rotation: ale_math::Quaternion<f32>,
  rigidbody_shape: RigidBodyShape,
  rigidbody_type: RigidBodyType,
  gravity_enable: bool,
) -> (RigidBodyHandle, ColliderHandle) {
  let r = UnitQuaternion::from_quaternion(Quaternion::new(rotation.s, rotation.v.x, rotation.v.y, rotation.v.z));

  let mut rigidbody_isometry = Isometry::new(Vector3::new(position.x, position.y, position.z), AngVector::default());
  rigidbody_isometry.rotation = r;

  let mut rigidbody_builder = RigidBodyBuilder::new(match rigidbody_type {
    RigidBodyType::Kinematic => BodyStatus::Kinematic,
    RigidBodyType::Dynamic => BodyStatus::Dynamic,
    RigidBodyType::Static => BodyStatus::Static,
  })
  .position(rigidbody_isometry);
  if !gravity_enable {
    rigidbody_builder = rigidbody_builder.gravity_scale(0.0);
  }
  let rigidbody = rigidbody_builder.build();
  let rigidbody_handle = physics_context.bodies.insert(rigidbody);

  let mut collider_isometry = Isometry::new(Vector3::default(), AngVector::default());
  collider_isometry.rotation = r;

  let collider = match rigidbody_shape {
    RigidBodyShape::Cube(box_extent) => ColliderBuilder::cuboid(box_extent.x, box_extent.y, box_extent.z).build(),
    RigidBodyShape::Sphere(radius) => ColliderBuilder::ball(radius).build(),
  };

  let collider_handle = physics_context
    .colliders
    .insert(collider, rigidbody_handle, &mut physics_context.bodies);

  (rigidbody_handle, collider_handle)
}

pub fn ale_physics_context_tick(physics_context: &mut PhysicsContext, delta_time: f32) {
  let physics_hooks = ();
  let event_handler = ();

  let integration_parameter = IntegrationParameters {
    dt: delta_time,
    ..Default::default()
  };

  physics_context.pipeline.step(
    &physics_context.gravity,
    &integration_parameter,
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

pub fn ale_physics_context_update(
  physics_context: &mut PhysicsContext,
  objects: Vec<(&mut ale_math::transform::AleTransform, &RigidBodyHandle)>,
) {
  for (t, rigidbody_handle) in objects {
    let rigidbody = physics_context.bodies.get(*rigidbody_handle).unwrap();
    let position: [f32; 3] = rigidbody.position().translation.into();
    let rotation = rigidbody.position().rotation;

    t.set_position(ale_math::Vector3::new(position[0], position[1], position[2]));
    t.set_rotation(ale_math::Quaternion::new(
      rotation.w, rotation.i, rotation.j, rotation.k,
    ));
  }
}
