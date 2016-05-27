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

pub trait MeshTrait<'a, E> where E: VectorElement + 'a {
    fn read(&mut self, s: &mut Stream);
    fn hit(&self, r: &Ray3<E>, tmin: &E) -> Option<(E, E, E, usize)>;
    fn get_material(&self) -> &Box<Material<E> + 'a>;
}

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

impl<'a, E> MeshTrait<'a, E> for SolidMesh<'a, E> where E: VectorElement + 'a {
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

    fn hit(&self, r: &Ray3<E>, tmin: &E) -> Option<(E, E, E, usize)> {
        return KDNode::hit(&*self.kdtree, r, &tmin, &self.vertices, &self.triangles);
    }

    fn get_material(&self) -> &Box<Material<E> + 'a> {
        &self.material
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

impl<'a, E> MeshTrait<'a, E> for TexturedMesh<'a, E> where E: VectorElement + 'a {
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

    fn hit(&self, r: &Ray3<E>, tmin: &E) -> Option<(E, E, E, usize)> {
        return KDNode::hit(&*self.kdtree, r, &tmin, &self.vertices, &self.triangles);
    }

    fn get_material(&self) -> &Box<Material<E>> {
        &*self.material
    }
}
