use ale_autoid::*;
use ale_buffer::{Buffer, BufferBuilder};
use ale_math::transform::AleTransform;
use ale_math::{Array, InnerSpace, Matrix4, Vector2, Vector3, Zero};

pub mod iter;
pub mod sdf;

const VERTEX: &str = "position";
const NORMAL: &str = "normal";
const UV: &str = "uv";
const BARYCENTRIC: &str = "barycentric";

#[derive(Debug)]
pub struct Mesh {
  pub id: MeshId,
  pub vertices: Buffer<f32>,
  pub indices: Option<Buffer<i32>>,
  pub bounding_box: (Vector3<f32>, Vector3<f32>),

  // Cache so this can be faster
  pub position_offset: Option<usize>,
  pub uv_offset: Option<usize>,
  pub normal_offset: Option<usize>,
}

pub struct Tri {
  position: [Vector3<f32>; 3],
  normal: [Vector3<f32>; 3],
  tri_normal: Vector3<f32>,
  uv: [Vector2<f32>; 3],
}

struct_id!(MeshId);
struct_id_impl!(MeshId, Mesh, id);

impl Mesh {
  pub fn new(
    vertices: Buffer<f32>,
    indices: Option<Buffer<i32>>,
    bounding_box: (Vector3<f32>, Vector3<f32>),
  ) -> Mesh {
    let position_offset = vertices.offset("position");
    let uv_offset = vertices.offset("uv");
    let normal_offset = vertices.offset("normal");

    Mesh {
      id: MeshId::new(),
      vertices,
      indices,
      bounding_box,
      position_offset,
      uv_offset,
      normal_offset,
    }
  }

  pub fn new_cube() -> Mesh {
    let vertices = BufferBuilder::new(vec![
      // back face
      -1.0f32, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // bottom-left
      1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, // top-right
      1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 0.0, // bottom-right
      1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, // top-right
      -1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // bottom-left
      -1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0, // top-left
      // front face
      -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom-left
      1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, // bottom-right
      1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // top-right
      1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // top-right
      -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, // top-left
      -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom-left
      // left face
      -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // top-right
      -1.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, 1.0, // top-left
      -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // bottom-left
      -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // bottom-left
      -1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, // bottom-right
      -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // top-right
      // right face
      1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, // top-left
      1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, // bottom-right
      1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top-right
      1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, // bottom-right
      1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, // top-left
      1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, // bottom-left
      // bottom face
      -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, // top-right
      1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 1.0, 1.0, // top-left
      1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, // bottom-left
      1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, // bottom-left
      -1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, // bottom-right
      -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, // top-right
      // top face
      -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top-left
      1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom-right
      1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0, // top-right
      1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom-right
      -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top-left
      -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, // bottom-left
    ])
      .info("position", 3)
      .info("normal", 3)
      .info("uv", 2)
      .build()
      .unwrap();

    let bounding_box = (Vector3::from_value(-1.0), Vector3::from_value(1.0));

    Mesh::new(vertices, None, bounding_box)
  }

  pub fn new_plane() -> Mesh {
    let vertices = BufferBuilder::new(vec![0.0f32, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0])
      .info("position", 2)
      .build()
      .unwrap();

    let bounding_box = (Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 0.0));

    Mesh::new(vertices, None, bounding_box)
  }
}





pub fn ale_mesh_ndc_plane_new() -> Mesh {
  let vertices = BufferBuilder::new(vec![
    -1.0f32, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 0.0, 1.0,
    1.0, 1.0, 1.0,
  ])
  .info("position", 2)
  .info("texcoords", 2)
  .build()
  .unwrap();

  let bounding_box = (Vector3::new(-1.0, -1.0, 0.0), Vector3::new(1.0, 1.0, 0.0));

  Mesh::new(vertices, None, bounding_box)
}

pub fn ale_mesh_bounding_box_new() -> Mesh {
  let vec = vec![
    // back face
    -1.0f32, -1.0, -1.0, 1.0, 0.0, 0.0, //
    1.0, 1.0, -1.0, 0.0, 1.0, 0.0, //
    1.0, -1.0, -1.0, 0.0, 0.0, 1.0, //
    1.0, 1.0, -1.0, 1.0, 0.0, 0.0, //
    -1.0, -1.0, -1.0, 0.0, 1.0, 0.0, //
    -1.0, 1.0, -1.0, 0.0, 0.0, 1.0, //
    // front face
    -1.0, -1.0, 1.0, 1.0, 0.0, 0.0, //
    1.0, -1.0, 1.0, 0.0, 1.0, 0.0, //
    1.0, 1.0, 1.0, 0.0, 0.0, 1.0, //
    1.0, 1.0, 1.0, 1.0, 0.0, 0.0, //
    -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, //
    -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, //
    // left face
    -1.0, 1.0, 1.0, 1.0, 0.0, 0.0, //
    -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, //
    -1.0, -1.0, -1.0, 0.0, 0.0, 1.0, //
    -1.0, -1.0, -1.0, 1.0, 0.0, 0.0, //
    -1.0, -1.0, 1.0, 0.0, 1.0, 0.0, //
    -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, //
    // right face
    1.0, 1.0, 1.0, 1.0, 0.0, 0.0, //
    1.0, -1.0, -1.0, 0.0, 1.0, 0.0, //
    1.0, 1.0, -1.0, 0.0, 0.0, 1.0, //
    1.0, -1.0, -1.0, 1.0, 0.0, 0.0, //
    1.0, 1.0, 1.0, 0.0, 1.0, 0.0, //
    1.0, -1.0, 1.0, 0.0, 0.0, 1.0, //
    // bottom face
    -1.0, -1.0, -1.0, 1.0, 0.0, 0.0, //
    1.0, -1.0, -1.0, 0.0, 1.0, 0.0, //
    1.0, -1.0, 1.0, 0.0, 0.0, 1.0, //
    1.0, -1.0, 1.0, 1.0, 0.0, 0.0, //
    -1.0, -1.0, 1.0, 0.0, 1.0, 0.0, //
    -1.0, -1.0, -1.0, 0.0, 0.0, 1.0, //
    // top face
    -1.0, 1.0, -1.0, 1.0, 0.0, 0.0, //
    1.0, 1.0, 1.0, 0.0, 1.0, 0.0, //
    1.0, 1.0, -1.0, 0.0, 0.0, 1.0, //
    1.0, 1.0, 1.0, 1.0, 0.0, 0.0, //
    -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, //
    -1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
  ];

  let vertices = BufferBuilder::new(vec)
    .info("position", 3)
    .info("barycentric", 3)
    .build()
    .unwrap();

  let bounding_box = (Vector3::from_value(-1.0), Vector3::from_value(1.0));

  Mesh::new(vertices, None, bounding_box)
}

pub fn ale_mesh_bounding_box_matrix(bounding_box: (Vector3<f32>, Vector3<f32>)) -> Matrix4<f32> {
  let (min, max) = bounding_box;
  let size = (max - min) / 2.0;
  let center = (max + min) / 2.0;

  //println!("min: {:?}, max: {:?}, center: {:?}, size: {:?}", min, max, center, size);

  // let transform = Transform::from_position_scale(center, size);
  // transform

  Matrix4::from_translation(center) * Matrix4::from_nonuniform_scale(size.x, size.y, size.z)
}

pub fn ale_mesh_tri_len(mesh: &Mesh) -> usize {
  match &mesh.indices {
    None => mesh.vertices.total_row_len() / 3,
    Some(ind) => ind.len() / 3,
  }
}

pub fn ale_mesh_tri_get(mesh: &Mesh, i: usize) -> Option<Tri> {
  let column_len = mesh.vertices.total_column_len();
  if i > ale_mesh_tri_len(mesh) {
    return None;
  }

  let vertex_offset = mesh.position_offset.expect("This mesh doesn't have positions");
  let uv_offset = mesh.uv_offset.expect("This mesh doesn't have UVs");
  let normal_offset = mesh.normal_offset.expect("This mesh doesn't have normal");

  let (s0, s1, s2) = match &mesh.indices {
    None => {
      let s0 = i * 3 * column_len;
      let s1 = (i * 3 + 1) * column_len;
      let s2 = (i * 3 + 2) * column_len;
      (s0, s1, s2)
    }
    Some(ind) => {
      let s0 = ind[i * 3] as usize * column_len;
      let s1 = ind[i * 3 + 1] as usize * column_len;
      let s2 = ind[i * 3 + 2] as usize * column_len;
      (s0, s1, s2)
    }
  };

  let vert = &mesh.vertices;
  let (p0, p1, p2) = (s0 + vertex_offset, s1 + vertex_offset, s2 + vertex_offset);
  let position = [
    Vector3::new(vert[p0], vert[p0 + 1], vert[p0 + 2]),
    Vector3::new(vert[p1], vert[p1 + 1], vert[p1 + 2]),
    Vector3::new(vert[p2], vert[p2 + 1], vert[p2 + 2]),
  ];
  let (u0, u1, u2) = (s0 + uv_offset, s1 + uv_offset, s2 + uv_offset);
  let uv = [
    Vector2::new(vert[u0], vert[u0 + 1]),
    Vector2::new(vert[u1], vert[u1 + 1]),
    Vector2::new(vert[u2], vert[u2 + 1]),
  ];

  let (n0, n1, n2) = (s0 + normal_offset, s1 + normal_offset, s2 + normal_offset);
  let normal = [
    Vector3::new(vert[n0], vert[n0 + 1], vert[n0 + 2]),
    Vector3::new(vert[n1], vert[n1 + 1], vert[n1 + 2]),
    Vector3::new(vert[n2], vert[n2 + 1], vert[n2 + 2]),
  ];
  let tri_normal = Vector3::normalize((position[1] - position[0]).cross(position[2] - position[0]));

  Some(Tri {
    position,
    normal,
    tri_normal,
    uv,
  })
}

#[test]
pub fn test_tri_get_no_ebo() {
  use approx::relative_eq;

  let data: Vec<f32> = vec![
    // vertices (3), uv(2), normals(3)
    1.0, 1.0, 1.0, 10.0, 10.0, -1.0, -1.0, -1.0, 2.0, 2.0, 2.0, 20.0, 20.0, -2.0, -2.0, -3.0, 3.0, 3.0, 3.0, 30.0, 30.0,
    -2.0, -2.0, -3.0, 4.0, 1.0, 1.0, 15.0, 10.0, -1.0, -5.0, -1.0, 5.0, 2.0, 2.0, 25.0, 20.0, -2.0, -5.0, -3.0, 6.0,
    3.0, 3.0, 35.0, 30.0, -2.0, -5.0, -3.0, 10.0, 1.0, 1.0, 7.0, 10.0, -3.0, -5.0, -1.0, 12.0, 2.0, 2.0, 17.0, 20.0,
    -3.0, -5.0, -3.0, 13.0, 3.0, 3.0, 27.0, 30.0, -3.0, -5.0, -3.0,
  ];

  /*

   1.0, 1.0, 1.0, 10.0, 10.0, -1.0, -1.0, -1.0,
   2.0, 2.0, 2.0, 20.0, 20.0, -2.0, -2.0, -3.0,
   3.0, 3.0, 3.0, 30.0, 30.0, -2.0, -2.0, -3.0,

   4.0, 1.0, 1.0, 15.0, 10.0, -1.0, -5.0, -1.0,
   5.0, 2.0, 2.0, 25.0, 20.0, -2.0, -5.0, -3.0,
   6.0, 3.0, 3.0, 35.0, 30.0, -2.0, -5.0, -3.0,

   10.0, 1.0, 1.0, 7.0, 10.0, -3.0, -5.0, -1.0,
   12.0, 2.0, 2.0, 17.0, 20.0,-3.0, -5.0, -3.0,
   13.0, 3.0, 3.0, 27.0, 30.0, -3.0, -5.0, -3.0,
  */

  let buffer: Buffer<f32> = BufferBuilder::new(data)
    .info("position", 3)
    .info("uv", 2)
    .info("normal", 3)
    .build()
    .unwrap();

  let mesh = Mesh::new(buffer, None, (Vector3::zero(), Vector3::zero()));

  assert_eq!(ale_mesh_tri_len(&mesh), 3);

  let tri = ale_mesh_tri_get(&mesh, 0).unwrap();
  assert_eq!(tri.position[0], Vector3::new(1.0, 1.0, 1.0,));
  assert_eq!(tri.position[1], Vector3::new(2.0, 2.0, 2.0,));
  assert_eq!(tri.position[2], Vector3::new(3.0, 3.0, 3.0,));

  assert_eq!(tri.uv[0], Vector2::new(10.0, 10.0));
  assert_eq!(tri.uv[1], Vector2::new(20.0, 20.0));
  assert_eq!(tri.uv[2], Vector2::new(30.0, 30.0));

  assert_eq!(tri.normal[0], Vector3::new(-1.0, -1.0, -1.0));
  assert_eq!(tri.normal[1], Vector3::new(-2.0, -2.0, -3.0));
  assert_eq!(tri.normal[2], Vector3::new(-2.0, -2.0, -3.0));

  let tri = ale_mesh_tri_get(&mesh, 2).unwrap();
  assert_eq!(tri.position[0], Vector3::new(10.0, 1.0, 1.0,));
  assert_eq!(tri.position[1], Vector3::new(12.0, 2.0, 2.0,));
  assert_eq!(tri.position[2], Vector3::new(13.0, 3.0, 3.0,));

  assert_eq!(tri.uv[0], Vector2::new(7.0, 10.0));
  assert_eq!(tri.uv[1], Vector2::new(17.0, 20.0));
  assert_eq!(tri.uv[2], Vector2::new(27.0, 30.0));

  assert_eq!(tri.normal[0], Vector3::new(-3.0, -5.0, -1.0));
  assert_eq!(tri.normal[1], Vector3::new(-3.0, -5.0, -3.0));
  assert_eq!(tri.normal[2], Vector3::new(-3.0, -5.0, -3.0));
}

#[test]
pub fn test_tri_cube_get_no_ebo() {
  use approx::relative_eq;

  let mesh = Mesh::new_cube();

  assert_eq!(ale_mesh_tri_len(&mesh), 12);

  let mut res: Vec<f32> = vec![];
  for i in 0..ale_mesh_tri_len(&mesh) {
    let tri = ale_mesh_tri_get(&mesh, i).unwrap();

    res.extend_from_slice(&[tri.position[0][0], tri.position[0][1], tri.position[0][2]]);
    res.extend_from_slice(&[tri.normal[0][0], tri.normal[0][1], tri.normal[0][2]]);
    res.extend_from_slice(&[tri.uv[0][0], tri.uv[0][1]]);

    res.extend_from_slice(&[tri.position[1][0], tri.position[1][1], tri.position[1][2]]);
    res.extend_from_slice(&[tri.normal[1][0], tri.normal[1][1], tri.normal[1][2]]);
    res.extend_from_slice(&[tri.uv[1][0], tri.uv[1][1]]);

    res.extend_from_slice(&[tri.position[2][0], tri.position[2][1], tri.position[2][2]]);
    res.extend_from_slice(&[tri.normal[2][0], tri.normal[2][1], tri.normal[2][2]]);
    res.extend_from_slice(&[tri.uv[2][0], tri.uv[2][1]]);
  }
  assert_eq!(
    res,
    vec![
      // back face
      -1.0f32, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // bottom-left
      1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, // top-right
      1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 0.0, // bottom-right
      1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 1.0, 1.0, // top-right
      -1.0, -1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 0.0, // bottom-left
      -1.0, 1.0, -1.0, 0.0, 0.0, -1.0, 0.0, 1.0, // top-left
      // front face
      -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom-left
      1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, // bottom-right
      1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // top-right
      1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // top-right
      -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, // top-left
      -1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom-left
      // left face
      -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // top-right
      -1.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, 1.0, // top-left
      -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // bottom-left
      -1.0, -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 1.0, // bottom-left
      -1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, // bottom-right
      -1.0, 1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, // top-right
      // right face
      1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, // top-left
      1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, // bottom-right
      1.0, 1.0, -1.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top-right
      1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 1.0, // bottom-right
      1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, // top-left
      1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, // bottom-left
      // bottom face
      -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, // top-right
      1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 1.0, 1.0, // top-left
      1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, // bottom-left
      1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, // bottom-left
      -1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, // bottom-right
      -1.0, -1.0, -1.0, 0.0, -1.0, 0.0, 0.0, 1.0, // top-right
      // top face
      -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top-left
      1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom-right
      1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 1.0, // top-right
      1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom-right
      -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, 1.0, // top-left
      -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, // bottom-left
    ]
  );
}

#[test]
pub fn test_tri_get_with_ebo() {
  use approx::relative_eq;

  let data: Vec<f32> = vec![
    // vertices (3), uv(2), normals(3)
    1.0, 1.0, 1.0, 10.0, 10.0, -1.0, -1.0, -1.0, 2.0, 2.0, 2.0, 20.0, 20.0, -2.0, -2.0, -3.0, 3.0, 3.0, 3.0, 30.0, 30.0,
    -2.0, -2.0, -3.0, 4.0, 1.0, 1.0, 15.0, 10.0, -1.0, -5.0, -1.0, 5.0, 2.0, 2.0, 25.0, 20.0, -2.0, -5.0, -3.0, 6.0,
    3.0, 3.0, 35.0, 30.0, -2.0, -5.0, -3.0, 10.0, 1.0, 1.0, 7.0, 10.0, -3.0, -5.0, -1.0, 12.0, 2.0, 2.0, 17.0, 20.0,
    -3.0, -5.0, -3.0, 13.0, 3.0, 3.0, 27.0, 30.0, -3.0, -5.0, -3.0,
  ];

  /*
   1.0, 1.0, 1.0, 10.0, 10.0, -1.0, -1.0, -1.0,
   2.0, 2.0, 2.0, 20.0, 20.0, -2.0, -2.0, -3.0,
   3.0, 3.0, 3.0, 30.0, 30.0, -2.0, -2.0, -3.0,

   4.0, 1.0, 1.0, 15.0, 10.0, -1.0, -5.0, -1.0,
   5.0, 2.0, 2.0, 25.0, 20.0, -2.0, -5.0, -3.0,
   6.0, 3.0, 3.0, 35.0, 30.0, -2.0, -5.0, -3.0,

   10.0, 1.0, 1.0, 7.0, 10.0, -3.0, -5.0, -1.0,
   12.0, 2.0, 2.0, 17.0, 20.0,-3.0, -5.0, -3.0,
   13.0, 3.0, 3.0, 27.0, 30.0, -3.0, -5.0, -3.0,
  */

  let buffer: Buffer<f32> = BufferBuilder::new(data)
    .info("position", 3)
    .info("uv", 2)
    .info("normal", 3)
    .build()
    .unwrap();

  let indices: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 2, 5, 8, 4, 1, 7];
  let ibuffer = BufferBuilder::new(indices).info("index", 3).build().unwrap();

  let mesh = Mesh::new(buffer, Some(ibuffer), (Vector3::zero(), Vector3::zero()));

  assert_eq!(ale_mesh_tri_len(&mesh), 5);

  let tri = ale_mesh_tri_get(&mesh, 0).unwrap();
  assert_eq!(tri.position[0], Vector3::new(1.0, 1.0, 1.0,));
  assert_eq!(tri.position[1], Vector3::new(2.0, 2.0, 2.0,));
  assert_eq!(tri.position[2], Vector3::new(3.0, 3.0, 3.0,));

  assert_eq!(tri.uv[0], Vector2::new(10.0, 10.0));
  assert_eq!(tri.uv[1], Vector2::new(20.0, 20.0));
  assert_eq!(tri.uv[2], Vector2::new(30.0, 30.0));

  assert_eq!(tri.normal[0], Vector3::new(-1.0, -1.0, -1.0));
  assert_eq!(tri.normal[1], Vector3::new(-2.0, -2.0, -3.0));
  assert_eq!(tri.normal[2], Vector3::new(-2.0, -2.0, -3.0));

  let tri = ale_mesh_tri_get(&mesh, 2).unwrap();
  assert_eq!(tri.position[0], Vector3::new(10.0, 1.0, 1.0,));
  assert_eq!(tri.position[1], Vector3::new(12.0, 2.0, 2.0,));
  assert_eq!(tri.position[2], Vector3::new(13.0, 3.0, 3.0,));

  assert_eq!(tri.uv[0], Vector2::new(7.0, 10.0));
  assert_eq!(tri.uv[1], Vector2::new(17.0, 20.0));
  assert_eq!(tri.uv[2], Vector2::new(27.0, 30.0));

  assert_eq!(tri.normal[0], Vector3::new(-3.0, -5.0, -1.0));
  assert_eq!(tri.normal[1], Vector3::new(-3.0, -5.0, -3.0));
  assert_eq!(tri.normal[2], Vector3::new(-3.0, -5.0, -3.0));

  // 5.0, 2.0, 2.0, 25.0, 20.0, -2.0, -5.0, -3.0,
  // 2.0, 2.0, 2.0, 20.0, 20.0, -2.0, -2.0, -3.0,
  // 12.0, 2.0, 2.0, 17.0, 20.0,-3.0, -5.0, -3.0,

  let tri = ale_mesh_tri_get(&mesh, 4).unwrap();
  assert_eq!(tri.position[0], Vector3::new(5.0, 2.0, 2.0,));
  assert_eq!(tri.position[1], Vector3::new(2.0, 2.0, 2.0,));
  assert_eq!(tri.position[2], Vector3::new(12.0, 2.0, 2.0,));

  assert_eq!(tri.uv[0], Vector2::new(25.0, 20.0));
  assert_eq!(tri.uv[1], Vector2::new(20.0, 20.0));
  assert_eq!(tri.uv[2], Vector2::new(17.0, 20.0));

  assert_eq!(tri.normal[0], Vector3::new(-2.0, -5.0, -3.0));
  assert_eq!(tri.normal[1], Vector3::new(-2.0, -2.0, -3.0));
  assert_eq!(tri.normal[2], Vector3::new(-3.0, -5.0, -3.0));
}
