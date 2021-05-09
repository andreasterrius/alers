use ale_buffer::{Buffer, SeparateBufferBuilder};
use ale_math::transform::AleTransform;
use ale_math::Matrix4;
use ale_math::{Decomposed, One, Quaternion};
use ale_mesh::{ale_mesh_new, Mesh};
use gltf::mesh::util::{ReadIndices, ReadTexCoords};
use gltf::mesh::Reader;
use gltf::Gltf;
use std::collections::HashMap;

pub fn ale_gltf_load(path: &str) -> Vec<(AleTransform, Mesh)> {
  let (gltf, buffers, _) = gltf::import(path).unwrap();

  let mut nodes = HashMap::new();
  for node in gltf.nodes() {
    //println!("Node #{} {:?} {:?}", node.index(), node.name(), node);

    match node.transform() {
      gltf::scene::Transform::Matrix { matrix } => {
        // do nothing for now
      }
      gltf::scene::Transform::Decomposed {
        translation,
        rotation,
        scale,
      } => {
        let transform = AleTransform::from_all(
          translation.into(),
          Quaternion::new(rotation[3], rotation[2], rotation[1], rotation[0]),
          scale.into(),
        );
        nodes.insert(node.index(), transform);
      }
    }
  }

  let mut objects = vec![];
  for mesh in gltf.meshes() {
    //println!("Mesh #{}", mesh.index());
    for primitive in mesh.primitives() {
      //println!("- Primitive #{}", primitive.index());
      let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

      let (positions, bb_min, bb_max) = intern_get_positions(&reader);

      //println!("pos {:?}", positions);

      let normals = intern_get_normals(&reader);
      let tex_coords = intern_get_tex_coords(&reader);
      let indices = intern_get_indices(&reader);

      let vbuffer = intern_construct_vertices_buffer(positions, normals, tex_coords);
      let ibuffer = intern_construct_indices_buffer(indices);

      let ale_mesh = ale_mesh_new(vbuffer, Some(ibuffer), (bb_min.into(), bb_max.into()));
      objects.push((nodes.remove(&mesh.index()).unwrap(), ale_mesh));
    }
  }

  return objects;
}

fn intern_get_positions<'a, 's, F>(reader: &Reader<'a, 's, F>) -> (Vec<f32>, (f32, f32, f32), (f32, f32, f32))
where
  F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
  let mut positions = vec![];
  let mut min = (f32::MAX, f32::MAX, f32::MAX);
  let mut max = (f32::MIN, f32::MIN, f32::MIN);
  // Also returns the bounding box for this mesh
  if let Some(read_positions) = reader.read_positions() {
    for rp in read_positions {
      positions.push(rp[0]);
      positions.push(rp[1]);
      positions.push(rp[2]);

      min.0 = f32::min(min.0, rp[0]);
      min.1 = f32::min(min.1, rp[1]);
      min.2 = f32::min(min.2, rp[2]);

      max.0 = f32::max(max.0, rp[0]);
      max.1 = f32::max(max.1, rp[1]);
      max.2 = f32::max(max.2, rp[2]);
    }
  }

  if positions.len() == 0 {
    min = (0.0, 0.0, 0.0);
    max = (0.0, 0.0, 0.0);
  }

  return (positions, min, max);
}

fn intern_get_normals<'a, 's, F>(reader: &Reader<'a, 's, F>) -> Vec<f32>
where
  F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
  let mut normals = vec![];
  if let Some(read_normals) = reader.read_normals() {
    for rp in read_normals {
      normals.push(rp[0]);
      normals.push(rp[1]);
      normals.push(rp[2]);
    }
  }
  return normals;
}

fn intern_get_tex_coords<'a, 's, F>(reader: &Reader<'a, 's, F>) -> Vec<f32>
where
  F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
  let mut tex_coords: Vec<f32> = vec![];
  if let Some(read_tex_coords) = reader.read_tex_coords(0) {
    match read_tex_coords {
      ReadTexCoords::U8(iter) => {
        for i in iter {
          tex_coords.push(i[0] as f32);
          tex_coords.push(i[1] as f32);
        }
      }
      ReadTexCoords::U16(iter) => {
        for i in iter {
          tex_coords.push(i[0] as f32);
          tex_coords.push(i[1] as f32);
        }
      }
      ReadTexCoords::F32(iter) => {
        for i in iter {
          tex_coords.push(i[0]);
          tex_coords.push(i[1]);
        }
      }
    }
  }
  return tex_coords;
}

fn intern_get_indices<'a, 's, F>(reader: &Reader<'a, 's, F>) -> Vec<i32>
where
  F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
  let mut indices: Vec<i32> = vec![];
  if let Some(read_indices) = reader.read_indices() {
    match read_indices {
      ReadIndices::U8(iter) => {
        for i in iter {
          indices.push(i as i32)
        }
      }
      ReadIndices::U16(iter) => {
        for i in iter {
          indices.push(i as i32)
        }
      }
      ReadIndices::U32(iter) => {
        for i in iter {
          indices.push(i as i32)
        }
      }
    }
  }

  return indices;
}

fn intern_construct_vertices_buffer(positions: Vec<f32>, normals: Vec<f32>, tex_coords: Vec<f32>) -> Buffer<f32> {
  let vbuffer = SeparateBufferBuilder::new()
    .info("position", 3, positions)
    .info("normal", 3, normals)
    .info("uv", 2, tex_coords)
    .build()
    .unwrap();

  return vbuffer;
}

fn intern_construct_indices_buffer(indices: Vec<i32>) -> Buffer<i32> {
  let ibuffer = SeparateBufferBuilder::new().info("index", 3, indices).build().unwrap();

  return ibuffer;
}

#[test]
pub fn gltf_to_buffers_should_properly_parse() {
  ale_gltf_load("test_resources/standard.gltf");
}

#[test]
pub fn gltf_to_buffers_should_properly_parse_cube() {
  ale_gltf_load("test_resources/cube.glb");
}
