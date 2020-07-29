use std::collections::HashMap;

use cgmath::prelude::*;
use cgmath::{Deg, Euler, Quaternion, Vector3};

use crate::data::buffer::{Buffer, SeparateBufferBuilder};
use crate::math::transform::Transform;
use crate::resource::bone::Bone;
use crate::resource::mesh::Mesh;
use crate::resource::skin::Skin;

#[derive(Debug)]
pub enum ConversionError {
  NGonNotSupported,
  IncompleteLastPoly,
  PolgyonVertexIndexNotFound,
}

pub fn to_static_meshes(fbx: fbxcel_dom::v7400::Document) -> Result<Vec<(Transform, Mesh)>, ConversionError> {
  //Get root node
  let root = fbx.scenes().nth(0).unwrap().node().tree().root();
  let objects = root.children_by_name("Objects").nth(0).unwrap();

  // Crawl connectivity nodes
  let mut connectivity = HashMap::new();
  for connection in root.children_by_name("Connections") {
    for p in connection.children_by_name("C") {
      //why C
      let attrs = p.attributes();
      if attrs[0].get_string().unwrap() == "OO" {
        connectivity.insert(attrs[1].get_i64().unwrap(), attrs[2].get_i64().unwrap());
      }
    }
  }

  // Get transforms
  let mut transforms: HashMap<i64, Transform> = HashMap::new();
  for model in objects.children_by_name("Model") {
    let mut location = Vector3::zero();
    let mut rotation = Vector3::zero();
    let mut scale = Vector3::zero();
    let properties70_node = model.children_by_name("Properties70").nth(0).unwrap();
    for p_node in properties70_node.children_by_name("P") {
      //? wtf, why P ?

      let arr = p_node.attributes();
      let identifier = arr[0].get_string().unwrap();
      if identifier == "Lcl Translation" {
        let ff = arr[4].get_f64().unwrap() as f32 / 100.0f32;
        let sf = arr[5].get_f64().unwrap() as f32 / 100.0f32;
        let tf = arr[6].get_f64().unwrap() as f32 / 100.0f32;

        location = Vector3::new(ff, sf, tf);
      } else if identifier == "Lcl Rotation" {
        let ff = arr[4].get_f64().unwrap() as f32;
        let sf = arr[5].get_f64().unwrap() as f32;
        let tf = arr[6].get_f64().unwrap() as f32;

        rotation = Vector3::new(ff, sf, tf);
      } else if identifier == "Lcl Scaling" {
        let ff = arr[4].get_f64().unwrap() as f32 / 100.0f32;
        let sf = arr[5].get_f64().unwrap() as f32 / 100.0f32;
        let tf = arr[6].get_f64().unwrap() as f32 / 100.0f32;

        scale = Vector3::new(ff, sf, tf);
      }
    }

    let transform = Transform::from_all(
      location,
      Quaternion::from(Euler {
        x: Deg(rotation.x),
        y: Deg(rotation.y),
        z: Deg(rotation.z),
      }),
      scale,
    );
    //let name = model.attributes().iter().nth(1).unwrap().get_string().unwrap().split("\u{0}\u{1}").collect::<Vec<&str>>()[0];
    let id = model.attributes()[0].get_i64().unwrap();
    transforms.insert(id, transform);
  }

  //Get skin information
  for _deformers in objects.children_by_name("Deformer") {
    let _root_bone = crawl_deformers(&fbx)?;
  }

  // Get geometries
  let mut meshes: Vec<(Transform, Mesh)> = vec![];
  for object in objects.children_by_name("Geometry") {
    let element_node = object.children_by_name("PolygonVertexIndex").nth(0);
    let (indices, indices_con) = match element_node {
      None => {
        return Err(ConversionError::PolgyonVertexIndexNotFound);
      }
      Some(element_node) => {
        let indices = element_node.attributes().iter().nth(0).unwrap().get_arr_i32().unwrap();
        parse_indices(indices)?
      }
    };

    // Get position of vertices
    let position_arr = object
      .children_by_name("Vertices")
      .nth(0)
      .unwrap()
      .attributes()
      .iter()
      .nth(0)
      .unwrap()
      .get_arr_f64()
      .unwrap();

    // Get uv coords
    let uv_node = object.children_by_name("LayerElementUV").nth(0);
    let uv_arr = match uv_node {
      None => &[0.0f64; 0],
      Some(uv_node) => uv_node
        .children_by_name("UV")
        .nth(0)
        .unwrap()
        .attributes()
        .iter()
        .nth(0)
        .unwrap()
        .get_arr_f64()
        .unwrap(),
    };

    //Get normals
    let normal_node = object.children_by_name("LayerElementNormal").nth(0);
    let normal_arr = match normal_node {
      None => &[0.0f64; 0],
      Some(normal_node) => normal_node
        .children_by_name("Normals")
        .nth(0)
        .unwrap()
        .attributes()
        .iter()
        .nth(0)
        .unwrap()
        .get_arr_f64()
        .unwrap(),
    };

    let vbuffer = construct_buffer_flat(&indices, &indices_con, position_arr, uv_arr, normal_arr)?;
    //let name = object.attributes().iter().nth(1).unwrap().get_string().unwrap().split("\u{0}\u{1}").collect::<Vec<&str>>()[0];
    let id = object.attributes()[0].get_i64().unwrap();
    let model_id = connectivity[&id];
    let transform = transforms.get(&model_id).map(|x| x.clone()).unwrap_or(Transform::new());

    meshes.push((transform, Mesh::new(vbuffer, None)));
  }

  Ok(meshes)
}

pub fn to_skeletal_meshes(_fbx: fbxcel_dom::v7400::Document) -> Result<Vec<(Transform, Mesh, Skin)>, ConversionError> {
  Ok(vec![])
}

// Only receives tris or quads
pub fn parse_indices(indices: &[i32]) -> Result<(Vec<i32>, Vec<usize>), ConversionError> {
  let mut start = 0;
  let mut end = 0;
  let mut arr = vec![];
  let mut con = Vec::new();
  //println!("bb: {:?}", indices);
  for i in 0..indices.len() {
    if indices[i] < 0 {
      end = i;
      //println!("start {}, end {}, indices {}", start, end, i);
      if end - start == 2 {
        // 2 index apart
        let _last_idx = arr.len() as i32;
        // 1 tri
        arr.push(indices[start]);
        arr.push(indices[start + 1]);
        arr.push(!indices[start + 2]); // flip last bit

        con.push(start);
        con.push(start + 1);
        con.push(start + 2);
      } else if end - start == 3 {
        // 3 index apart
        let _last_idx = arr.len() as i32;
        // Quad, convert to 2 tris
        arr.push(indices[start]);
        arr.push(indices[start + 1]);
        arr.push(indices[start + 2]);

        arr.push(indices[start + 2]);
        arr.push(!indices[start + 3]);
        arr.push(indices[start]);

        con.push(start);
        con.push(start + 1);
        con.push(start + 2);

        con.push(start + 2);
        con.push(start + 3);
        con.push(start);
      } else if end - start >= 4 {
        return Err(ConversionError::NGonNotSupported);
      }
      // Reset counter to the next index
      // since we have consumed a set of polys
      start = i + 1;
    } else {
      end = i;
    }
  }

  //  println!("result {:?}", arr);
  //  println!("start {}, end {}", start, end);

  // there's a poly that isn't consumed
  // possibly because it has less than 2 vertices
  if end != indices.len() - 1 {
    return Err(ConversionError::IncompleteLastPoly);
  }

  Ok((arr, con))
}

pub fn construct_buffer_flat(
  indices: &[i32],
  indices_con: &Vec<usize>,
  position_arr: &[f64],
  uv_arr: &[f64],
  normal_arr: &[f64],
) -> Result<Buffer<f32>, ConversionError> {
  let mut position_vec = vec![];
  let mut uv_vec = vec![];
  let mut normal_vec = vec![];

  for i in 0..indices.len() {
    let idx = indices[i];

    // calculate offsets
    let index_3 = (idx * 3) as usize;
    let index_2 = (idx * 2) as usize;

    // Push vertices
    position_vec.push(position_arr[index_3] as f32);
    position_vec.push(position_arr[index_3 + 1] as f32);
    position_vec.push(position_arr[index_3 + 2] as f32);

    //    if normal_arr.len() != 0 {
    //      normal_vec.push(normal_arr[i] as f32);
    //      normal_vec.push(normal_arr[i + 1] as f32);
    //      normal_vec.push(normal_arr[i + 2] as f32);
    //    } else {
    //      normal_vec.push(0.0f32);
    //      normal_vec.push(0.0f32);
    //      normal_vec.push(0.0f32);
    //    }

    if uv_arr.len() != 0 {
      uv_vec.push(uv_arr[index_2] as f32);
      uv_vec.push(uv_arr[index_2 + 1] as f32);
    } else {
      uv_vec.push(0.0f32);
      uv_vec.push(0.0f32);
    }
  }

  for i in 0..indices_con.len() {
    let idx = indices_con[i] * 3;
    normal_vec.push(normal_arr[idx] as f32);
    normal_vec.push(normal_arr[idx + 1] as f32);
    normal_vec.push(normal_arr[idx + 2] as f32);
  }

  let vbuffer = SeparateBufferBuilder::new()
    .info("position", 3, position_vec)
    .info("normal", 3, normal_vec)
    .info("uv", 2, uv_vec)
    .build()
    .unwrap();

  //println!("{:#?}", vbuffer);

  Ok(vbuffer)
}

pub fn crawl_deformers(_fbx: &fbxcel_dom::v7400::Document) -> Result<Vec<Bone>, ConversionError> {
  Ok(vec![])
}

#[test]
pub fn fbx_to_buffers_should_properly_parse() {
  crate::log::init_term();

  let fbx = crate::resource::fbx::load("resources/test/geom/basic_blender.fbx").expect("Fail to load rigged fbx");

  let meshes = to_static_meshes(fbx);
  println!("{:#?}", meshes);
}
