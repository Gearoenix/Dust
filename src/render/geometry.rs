use std;
use std::collections::HashMap;

use ::math::vector::{
    Vec3,
    Vec2,
};
use ::math::aabbox::{
    AABBox3,
    ExpandableToPoint3,
};
use ::math::matrix::Mat4x4;
use ::math::ray::Ray3;
use ::math::triangle::Triangle;
use ::math::kdtree::KDNode;
use ::render::mesh::Mesh;
use ::render::vertex::Vertex;
use ::render::material::Material;
use ::io::file::Stream;

// pub trait Geometry {
//     fn read<E>(&mut self, s: &mut Stream) -> AABBox3<E> where E: VectorElement;
// }

pub struct BasicGeometry {
    pub meshes: Vec<Mesh>,
    pub name_mesh_index: HashMap<String, usize>,
    pub position: Vec3,
    pub transform: Mat4x4,
    // TODO add kdtree for your meshes
}

impl BasicGeometry {
    pub fn new() -> BasicGeometry {
        BasicGeometry {
            meshes: Vec::new(),
            name_mesh_index: HashMap::new(),
            position: Vec3::new(),
            transform: Mat4x4::new(),
        }
    }

    pub fn read(&mut self, s: &mut Stream) -> AABBox3 {
        let meshes_count = s.read(&0u8);
        for _ in 0..meshes_count {
            let mesh_index = self.meshes.len();
            let mesh_name = s.read_string();
            self.meshes.push(Mesh::new());
            if s.read_bool() {
                let texture_index = s.read(&0u16);
                self.meshes[mesh_index].read(s, true, true);
            } else {
                self.meshes[mesh_index].read(s, true, false);
            }
            self.name_mesh_index.insert(mesh_name, mesh_index);
        }
        self.transform.read(s);
        self.position.read(s);
        s.read(&0f32);
        {
            let vertex_count = (s.read(&0u32) / 3u32) as usize;
            let mut vs = Vec::new();
            let mut aabb = AABBox3::new();
            vs.resize(vertex_count, Vec3::new());
            for i in 0..vertex_count {
                vs[i].read(s);
                aabb.expand(&vs[i]);
            }
            let triangles_count = (s.read(&0u32) / 3) as usize;
            let mut indices = Vec::new();
            indices.resize(triangles_count, [0usize; 3]);
            for i in 0..triangles_count {
                indices[i] = [
                    s.read(&0u32) as usize,
                    s.read(&0u32) as usize,
                    s.read(&0u32) as usize,
                ];
            }
            aabb
        }
    }

    // distance, position, normal, material
    pub fn hit(&self, r: &Ray3, tmin: f64) -> Option<(f64, f64, f64, usize)> {
        // let mut hit = false;
        let mut distance = tmin;
        let mut result: Option<(f64, f64, f64, usize)> = None;
        // TODO i must search through kdtree for mesh finding.
        for (mesh_index, m) in self.meshes.iter().enumerate() {
            let hited = m.hit(r, distance);
            if hited.is_some() {
                let (d, u, v, _) = hited.unwrap();
                if d < distance {
                    result = Some((d, u, v, mesh_index));
                    distance = d;
                }
            }
        }
        result
    }

    pub fn get_color(&self, r: &Ray3, tmin: f64) -> Option<Vec3> {
        let h = self.hit(r, tmin);
        if h.is_none() {
            return None;
        }
        let (d, u, v, mesh_index) = h.unwrap();
        return None;
    }
}

// impl<E, T> Geometry<E, T> where E: VectorElement, T: Triangle<E> {
//     // distance form start point, normal of intersecting triangle, material
//     fn get_intersection(&self, r: &Ray3<E>) -> Option<(E, Vec3<E>, Material)> {
//         // TODO
//         return None;
//     }
// }
