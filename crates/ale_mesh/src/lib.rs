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
  mesh.vertices.total_row_len()
}

pub fn ale_mesh_tri_get(mesh: &Mesh, i: usize) -> Option<Tri> {
  if i > mesh.vertices.total_row_len() {
    return None;
  }

  None
  /*
  match &mesh.indices {
    None =>
      let c = self.curr;
      let vert = &self.mesh.vertices;
      if c + 8 >= vert.len() {
        return None;
      }
      let t1 = Vector3::new(vert[c], vert[c + 1], vert[c + 2]);
      let t2 = Vector3::new(vert[c + 3], vert[c + 4], vert[c + 5]);
      let t3 = Vector3::new(vert[c + 6], vert[c + 7], vert[c + 8]);

      self.curr += 9;
      Some((t1, t2, t3))
    }
    Some(ind) => {
      let c = self.curr;
      let vert = &self.mesh.vertices;
      if c + 2 >= ind.len() {
        return None;
      }
      let index = ind[c] as usize;
      let t1 = Vector3::new(vert[index], vert[index + 1], vert[index + 2]);

      let index = ind[c + 1] as usize;
      let t2 = Vector3::new(vert[index], vert[index + 1], vert[index + 2]);

      let index = ind[c + 2] as usize;
      let t3 = Vector3::new(vert[index], vert[index + 1], vert[index + 2]);

      self.curr += 3;
      Some((t1, t2, t3))
    }
  }
   */
}
