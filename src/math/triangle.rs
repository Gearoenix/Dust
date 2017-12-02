use super::vector::{Vec2, Vec3};
// use ::math::aabbox::AABBox3;
use super::ray::Ray3;
use super::super::render::vertex::Vertex;

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    edg: [Vec3; 2],
    ind: [usize; 3],
    tedg: [Vec2; 2],
}

impl Triangle {
    pub fn new(inds: &[usize; 3], vertices: &Vec<Vertex>) -> Triangle {
        Triangle {
            edg: [
                &vertices[inds[1]].ps - &vertices[inds[0]].ps,
                &vertices[inds[2]].ps - &vertices[inds[0]].ps,
            ],
            ind: [inds[0], inds[1], inds[2]],
            tedg: [
                vertices[inds[1]].uv - vertices[inds[0]].uv,
                vertices[inds[2]].uv - vertices[inds[0]].uv,
            ],
        }
    }

    pub fn get_texture_coord(&self, u: f64, v: f64) -> Vec2 {
        self.tedg[0] * u + self.tedg[1] * v
    }

    // pub fn get_aabb(&self, vertices: &Vec<Vertex>) -> AABBox3 {
    //     AABBox3 {
    //         blf: Vec3 {
    //             x: vertices[self.ind[0]].ps.x.min(vertices[self.ind[1]].ps.x.min(vertices[self.ind[2]].ps.x)),
    //             y: vertices[self.ind[0]].ps.y.min(vertices[self.ind[1]].ps.y.min(vertices[self.ind[2]].ps.y)),
    //             z: vertices[self.ind[0]].ps.z.min(vertices[self.ind[1]].ps.z.min(vertices[self.ind[2]].ps.z)),
    //         },
    //         trr: Vec3 {
    //             x: vertices[self.ind[0]].ps.x.max(vertices[self.ind[1]].ps.x.max(vertices[self.ind[2]].ps.x)),
    //             y: vertices[self.ind[0]].ps.y.max(vertices[self.ind[1]].ps.y.max(vertices[self.ind[2]].ps.y)),
    //             z: vertices[self.ind[0]].ps.z.max(vertices[self.ind[1]].ps.z.max(vertices[self.ind[2]].ps.z)),
    //         },
    //     }
    // }

    pub fn get_midpoint(&self, vertices: &Vec<Vertex>) -> Vec3 {
        &(&(&vertices[self.ind[0]].ps + &vertices[self.ind[1]].ps) + &vertices[self.ind[2]].ps)
            / 3f64
    }

    pub fn intersect(
        &self,
        r: &Ray3,
        tmin: f64,
        vertices: &Vec<Vertex>,
    ) -> Option<(f64, f64, f64)> {
        let pvec = r.d.cross(&self.edg[1]);
        let det = self.edg[0].dot(&pvec);
        if det < 0.000001 && det > -0.000001 {
            return None;
        }
        println!("Reach1");
        let inv_det = 1f64 / det;
        let tvec = &r.o - &vertices[self.ind[0]].ps;
        let u = tvec.dot(&pvec) * inv_det;
        if u < 0f64 || u > 1f64 {
            return None;
        }
        println!("Reach2");
        let qvec = tvec.cross(&self.edg[0]);
        let v = r.d.dot(&qvec) * inv_det;
        if v < 0f64 || u + v > 1f64 {
            return None;
        }
        println!("Reach3");
        let t = self.edg[1].dot(&qvec) * inv_det; // Set distance along ray to intersection
        if t < tmin {
            if t > 1e-9 {
                return Some((t, u, v));
            }
        }
        println!("Reach4");
        None
    }

    pub fn barycentric(&self, p: &Vec3, vertices: &Vec<Vertex>) -> Vec3 {
        let v2_ = p - &vertices[self.ind[0]].ps;
        let d00 = self.edg[0].dot(&self.edg[0]);
        let d01 = self.edg[0].dot(&self.edg[1]);
        let d11 = self.edg[1].dot(&self.edg[1]);
        let d20 = v2_.dot(&self.edg[0]);
        let d21 = v2_.dot(&self.edg[1]);
        let d = d00 * d11 - d01 * d01;
        let v = (d11 * d20 - d01 * d21) / d;
        let w = (d00 * d21 - d01 * d20) / d;
        let u = 1f64 - v - w;
        return Vec3 { x: u, y: v, z: w };
    }
}
