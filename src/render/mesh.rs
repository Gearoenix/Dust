use ::io::file::Stream;
use ::math::kdtree::KDNode;
use ::math::ray::Ray3;
use ::math::triangle::Triangle;
use ::math::vector::Vec3;
use ::render::material::Material;
use ::render::vertex::Vertex;

pub struct Mesh {
    vertices:  Vec<Vertex>,
    triangles: Vec<Triangle>,
    material:  Material,
    kdtree:    Box<KDNode>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices:  Vec::new(),
            triangles: Vec::new(),
            material:  Material::new(),
            kdtree:    Box::new(KDNode::new()),
        }
    }

    pub fn read(&mut self, s: &mut Stream, has_normal: bool, has_uv: bool) {
        let vertex_count = (s.read(&0u32) as usize) / 8;  // TODO
        self.vertices.resize(vertex_count, Vertex::new());
        for i in 0..vertex_count {
            self.vertices[i].read(s, has_normal, has_uv);
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
        self.triangles.resize(triangles_count, Triangle::new(&indices[0], &self.vertices));
        let mut triangle_indices = Vec::new();
        triangle_indices.resize(triangles_count, 0usize);
        for i in 1..triangles_count {
            self.triangles[i] = Triangle::new(&indices[i], &self.vertices);
            triangle_indices[i] = i;
        }
        self.kdtree = KDNode::build(&triangle_indices, &self.vertices, &self.triangles).unwrap();
    }

    pub fn hit(&self, r: &Ray3, tmin: f64) -> Option<(f64, f64, f64, usize)> {
        return KDNode::hit(&*self.kdtree, r, tmin, &self.vertices, &self.triangles);
    }
}
