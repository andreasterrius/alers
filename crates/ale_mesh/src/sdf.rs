use crate::iter::ale_mesh_triangle_iter_new;
use crate::Mesh;
use ale_math::num_traits::clamp;
use ale_math::{dot, InnerSpace};
use ale_math::{vec1, Vector3};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::cmp::Ordering;
use std::time::Instant;

pub struct MeshSDF {
  dist: Vec<Vec<Vec<f32>>>,
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

// find distance from triangle to point, copied from https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
pub fn ale_mesh_point_triangle_closest_dist(tri: (Vector3<f32>, Vector3<f32>, Vector3<f32>), p: Vector3<f32>) -> f32 {
  let (a, b, c) = tri;
  let ba = b - a;
  let cb = c - b;
  let ac = a - c;
  let pa = p - a;
  let pb = p - b;
  let pc = p - c;
  let nor = ba.cross(ac);

  let s = (sign(dot(cross(ba, nor), pa)) + sign(dot(cross(cb, nor), pb)) + sign(dot(cross(ac, nor), pc)) < 2.0);
  let r = if s {
    f32::min(
      f32::min(
        dot2(ba * clamp(dot(ba, pa) / dot2(ba), 0.0, 1.0) - pa),
        dot2(cb * clamp(dot(cb, pb) / dot2(cb), 0.0, 1.0) - pb),
      ),
      dot2(ac * clamp(dot(ac, pc) / dot2(ac), 0.0, 1.0) - pc),
    )
  } else {
    dot(nor, pa) * dot(nor, pa) / dot2(nor)
  };

  f32::sqrt(r)
}

pub fn ale_mesh_sdf_new(mesh: &Mesh, reso: u32) -> MeshSDF {
  let start_time = Instant::now();

  let (min, max) = mesh.bounding_box;
  let size = max - min;
  let step = size / reso as f32;
  let offset = step / 2.0;

  let mut dist = vec![vec![vec![0.0; reso as usize]; reso as usize]; reso as usize];
  for i in 0..reso {
    let inner_start_time = Instant::now();
    for j in 0..reso {
      for k in 0..reso {
        let x = offset.x + step.x * i as f32;
        let y = offset.y + step.y * j as f32;
        let z = offset.z + step.z * k as f32;
        let xyz = Vector3::new(x, y, z);

        //let mut min_dist = f32::MAX;
        // for tri in ale_mesh_triangle_iter_new(mesh) {
        //   let dist = ale_mesh_point_triangle_closest_dist(tri, xyz.clone());
        //   min_dist = f32::min(dist, min_dist);
        // }
        let mut min_dist = ale_mesh_triangle_iter_new(mesh)
          .par_bridge()
          .map(|tri| ale_mesh_point_triangle_closest_dist(tri, xyz.clone()))
          .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Greater))
          .unwrap();

        dist[i as usize][j as usize][k as usize] = min_dist;
      }
    }
    println!(
      "{}: {} ms",
      i,
      Instant::now().duration_since(inner_start_time).as_millis()
    );
  }

  println!(
    "SDF generation done : {} ms",
    Instant::now().duration_since(start_time).as_millis()
  );

  MeshSDF { dist }
}
