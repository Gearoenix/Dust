use ::io::file::Stream;
use ::math::kdtree::KDNode;
use ::math::ray::Ray3;
use ::math::triangle::{
    Triangle,
    SolidTriangle,
    TexturedTriangle,
};
use ::math::vector::{
    Vec3,
    VectorElement
};
use ::materials::material::Material;
use ::materials::solid_materials::{
    SolidMaterial,
    BasicSolidMaterial,
};
use ::materials::textured_materials::{
    TexturedMaterial,
    BasicTexturedMaterial,
};
use ::render::vertex::{
    PosNrm,
    PosNrmUV,
};

pub trait Mesh {
    fn read(&mut self, s: &mut Stream);
    fn hit<E, T>(&self, r: &Ray3<E>) -> Option<(E, Vec3<E>, Vec3<E>, Box<Material>, *const T)> where E: VectorElement, T: Triangle<E>;
}


pub struct BasicMesh {
    vs: Vec<PosNrm<f32>>,
    m: Box<SolidMaterial>,
    kdt: Box<KDNode<f32, SolidTriangle<f32>>>,
}

impl BasicMesh {
    pub fn new() -> BasicMesh {
        BasicMesh {
            vs: Vec::new(),
            m: Box::new(BasicSolidMaterial::new(&0f32)),
            kdt: Box::new(KDNode::new()),
        }
    }
}

impl Mesh for BasicMesh {
    fn read(&mut self, s: &mut Stream) {
        let vertex_count = (s.read(&0u32) / 6u32) as usize;
        self.vs.resize(vertex_count, PosNrm::new(0f32));
        for i in 0..vertex_count {
            self.vs[i] = PosNrm::read(s);
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
        let mut triangles = Vec::new();
        triangles.resize(triangles_count, SolidTriangle::new(&indices[0], &self.vs));
        for i in 0..triangles_count {
            triangles[i] = SolidTriangle::new(&indices[i], &self.vs);
        }
        self.kdt = KDNode::build(&triangles, &0, &self.vs).unwrap();
    }

    // distance, position, normal, material
    fn hit<E>(&self, r: &Ray3<E>) -> Option<(E, Vec3<E>, Vec3<E>, Box<Material>)> where E: VectorElement {

    }
}

pub struct TexturedMesh {
    vs: Vec<PosNrmUV<f32>>,
    m: Box<TexturedMaterial>,
    kdt: Box<KDNode<f32, TexturedTriangle<f32>>>,
}

impl TexturedMesh {
    pub fn new() -> TexturedMesh {
        TexturedMesh {
            vs: Vec::new(),
            m: Box::new(BasicTexturedMaterial::new(&0f32)),
            kdt: Box::new(KDNode::new()),
        }
    }
}

impl Mesh for TexturedMesh {
    fn read(&mut self, s: &mut Stream) {
        let vertex_count = (s.read(&0u32) / 6u32) as usize;
        self.vs.resize(vertex_count, PosNrmUV::new(0f32));
        for i in 0..vertex_count {
            self.vs[i].read(s);
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
        let mut triangles = Vec::new();
        triangles.resize(triangles_count, TexturedTriangle::new(&indices[0], &self.vs));
        for i in 0..triangles_count {
            triangles[i] = TexturedTriangle::new(&indices[i], &self.vs);
        }
        self.kdt = KDNode::build(&triangles, &0, &self.vs).unwrap();
    }
}
