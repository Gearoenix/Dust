use std;
use std::collections::HashMap;

use ::math::vector::{
    Vec3,
    Vec2,
    MathVector,
    VectorElement,
};
use ::math::aabbox::{
    AABBox3,
    ExpandableToPoint3,
};
use ::math::matrix::{
    Mat,
    Mat4x4,
};
use ::math::ray::Ray3;
use ::math::triangle::Triangle;
use ::math::kdtree::KDNode;
use ::render::mesh::{
    Mesh,
    TexturedMesh,
    BasicMesh,
};
use ::materials::material::Material;
use ::io::file::Stream;

// pub trait Geometry {
//     fn read<E>(&mut self, s: &mut Stream) -> AABBox3<E> where E: VectorElement;
// }

pub struct BasicGeometry {
    pub ms: Vec<Box<Mesh>>,
    pub mm: HashMap<String, usize>,
    pub position: Vec3<f32>,
    pub transform: Mat4x4<f32>,
    // TODO add kdtree for your meshes
}

impl BasicGeometry {
    pub fn new() -> BasicGeometry {
        BasicGeometry {
            ms: Vec::new(),
            mm: HashMap::new(),
            position: Vec3::new(0f32),
            transform: Mat4x4::new(),
        }
    }

    pub fn read(&mut self, s: &mut Stream) -> AABBox3<f32> {
        let meshes_count = s.read(&0u8);
        for _ in 0..meshes_count {
            let mesh_index = self.ms.len();
            let mesh_name = s.read_string();
            let mut mesh: Box<Mesh>;
            if s.read_bool() {
                let texture_index = s.read(&0u16);
                mesh = Box::new(TexturedMesh::new());
            } else {
                mesh = Box::new(BasicMesh::new());
            }
            mesh.read(s);
            self.ms.push(mesh);
            self.mm.insert(mesh_name, mesh_index);
        }
        self.transform.read(s);
        self.position.read(s);
        s.read(&0f32);
        {
            let vertex_count = (s.read(&0u32) / 3u32) as usize;
            let mut vs = Vec::new();
            let mut aabb = AABBox3::new();
            vs.resize(vertex_count, Vec3::new(0f32));
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
    pub fn hit(&self, r: &Ray3<f64>) -> Option<(f64, Vec3<f64>, Vec3<f64>, Box<Material>)> {
        let mut hit = false;
        let mut distance = std::f64::MAX;
        let mut result: Option<(f64, Vec3<f64>, Vec3<f64>, Box<Material>)> = None;
        // TODO i must search through kdtree for mesh finding.
        for m in self.ms {
            let hited = m.hit(r);
            if hited.is_some() {
                let (d, _, _, _) = hited;
                if d < distance {
                    result = hited;
                }
            }
        }
        result
    }
}

// impl<E, T> Geometry<E, T> where E: VectorElement, T: Triangle<E> {
//     // distance form start point, normal of intersecting triangle, material
//     fn get_intersection(&self, r: &Ray3<E>) -> Option<(E, Vec3<E>, Material)> {
//         // TODO
//         return None;
//     }
// }
