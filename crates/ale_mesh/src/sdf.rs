use crate::iter::ale_mesh_triangle_iter_new;
use crate::{ale_mesh_tri_get, ale_mesh_tri_len, Mesh, Tri};
use ale_math::num_traits::clamp;
use ale_math::{dot, InnerSpace, MetricSpace, Zero};
use ale_math::{vec1, Vector3};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::cmp::Ordering;
use std::time::Instant;

pub struct MeshSDF {
  dist: Vec<Vec<Vec<f32>>>,

  // for debug purposes
  pub points: Vec<(Vector3<f32>, Vector3<f32>, f32)>, // from, to, distance (minus on inside)
}

pub fn sign(x: f32) -> f32 {
  assert!(!x.is_nan());
  return if x == 0.0 {
    0.0
  } else if x > 0.0 {
    1.0
  } else {
    -1.0
  };
}

fn dot2(a: Vector3<f32>) -> f32 {
  dot(a, a)
}

fn cross(a: Vector3<f32>, b: Vector3<f32>) -> Vector3<f32> {
  a.cross(b)
}

// copied from https://www.gamedev.net/forums/topic/552906-closest-point-on-triangle/
pub fn ale_mesh_point_triangle_closest_point(tri: &Tri, p: Vector3<f32>) -> Vector3<f32> {
  let edge0 = tri.position[1] - tri.position[0];
  let edge1 = tri.position[2] - tri.position[0];
  let v0 = tri.position[0] - p;

  let a = edge0.dot(edge0);
  let b = edge0.dot(edge1);
  let c = edge1.dot(edge1);
  let d = edge0.dot(v0);
  let e = edge1.dot(v0);

  let det = a * c - b * b;
  let mut s = b * e - c * d;
  let mut t = b * d - a * e;

  if s + t < det {
    if s < 0.0 {
      if t < 0.0 {
        if d < 0.0 {
          s = clamp(-d / a, 0.0, 1.0);
          t = 0.0;
        } else {
          s = 0.0;
          t = clamp(-e / c, 0.0, 1.0);
        }
      } else {
        s = 0.0;
        t = clamp(-e / c, 0.0, 1.0);
      }
    } else if t < 0.0 {
      s = clamp(-d / a, 0.0, 1.0);
      t = 0.0;
    } else {
      let inv_det = 1.0 / det;
      s *= inv_det;
      t *= inv_det;
    }
  } else {
    if s < 0.0 {
      let tmp0 = b + d;
      let tmp1 = c + e;
      if tmp1 > tmp0 {
        let numer = tmp1 - tmp0;
        let denom = a - 2.0 * b + c;
        s = clamp(numer / denom, 0.0, 1.0);
        t = 1.0 - s;
      } else {
        t = clamp(-e / c, 0.0, 1.0);
        s = 0.0;
      }
    } else if t < 0.0 {
      if a + d > b + e {
        let numer = c + e - b - d;
        let denom = a - 2.0 * b + c;
        s = clamp(numer / denom, 0.0, 1.0);
        t = 1.0 - s;
      } else {
        s = clamp(-e / c, 0.0, 1.0);
        t = 0.0;
      }
    } else {
      let numer = c + e - b - d;
      let denom = a - 2.0 * b + c;
      s = clamp(numer / denom, 0.0, 1.0);
      t = 1.0 - s;
    }
  }

  return tri.position[0] + s * edge0 + t * edge1;
}

pub fn ale_mesh_sdf_new(mesh: &Mesh, reso: u32) -> MeshSDF {
  let start_time = Instant::now();

  let (min, max) = mesh.bounding_box;
  let size = (max - min);
  let sdf_size = size + 0.4f32 * size;
  let step = sdf_size / reso as f32;
  let initial = (min - 0.2f32 * size) + step / 2.0;

  let mut dist = vec![vec![vec![0.0; reso as usize]; reso as usize]; reso as usize];
  let mut points = vec![];
  for i in 0..reso {
    let inner_start_time = Instant::now();
    for j in 0..reso {
      for k in 0..reso {
        let x = initial.x + step.x * i as f32;
        let y = initial.y + step.y * j as f32;
        let z = initial.z + step.z * k as f32;
        let xyz = Vector3::new(x, y, z);

        let mut min_dist = f32::MAX;
        let mut min_point = Vector3::zero();
        let tri_len = ale_mesh_tri_len(mesh);
        for tri_idx in 0..tri_len {
          let tri = ale_mesh_tri_get(mesh, tri_idx).unwrap();
          let point = ale_mesh_point_triangle_closest_point(&tri, xyz.clone());

          let dist = point.distance(xyz.clone());
          if min_dist > dist {
            min_dist = if dot((point - xyz.clone()).normalize(), tri.tri_normal) < 0.0 {
              -dist
            } else {
              dist
            };
            min_point = point.clone();
          }
        }

        dist[i as usize][j as usize][k as usize] = min_dist;
        points.push((xyz.clone(), min_point.clone(), min_dist));
      }
    }
    // println!(
    //   "{}: {} ms",
    //   i,
    //   Instant::now().duration_since(inner_start_time).as_millis()
    // );
  }

  //points.push((min, min));

  println!(
    "SDF generation done : {} ms",
    Instant::now().duration_since(start_time).as_millis()
  );

  MeshSDF { dist, points }
}

// #[test]
// fn test_ale_mesh_point_triangle_closest_point(){
//   let Tri = Tri {
//     position: [Vector3::new(1.0, )],
//     normal: [],
//     tri_normal: Vector3 {},
//     uv: []
//   }
// }
