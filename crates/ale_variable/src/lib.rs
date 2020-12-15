use ale_math::{Vector3, Vector4};
use std::hash::{Hash, Hasher};

pub trait ToVariable {
  fn to_variable(&self, name: &str) -> Variable;
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Variable {
  F32_1(String, f32),
  F32_3(String, Vector3<f32>),
  F32_4(String, Vector4<f32>),
  Bool(String, bool),
  Void(String),
}

impl Variable {
  pub fn name_str(&self) -> String {
    match self {
      Variable::F32_1(n, _) => n.to_owned(),
      Variable::F32_3(n, _) => n.to_owned(),
      Variable::F32_4(n, _) => n.to_owned(),
      Variable::Bool(n, _) => n.to_owned(),
      Variable::Void(n) => n.to_owned(),
    }
  }

  pub fn value_str(&self) -> String {
    match self {
      Variable::F32_1(n, v) => v.to_string(),
      Variable::F32_3(n, v) => format!("({}, {}, {})", v.x, v.y, v.z),
      Variable::F32_4(n, v) => format!("({}, {}, {}, {})", v.x, v.y, v.z, v.w),
      Variable::Bool(n, v) => v.to_string(),
      Variable::Void(n) => n.to_owned(),
    }
  }
}

impl ToVariable for f32 {
  fn to_variable(&self, name: &str) -> Variable {
    Variable::F32_1(name.to_owned(), *self)
  }
}

impl ToVariable for bool {
  fn to_variable(&self, name: &str) -> Variable {
    Variable::Bool(name.to_owned(), *self)
  }
}

impl ToVariable for Vector3<f32> {
  fn to_variable(&self, name: &str) -> Variable {
    Variable::F32_3(name.to_owned(), *self)
  }
}

impl ToVariable for Vector4<f32> {
  fn to_variable(&self, name: &str) -> Variable {
    Variable::F32_4(name.to_owned(), *self)
  }
}

impl From<Variable> for f32 {
  fn from(v: Variable) -> Self {
    match v {
      Variable::F32_1(_, v) => v.clone(),
      _ => {
        unimplemented!("variable mismatch, f32 expected {}", v.name_str())
      }
    }
  }
}

impl From<Variable> for bool {
  fn from(v: Variable) -> Self {
    match v {
      Variable::Bool(_, v) => v.clone(),
      _ => {
        unimplemented!("variable mismatch, f32 expected {}", v.name_str())
      }
    }
  }
}

#[macro_export]
macro_rules! to_variable {
  ($e:ident) => {
    $e.to_variable(stringify!($e));
  };

  ($e:ident.$v:ident) => {
    $e.$v.to_variable(stringify!($v));
  };

  ($e1:ident.$e2:ident.$v:ident) => {
    $e1.$e2.$v.to_variable(stringify!($v));
  };

  ($e1:ident.$e2:ident.$e3:ident.$v:ident) => {
    $e1.$e2.$e3.$v.to_variable(stringify!($v));
  };
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
