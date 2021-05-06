use crate::iter::ale_mesh_triangle_iter_new;
use crate::{ale_mesh_tri_get, ale_mesh_tri_len, Mesh, Tri};
use ale_math::num_traits::clamp;
use ale_math::{ale_bounding_box_closest_point, ale_bounding_box_size, dot, InnerSpace, MetricSpace, Zero};
use ale_math::{vec1, Vector3};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::cmp::Ordering;
use std::time::Instant;

pub struct MeshSDF {
  dist: Vec<Vec<Vec<f32>>>,

  mesh_bounding_box: (Vector3<f32>, Vector3<f32>),
  initial: Vector3<f32>,
  step: Vector3<f32>,

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

// copied from https://docs.ros.org/en/kinetic/api/geometric_tools_engine/html/GteDistPointTriangleExact_8h_source.html
pub fn ale_mesh_point_triangle_closest_point(triangle: &Tri, point: Vector3<f32>) -> Vector3<f32> {
  let diff = point - triangle.position[0];
  let edge0 = triangle.position[1] - triangle.position[0];
  let edge1 = triangle.position[2] - triangle.position[0];
  let a00 = dot(edge0, edge0);
  let a01 = dot(edge0, edge1);
  let a11 = dot(edge1, edge1);
  let b0 = -dot(diff, edge0);
  let b1 = -dot(diff, edge1);
  let mut det = a00 * a11 - a01 * a01;
  let mut t0 = a01 * b1 - a11 * b0;
  let mut t1 = a01 * b0 - a00 * b1;

  if t0 + t1 <= det {
    if t0 < 0.0 {
      if t1 < 0.0
      // region 4
      {
        if b0 < 0.0 {
          t1 = 0.0;
          if -b0 >= a00
          // V1
          {
            t0 = 1.0;
          } else
          // E01
          {
            t0 = -b0 / a00;
          }
        } else {
          t0 = 0.0;
          if b1 >= 0.0
          // V0
          {
            t1 = 0.0;
          } else if -b1 >= a11
          // V2
          {
            t1 = 1.0;
          } else
          // E20
          {
            t1 = -b1 / a11;
          }
        }
      } else
      // region 3
      {
        t0 = 0.0;
        if b1 >= 0.0
        // V0
        {
          t1 = 0.0;
        } else if -b1 >= a11
        // V2
        {
          t1 = 1.0;
        } else
        // E20
        {
          t1 = -b1 / a11;
        }
      }
    } else if t1 < 0.0
    // region 5
    {
      t1 = 0.0;
      if b0 >= 0.0
      // V0
      {
        t0 = 0.0;
      } else if -b0 >= a00
      // V1
      {
        t0 = 1.0;
      } else
      // E01
      {
        t0 = -b0 / a00;
      }
    } else
    // region 0, interior
    {
      let inv_det = 1.0 / det;
      t0 *= inv_det;
      t1 *= inv_det;
    }
  } else {
    let mut tmp0;
    let mut tmp1;
    let mut numer;
    let mut denom;

    if t0 < 0.0
    // region 2
    {
      tmp0 = a01 + b0;
      tmp1 = a11 + b1;
      if tmp1 > tmp0 {
        numer = tmp1 - tmp0;
        denom = a00 - 2.0 * a01 + a11;
        if numer >= denom
        // V1
        {
          t0 = 1.0;
          t1 = 0.0;
        } else
        // E12
        {
          t0 = numer / denom;
          t1 = 1.0 - t0;
        }
      } else {
        t0 = 0.0;
        if tmp1 <= 0.0
        // V2
        {
          t1 = 1.0;
        } else if b1 >= 0.0
        // V0
        {
          t1 = 0.0;
        } else
        // E20
        {
          t1 = -b1 / a11;
        }
      }
    } else if t1 < 0.0
    // region 6
    {
      tmp0 = a01 + b1;
      tmp1 = a00 + b0;
      if tmp1 > tmp0 {
        numer = tmp1 - tmp0;
        denom = a00 - 2.0 * a01 + a11;
        if numer >= denom
        // V2
        {
          t1 = 1.0;
          t0 = 0.0;
        } else
        // E12
        {
          t1 = numer / denom;
          t0 = 1.0 - t1;
        }
      } else {
        t1 = 0.0;
        if tmp1 <= 0.0
        // V1
        {
          t0 = 1.0;
        } else if b0 >= 0.0
        // V0
        {
          t0 = 0.0;
        } else
        // E01
        {
          t0 = -b0 / a00;
        }
      }
    } else
    // region 1
    {
      numer = a11 + b1 - a01 - b0;
      if numer <= 0.0
      // V2
      {
        t0 = 0.0;
        t1 = 1.0;
      } else {
        denom = a00 - 2.0 * a01 + a11;
        if numer >= denom
        // V1
        {
          t0 = 1.0;
          t1 = 0.0;
        } else
        // 12
        {
          t0 = numer / denom;
          t1 = 1.0 - t0;
        }
      }
    }
  }

  return triangle.position[0] + t0 * edge0 + t1 * edge1;
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
        let mut should_flip = false;
        let mut min_point = Vector3::zero();
        let tri_len = ale_mesh_tri_len(mesh);
        for tri_idx in 0..tri_len {
          let tri = ale_mesh_tri_get(mesh, tri_idx).unwrap();
          let point = ale_mesh_point_triangle_closest_point(&tri, xyz.clone());

          let dist = point.distance(xyz.clone());
          if min_dist > dist {
            min_dist = dist;
            should_flip = if dot((point - xyz.clone()).normalize(), tri.tri_normal) < 0.0 {
              true
            } else {
              false
            };
            min_point = point.clone();
          }
        }

        if should_flip {
          min_dist = -min_dist;
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

  MeshSDF {
    dist,
    mesh_bounding_box : mesh.bounding_box.clone(),
    initial,
    step,
    points,
  }
}

pub fn ale_mesh_sdf_distance(sdf: &MeshSDF, point: Vector3<f32>) -> f32 {
  let point_in_sdf = ale_bounding_box_closest_point(point.clone(), sdf.mesh_bounding_box);
  let (i, j, k) = ale_mesh_sdf_find_quadrant(sdf, point_in_sdf);
  //println!("{} {} {}", i, j, k);
  let dori = point.distance(point_in_sdf);
  let dsdf = -sdf.dist[i][j][k];

  println!("{} {}", dori, dsdf);

  dori + dsdf
}

pub fn ale_mesh_sdf_find_quadrant(sdf: &MeshSDF, point: Vector3<f32>) -> (usize, usize, usize) {
  let find_point = point - sdf.initial;
  let i = find_point.x / sdf.step.x;
  let j = find_point.y / sdf.step.y;
  let k = find_point.z / sdf.step.z;
  return (i as usize, j as usize, k as usize);
}
