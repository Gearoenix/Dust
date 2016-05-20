use std::collections::HashMap;

use ::math::vector::{
    Vec3,
    Vec2,
    MathVector,
    VectorElement,
};
use ::math::aabbox::AABBox3;
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
    pub positon: Vec3<f32>,
    pub transform: Mat4x4<f32>,
    // TODO add kdtree for your meshes
}

impl BasicGeometry {
    pub fn new() -> BasicGeometry {
        BasicGeometry {
            ms: Vec::new(),
            mm: HashMap::new(),
            position: Vec3::new(0f32);
            transform: Mat4x4::new();
        }
    }

    pub fn read<E>(&mut self, s: &mut Stream) -> AABBox3<E> where E: VectorElement {
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
        
    }
}

// impl<E, T> Geometry<E, T> where E: VectorElement, T: Triangle<E> {
//     // distance form start point, normal of intersecting triangle, material
//     fn get_intersection(&self, r: &Ray3<E>) -> Option<(E, Vec3<E>, Material)> {
//         // TODO
//         return None;
//     }
// }
