use ale_math::{Vector3, Vector4};
use std::hash::{Hash, Hasher};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Variable {
  F32_1(String, f32),
  F32_3(String, Vector3<f32>),
  F32_4(String, Vector4<f32>),
}

pub fn ale_variable_name_get(variable: &Variable) -> String {
  match variable {
    Variable::F32_1(n, _) => n.to_owned(),
    Variable::F32_3(n, _) => n.to_owned(),
    Variable::F32_4(n, _) => n.to_owned(),
  }
}

pub fn ale_variable_value_to_string(variable: &Variable) -> String {
  match variable {
    Variable::F32_1(n, v) => v.to_string(),
    Variable::F32_3(n, v) => format!("({}, {}, {})", v.x, v.y, v.z),
    Variable::F32_4(n, v) => format!("({}, {}, {}, {})", v.x, v.y, v.z, v.w),
  }
}
// impl Hash for Variable {
//   fn hash<H: Hasher>(&self, state: &mut H) {
//     match self {
//       Variable::F32_1(n, _) => n.hash(state),
//       Variable::F32_3(n, _) => n.hash(state),
//       Variable::F32_4(n, _) => n.hash(state),
//     }
//   }
// }
//
// impl PartialEq for Variable {
//   fn eq(&self, other: &Self) -> bool {
//     let n = match self {
//       Variable::F32_1(n, _) => n,
//       Variable::F32_3(n, _) => n,
//       Variable::F32_4(n, _) => n,
//     };
//
//     let n2 = match other {
//       Variable::F32_1(n, _) => n,
//       Variable::F32_3(n, _) => n,
//       Variable::F32_4(n, _) => n,
//     };
//
//     n == n2
//   }
// }
// impl Eq for Variable {}
