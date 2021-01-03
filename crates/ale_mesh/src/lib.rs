use crate::buffer::{Buffer, BufferBuilder, SeparateBufferBuilder};
use ale_autoid::*;
use ale_math::transform::Transform;
use ale_math::{Array, Matrix4, Vector2, Vector3};

pub mod buffer;
pub mod iter;
pub mod sdf;

const VERTEX: &str = "vertex";
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
  pub vertex_offset: Option<usize>,
  pub uv_offset: Option<usize>,
  pub normal_offset: Option<usize>,
}

pub struct Tri {
  position: [Vector3<f32>; 3],
  normal: [Vector3<f32>; 3],
  uv: [Vector2<f32>; 3],
}

struct_id!(MeshId);
struct_id_impl!(MeshId, Mesh, id);

pub fn ale_mesh_new(
  vertices: Buffer<f32>,
  indices: Option<Buffer<i32>>,
  bounding_box: (Vector3<f32>, Vector3<f32>),
) -> Mesh {
  let vertex_offset = vertices.offset("vertex");
  let uv_offset = vertices.offset("uv");
  let normal_offset = vertices.offset("normal");

  Mesh {
    id: MeshId::new(),
    vertices,
    indices,
    bounding_box,
    vertex_offset,
    uv_offset,
    normal_offset,
  }
}

pub fn ale_mesh_cube_new() -> Mesh {
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
  .info("vertex", 3)
  .info("normal", 3)
  .info("uv", 2)
  .build()
  .unwrap();

  let bounding_box = (Vector3::from_value(-1.0), Vector3::from_value(1.0));

  ale_mesh_new(vertices, None, bounding_box)
}

pub fn ale_mesh_plane_new() -> Mesh {
  let vertices = BufferBuilder::new(vec![0.0f32, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0])
    .info("vertex", 2)
    .build()
    .unwrap();

  let bounding_box = (Vector3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 1.0, 0.0));

  ale_mesh_new(vertices, None, bounding_box)
}

pub fn ale_mesh_ndc_plane_new() -> Mesh {
  let vertices = BufferBuilder::new(vec![
    -1.0f32, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 0.0, 1.0,
    1.0, 1.0, 1.0,
  ])
  .info("vertex", 2)
  .info("texcoords", 2)
  .build()
  .unwrap();

  let bounding_box = (Vector3::new(-1.0, -1.0, 0.0), Vector3::new(1.0, 1.0, 0.0));

  ale_mesh_new(vertices, None, bounding_box)
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
    .info("vertex", 3)
    .info("barycentric", 3)
    .build()
    .unwrap();

  let bounding_box = (Vector3::from_value(-1.0), Vector3::from_value(1.0));

  ale_mesh_new(vertices, None, bounding_box)
}

pub fn ale_mesh_bounding_box_matrix(bounding_box: (Vector3<f32>, Vector3<f32>)) -> Matrix4<f32> {
  let (min, max) = bounding_box;
  let size = (max - min) / 2.0;
  let center = (max + min) / 2.0;

  //println!("min: {:?}, max: {:?}, center: {:?}, size: {:?}", min, max, center, size);

  // let transform = Transform::from_position_scale(center, size);
  // transform

  Matrix4::from_translation(-center) * Matrix4::from_nonuniform_scale(size.x, size.y, size.z)
}

pub fn ale_mesh_tri_len(mesh: &Mesh) -> usize {
  mesh.vertices.total_row_len() / 3
}

pub fn ale_mesh_tri_get(mesh: &Mesh, i: usize) -> Option<Tri> {
  let column_len = mesh.vertices.total_column_len();
  if i > ale_mesh_tri_len(mesh) {
    return None;
  }

  let vertex_offset = mesh.vertex_offset.expect("This mesh doesn't have positions");
  let uv_offset = mesh.uv_offset.expect("This mesh doesn't have UVs");
  let normal_offset = mesh.normal_offset.expect("This mesh doesn't have normal");

  let tri = match &mesh.indices {
    None => {
      let s0 = i * 3;
      let s1 = (i * 3) + column_len;
      let s2 = (i * 3) + (2 * column_len);
      let vert = &mesh.vertices;

      let (p0, p1, p2) = (s0 + vertex_offset, s1 + vertex_offset, s2 + vertex_offset);
      let position = [
        Vector3::new(vert[p0], vert[p0 + 1], vert[p0 + 2]),
        Vector3::new(vert[p1], vert[p1 + 1], vert[p1 + 1]),
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
        Vector3::new(vert[n1], vert[n1 + 1], vert[n1 + 1]),
        Vector3::new(vert[n2], vert[n2 + 1], vert[n2 + 2]),
      ];

      Tri { position, normal, uv }
    }
    Some(ind) => {
      let (s0, s1, s2) = (
        ind[i * 3] as usize * column_len,
        ind[i * 3 + 1] as usize * column_len,
        ind[i * 3 + 2] as usize * column_len,
      );
      let vert = &mesh.vertices;

      let (p0, p1, p2) = (s0 + vertex_offset, s1 + vertex_offset, s2 + vertex_offset);
      let position = [
        Vector3::new(vert[p0], vert[p0 + 1], vert[p0 + 2]),
        Vector3::new(vert[p1], vert[p1 + 1], vert[p1 + 1]),
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
        Vector3::new(vert[n1], vert[n1 + 1], vert[n1 + 1]),
        Vector3::new(vert[n2], vert[n2 + 1], vert[n2 + 2]),
      ];

      Tri { position, normal, uv }
    }
  };

  Some(tri)
}
