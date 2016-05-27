extern crate num;

use ::math::vector::{
    Vec3,
    Vec2,
    MathVector,
    VectorElement,
};
use ::math::num::{
    min,
    max,
};
use ::math::aabbox::AABBox3;
use ::math::ray::Ray3;
use ::render::vertex::{
    HasPosition,
    HasNormal,
    HasUV,
};

pub trait Triangle<E>: Sized + Clone + Copy where E: VectorElement {
    fn get_vertex_index(&self, index: usize) -> usize;
    fn get_edge(&self, index: usize) -> &Vec3<E>;
    fn get_aabb<V>(&self, vertices: &Vec<V>) -> AABBox3<E> where V: HasPosition<E>;
    fn get_midpoint<V>(&self, vertices: &Vec<V>) -> Vec3<E> where V: HasPosition<E>;
    fn intersect<V>(&self, r: &Ray3<E>, tmin: &E, vertices: &Vec<V>) -> Option<(E, E, E)> where V: HasPosition<E>;
    fn barycentric<V>(&self, p: &Vec3<E>, vertices: &Vec<V>) -> Vec3<E> where V: HasPosition<E>;
}


fn static_get_aabb<E>(v1: &Vec3<E>, v2: &Vec3<E>, v3: &Vec3<E>) -> AABBox3<E> where E: VectorElement {
    AABBox3 {
        blf: Vec3 {
            x: min(v3.x, min(v1.x, v2.x)),
            y: min(v3.y, min(v1.y, v2.y)),
            z: min(v3.z, min(v1.z, v2.z)),
        },
        trr: Vec3 {
            x: max(v3.x, max(v1.x, v2.x)),
            y: max(v3.y, max(v1.y, v2.y)),
            z: max(v3.z, max(v1.z, v2.z)),
        },
    }
}

fn static_get_midpoint<E>(v1: &Vec3<E>, v2: &Vec3<E>, v3: &Vec3<E>) -> Vec3<E> where E: VectorElement {
    ((*v1) + (*v2) + (*v3)) / num::cast::<u8, E>(3).unwrap()
}

/// return: It will t, u, v,
fn static_intersect<E>(v1: &Vec3<E>, v2: &Vec3<E>, v3: &Vec3<E>, e0: &Vec3<E>, e1: &Vec3<E>, r: &Ray3<E>, tmin: &E) -> Option<(E, E, E)> where E: VectorElement {
    let pvec = r.d.cross(e1);
    let det = e0.dot(&pvec);
    if det == num::cast(0).unwrap() {
        return None;
    }
    let inv_det: E;
    inv_det = num::cast::<i8, E>(1).unwrap() / det;
    let tvec = r.o - *v1;
    let u = tvec.dot(&pvec) * inv_det;
    if u < num::cast(0).unwrap() || u > num::cast(1).unwrap() {
        return None;
    }
    let qvec = tvec.cross(e0);
    let v = r.d.dot(&qvec) * inv_det;
    if v < num::cast(0).unwrap() || u + v > num::cast(1).unwrap() {
        return None;
    }
    let t = e1.dot(&qvec) * inv_det; // Set distance along ray to intersection
    if t < *tmin {
        if t > num::cast(1e-9).unwrap() {
            return Some((t, u, v));
        }
    }
    None
}

// Returns barycentric coordinates of point p on the triangle
fn static_barycentric<E>(v0: &Vec3<E>, v1: &Vec3<E>, v2: &Vec3<E>, e0: &Vec3<E>, e1: &Vec3<E>, p: &Vec3<E>) -> Vec3<E> where E: VectorElement {
    let v2_ = *p - *v0;
    let d00 = e0.dot(e0);
    let d01 = e0.dot(e1);
    let d11 = e1.dot(e1);
    let d20 = v2_.dot(e0);
    let d21 = v2_.dot(e1);
    let d = d00*d11 - d01*d01;
    let v = (d11*d20 - d01*d21) / d;
    let w = (d00*d21 - d01*d20) / d;
    let u = num::cast::<i8, E>(1).unwrap() - v - w;
    return Vec3 {
        x: u,
        y: v,
        z: w,
    };
}

#[derive(Debug, Clone, Copy)]
pub struct TexturedTriangle<T> where T: VectorElement {
    edg: [Vec3<T>; 2],
    ind: [usize; 3],
    tedg: [Vec2<T>; 2],
}

impl<E> TexturedTriangle<E> where E: VectorElement {
    pub fn new<V>(inds: &[usize; 3], vertices: &Vec<V>) -> TexturedTriangle<E> where V: HasPosition<E> + HasNormal<E> + HasUV<E> {
        TexturedTriangle {
            edg : [
                *vertices[inds[1]].get_pos() - *vertices[inds[0]].get_pos(),
                *vertices[inds[2]].get_pos() - *vertices[inds[0]].get_pos(),
            ],
            ind: [
                inds[0],
                inds[1],
                inds[2],
            ],
            tedg: [
                *vertices[inds[1]].get_uv() - *vertices[inds[0]].get_uv(),
                *vertices[inds[2]].get_uv() - *vertices[inds[0]].get_uv(),
            ]
        }
    }

    pub fn get_texture_coord(&self, u: E, v: E) -> Vec2<E> {
        self.tedg[0] * u + self.tedg[1] * v
    }
}

impl<E> Triangle<E> for TexturedTriangle<E> where E: VectorElement {
    fn get_vertex_index(&self, index: usize) -> usize {
        self.ind[index]
    }

    fn get_edge(&self, index: usize) -> &Vec3<E>{
        &self.edg[index]
    }

    fn get_aabb<V>(&self, vertices: &Vec<V>) -> AABBox3<E> where V: HasPosition<E> {
        static_get_aabb(
            vertices[self.ind[0]].get_pos(),
            vertices[self.ind[1]].get_pos(),
            vertices[self.ind[2]].get_pos(),
        )
    }

    fn get_midpoint<V>(&self, vertices: &Vec<V>) -> Vec3<E> where V: HasPosition<E> {
        static_get_midpoint(vertices[self.ind[0]].get_pos(), vertices[self.ind[1]].get_pos(), vertices[self.ind[2]].get_pos())
    }

    fn intersect<V>(&self, r: &Ray3<E>, tmin: &E, vertices: &Vec<V>) -> Option<(E, E, E)> where V: HasPosition<E> {
        static_intersect(
            vertices[self.ind[0]].get_pos(),
            vertices[self.ind[1]].get_pos(),
            vertices[self.ind[2]].get_pos(),
            &self.edg[0], &self.edg[1], r, tmin,
        )
    }

    fn barycentric<V>(&self, p: &Vec3<E>, vertices: &Vec<V>) -> Vec3<E> where V: HasPosition<E> {
        static_barycentric(
            vertices[self.ind[0]].get_pos(),
            vertices[self.ind[1]].get_pos(),
            vertices[self.ind[2]].get_pos(),
            &self.edg[0], &self.edg[1], p,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SolidTriangle<E> where E: VectorElement {
    edg: [Vec3<E>; 2],
    ind: [usize; 3],
}

impl<E> SolidTriangle<E> where E: VectorElement {
    pub fn new<V>(inds: &[usize; 3], vertices: &Vec<V>) -> SolidTriangle<E> where V: HasPosition<E> + HasNormal<E> {
        SolidTriangle {
            edg : [
                *vertices[inds[1]].get_pos() - *vertices[inds[0]].get_pos(),
                *vertices[inds[2]].get_pos() - *vertices[inds[0]].get_pos(),
            ],
            ind: [
                inds[0],
                inds[1],
                inds[2],
            ],
        }
    }
}

impl<E> Triangle<E> for SolidTriangle<E> where E: VectorElement {
    fn get_vertex_index(&self, index: usize) -> usize {
        self.ind[index]
    }

    fn get_edge(&self, index: usize) -> &Vec3<E> {
        &self.edg[index]
    }

    fn get_aabb<V>(&self, vertices: &Vec<V>) -> AABBox3<E> where V: HasPosition<E> {
        static_get_aabb(vertices[self.ind[0]].get_pos(), vertices[self.ind[1]].get_pos(), vertices[self.ind[2]].get_pos())
    }

    fn get_midpoint<V>(&self, vertices: &Vec<V>) -> Vec3<E> where V: HasPosition<E> {
        static_get_midpoint(vertices[self.ind[0]].get_pos(), vertices[self.ind[1]].get_pos(), vertices[self.ind[2]].get_pos())
    }

    fn intersect<V>(&self, r: &Ray3<E>, tmin: &E, vertices: &Vec<V>) -> Option<(E, E, E)> where V: HasPosition<E> {
        static_intersect(
            vertices[self.ind[0]].get_pos(),
            vertices[self.ind[1]].get_pos(),
            vertices[self.ind[2]].get_pos(),
            &self.edg[0], &self.edg[1], r, tmin,
        )
    }

    fn barycentric<V>(&self, p: &Vec3<E>, vertices: &Vec<V>) -> Vec3<E> where V: HasPosition<E> {
        static_barycentric(
            vertices[self.ind[0]].get_pos(),
            vertices[self.ind[1]].get_pos(),
            vertices[self.ind[2]].get_pos(),
            &self.edg[0], &self.edg[1], p,
        )
    }
}
