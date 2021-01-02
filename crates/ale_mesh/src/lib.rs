use crate::buffer::{Buffer, BufferBuilder, SeparateBufferBuilder};
use ale_autoid::*;
use ale_math::transform::Transform;
use ale_math::{Array, Matrix4, Vector3};

pub mod buffer;
pub mod sdf;
pub mod iter;

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
}

struct_id!(MeshId);
struct_id_impl!(MeshId, Mesh, id);

pub fn ale_mesh_new(
  vertices: Buffer<f32>,
  indices: Option<Buffer<i32>>,
  bounding_box: (Vector3<f32>, Vector3<f32>),
) -> Mesh {
  Mesh {
    id: MeshId::new(),
    vertices,
    indices,
    bounding_box,
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
