use rapier3d::dynamics::{IntegrationParameters, JointSet, RigidBodySet};
use rapier3d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier3d::pipeline::PhysicsPipeline;

pub use rapier3d;

pub struct PhysicsContext {
  pub pipeline: PhysicsPipeline,
  pub integration_parameters: IntegrationParameters,
  pub broad_phase: BroadPhase,
  pub narrow_pase: NarrowPhase,
  pub bodies: RigidBodySet,
  pub colliders: ColliderSet,
  pub joints: JointSet,
}

pub fn ale_physics_context_new() -> PhysicsContext {
  PhysicsContext {
    pipeline: PhysicsPipeline::new(),
    integration_parameters: IntegrationParameters::default(),
    broad_phase: BroadPhase::new(),
    narrow_pase: NarrowPhase::new(),
    bodies: RigidBodySet::new(),
    colliders: ColliderSet::new(),
    joints: JointSet::new(),
  }
}
