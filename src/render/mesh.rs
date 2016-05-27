extern crate num;

use ::io::file::Stream;
use ::materials::material::Material;
use ::materials::solid_materials::{
    SolidMaterial,
    BasicSolidMaterial,
};
use ::materials::textured_materials::{
    TexturedMaterial,
    BasicTexturedMaterial,
};
use ::math::kdtree::KDNode;
use ::math::ray::Ray3;
use ::math::triangle::{
    Triangle,
    SolidTriangle,
    TexturedTriangle,
};
use ::math::vector::{
    VectorElement,
    MathVector,
    Vec3,
};
use ::render::vertex::{
    Vertex,
    PosNrm,
    PosNrmUV,
};

pub trait MeshTrait<E> where E: VectorElement {
    fn read(&mut self, s: &mut Stream);
    // fn hit(&self, r: &Ray3<E>) -> Option<(E, E, E, usize)>;
}

// pub struct GenericMesh<E, V, M, T> where
//         E: VectorElement,
//         V: Vertex<E>,
//         M: Material<E>,
//         T: Triangle<E> {
//     vertices:  Vec<V>,
//     triangles: Vec<T>,
//     material:  M,
//     kdtree:    KDNode<E>,
// }
//
// impl<E, V, M, T> GenericMesh<E, V, M, T> where
//         E: VectorElement,
//         V: Vertex<E>,
//         M: Material<E>,
//         T: Triangle<E> {
//     pub fn new() -> GenericMesh<E, V, M, T> {
//         GenericMesh {
//             vertices:  Vec::new(),
//             triangles: Vec::new(),
//             material:  M::new(),
//             kdtree:    KDNode::new(),
//         }
//     }
// }
//
// impl<E, V, M, T> MeshTrait<E> for GenericMesh<E, V, M, T> where
//         E: VectorElement,
//         V: Vertex<E>,
//         M: Material<E>,
//         T: Triangle<E> {
//     fn read(&mut self, s: &mut Stream) {
//         let vertex_count = (s.read(&0u32) as usize) / V::elements_count();
//         self.vertices.resize(vertex_count, V::new());
//         for i in 0..vertex_count {
//             self.vertices[i].read(s);
//         }
//         let triangles_count = (s.read(&0u32) / 3) as usize;
//         let mut indices = Vec::new();
//         indices.resize(triangles_count, [0usize; 3]);
//         for i in 0..triangles_count {
//             indices[i] = [
//                 s.read(&0u32) as usize,
//                 s.read(&0u32) as usize,
//                 s.read(&0u32) as usize,
//             ];
//         }
//         self.triangles = Vec::new();
//         self.triangles.resize(triangles_count, T::new(&indices[0], &self.vertices));
//         let mut triangle_indices = Vec::new();
//         triangle_indices.resize(triangles_count, 0usize);
//         for i in 1..triangles_count {
//             self.triangles[i] = T::new(&indices[i], &self.vertices);
//             triangle_indices[i] = i;
//         }
//         self.kdtree = KDNode::build(&triangle_indices, &self.triangles, &self.vertices).unwrap();
//     }
// }

pub struct SolidMesh<'a, E> where E: VectorElement + 'a {
    vertices:  Vec<PosNrm<E>>,
    triangles: Vec<SolidTriangle<E>>,
    material:  Box<SolidMaterial<E> + 'a>,
    kdtree:    Box<KDNode<E>>,
}

impl<'a, E> SolidMesh<'a, E> where E: VectorElement + 'a {
    pub fn new() -> SolidMesh<'a, E> {
        SolidMesh {
            vertices:  Vec::new(),
            triangles: Vec::new(),
            material:  Box::new(BasicSolidMaterial::new()),
            kdtree:    Box::new(KDNode::new()),
        }
    }
}

impl<'a, E> MeshTrait<E> for SolidMesh<'a, E> where E: VectorElement + 'a {
    fn read(&mut self, s: &mut Stream) {
        let vertex_count: usize = (s.read(&0u32) as usize) / 6;
        self.vertices.resize(vertex_count, PosNrm::new());
        for i in 0..vertex_count {
            self.vertices[i].read(s);
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
        self.triangles = Vec::new();
        self.triangles.resize(triangles_count, SolidTriangle::new(&indices[0], &self.vertices));
        let mut triangle_indices = Vec::new();
        triangle_indices.resize(triangles_count, 0usize);
        for i in 1..triangles_count {
            self.triangles[i] = SolidTriangle::new(&indices[i], &self.vertices);
            triangle_indices[i] = i;
        }
        self.kdtree = KDNode::build(&triangle_indices, &self.vertices, &self.triangles).unwrap();
    }
}

pub struct TexturedMesh<'a, E> where E: VectorElement + 'a {
    vertices:  Vec<PosNrmUV<E>>,
    triangles: Vec<TexturedTriangle<E>>,
    material:  Box<TexturedMaterial<E> + 'a>,
    kdtree:    Box<KDNode<E>>,
}

impl<'a, E> TexturedMesh<'a, E> where E: VectorElement + 'a {
    pub fn new() -> TexturedMesh<'a, E> {
        TexturedMesh {
            vertices:  Vec::new(),
            triangles: Vec::new(),
            material:  Box::new(BasicTexturedMaterial::new()),
            kdtree:    Box::new(KDNode::new()),
        }
    }
}

impl<'a, E> MeshTrait<E> for TexturedMesh<'a, E> where E: VectorElement + 'a {
    fn read(&mut self, s: &mut Stream) {
        let vertex_count = (s.read(&0u32) as usize) / 8;
        self.vertices.resize(vertex_count, PosNrmUV::new());
        for i in 0..vertex_count {
            self.vertices[i].read(s);
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
        self.triangles = Vec::new();
        self.triangles.resize(triangles_count, TexturedTriangle::new(&indices[0], &self.vertices));
        let mut triangle_indices = Vec::new();
        triangle_indices.resize(triangles_count, 0usize);
        for i in 1..triangles_count {
            self.triangles[i] = TexturedTriangle::new(&indices[i], &self.vertices);
            triangle_indices[i] = i;
        }
        self.kdtree = KDNode::build(&triangle_indices, &self.vertices, &self.triangles).unwrap();
    }
}


//
// pub struct BasicMesh {
//     vs: Vec<PosNrm<f32>>,
//     m: Box<SolidMaterial>,
//     kdt: Box<KDNode<f32, SolidTriangle<f32>>>,
// }
//
// impl BasicMesh {
//     pub fn new() -> BasicMesh {
//         BasicMesh {
//             vs: Vec::new(),
//             m: Box::new(BasicSolidMaterial::new(&0f32)),
//             kdt: Box::new(KDNode::new()),
//         }
//     }
// }
//
// impl Mesh for BasicMesh {
//     fn read(&mut self, s: &mut Stream) {
//         let vertex_count = (s.read(&0u32) / 6u32) as usize;
//         self.vs.resize(vertex_count, PosNrm::new(0f32));
//         for i in 0..vertex_count {
//             self.vs[i] = PosNrm::read(s);
//         }
//         let triangles_count = (s.read(&0u32) / 3) as usize;
//         let mut indices = Vec::new();
//         indices.resize(triangles_count, [0usize; 3]);
//         for i in 0..triangles_count {
//             indices[i] = [
//                 s.read(&0u32) as usize,
//                 s.read(&0u32) as usize,
//                 s.read(&0u32) as usize,
//             ];
//         }
//         let mut triangles = Vec::new();
//         triangles.resize(triangles_count, SolidTriangle::new(&indices[0], &self.vs));
//         for i in 0..triangles_count {
//             triangles[i] = SolidTriangle::new(&indices[i], &self.vs);
//         }
//         self.kdt = KDNode::build(&triangles, &0, &self.vs).unwrap();
//     }
// }
//
// pub struct TexturedMesh {
//     vs: Vec<PosNrmUV<f32>>,
//     m: Box<TexturedMaterial>,
//     kdt: Box<KDNode<f32, TexturedTriangle<f32>>>,
// }
//
// impl TexturedMesh {
//     pub fn new() -> TexturedMesh {
//         TexturedMesh {
//             vs: Vec::new(),
//             m: Box::new(BasicTexturedMaterial::new(&0f32)),
//             kdt: Box::new(KDNode::new()),
//         }
//     }
// }
//
// impl Mesh for TexturedMesh {
//     fn read(&mut self, s: &mut Stream) {
//         let vertex_count = (s.read(&0u32) / 6u32) as usize;
//         self.vs.resize(vertex_count, PosNrmUV::new(0f32));
//         for i in 0..vertex_count {
//             self.vs[i].read(s);
//         }
//         let triangles_count = (s.read(&0u32) / 3) as usize;
//         let mut indices = Vec::new();
//         indices.resize(triangles_count, [0usize; 3]);
//         for i in 0..triangles_count {
//             indices[i] = [
//                 s.read(&0u32) as usize,
//                 s.read(&0u32) as usize,
//                 s.read(&0u32) as usize,
//             ];
//         }
//         let mut triangles = Vec::new();
//         triangles.resize(triangles_count, TexturedTriangle::new(&indices[0], &self.vs));
//         for i in 0..triangles_count {
//             triangles[i] = TexturedTriangle::new(&indices[i], &self.vs);
//         }
//         self.kdt = KDNode::build(&triangles, &0, &self.vs).unwrap();
//     }
// }
