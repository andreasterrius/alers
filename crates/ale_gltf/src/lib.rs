use ale_math::transform::Transform;
use ale_mesh::Mesh;
use gltf::mesh::util::ReadTexCoords;
use gltf::mesh::Reader;
use gltf::Gltf;

pub fn ale_gltf_load(path: &str) -> Vec<(Transform, Mesh)> {
  let (gltf, buffers, _) = gltf::import(path).unwrap();
  for mesh in gltf.meshes() {
    println!("Mesh #{}", mesh.index());
    for primitive in mesh.primitives() {
      println!("- Primitive #{}", primitive.index());
      let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

      intern_get_positions(&reader);
      intern_get_normals(&reader);
      intern_get_tex_coords(&reader);
    }
  }

  return vec![];
}

fn intern_get_positions<'a, 's, F>(reader: &Reader<'a, 's, F>) -> Vec<f32>
where
  F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>,
{
  let mut positions = vec![];
  if let Some(read_positions) = reader.read_positions() {
    for rp in read_positions {
      positions.push(rp[0]);
      positions.push(rp[1]);
      positions.push(rp[2]);
    }
  }
  return positions;
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

#[test]
pub fn gltf_to_buffers_should_properly_parse() {
  ale_gltf_load("test_resources/standard.gltf");
}

#[test]
pub fn gltf_to_buffers_should_properly_parse_cube() {
  ale_gltf_load("test_resources/cube.glb");
}
