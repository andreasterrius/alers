use crate::buffer::Buffer;
use crate::Mesh;
use ale_math::Vector3;

pub struct MeshTriangleIterator<'a> {
  mesh: &'a Mesh,
  curr: usize,
}

impl<'a> Iterator for MeshTriangleIterator<'a> {
  type Item = (Vector3<f32>, Vector3<f32>, Vector3<f32>);

  fn next(&mut self) -> Option<Self::Item> {
    match &self.mesh.indices {
      None => {
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
  }
}

pub fn ale_mesh_triangle_iter_new(mesh: &Mesh) -> MeshTriangleIterator {
  MeshTriangleIterator { mesh, curr: 0 }
}
