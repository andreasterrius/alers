use crate::math::transform::Transform;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub struct Bone {
  child: Vec<Rc<Bone>>,
  local_transform: Transform,
  combined_transform: Option<Transform>,

  // vertex id => weight (normalized 0->1)
  weights: HashMap<i32, f32>,
}
