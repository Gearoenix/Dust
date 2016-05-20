use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign
};

use ::math::vector::{
    Vec2,
    Vec3,
    MathVector,
    VectorElement,
};
use ::io::file::Stream;

pub trait Mat {
    fn read(&mut self, s: &mut Stream);
}

pub trait Mat4<E>: Mat + Mul<Vec3<E>> + Mul where E: VectorElement, Self: Sized {
}

pub struct Mat4x4<E> {
    pub data: [[E; 4]; 4],
}

impl<E> Mat4x4<E> where E: VectorElement {
    pub fn new() -> Mat4x4<E> {
        Mat4x4 {
            data: [[E::zero(); 4]; 4],
        }
    }
}

impl<E> Mat for Mat4x4<E> where E: VectorElement {
    fn read(&mut self, s: &mut Stream) {
        for i in 0..4 {
            for j in 0..4 {
                self.data[j][i] = s.read(&E::zero());
            }
        }
    }
}

impl<E> Mul<Vec3<E>> for Mat4x4<E> where E: VectorElement {
    type Output = Vec3<E>;
    fn mul(self, o: Vec3<E>) -> Vec3<E> {
        Vec3 {
            x: self.data[0][0] * o.x + self.data[0][1] * o.y + self.data[0][2] * o.z + self.data[0][3],
            y: self.data[1][0] * o.x + self.data[1][1] * o.y + self.data[1][2] * o.z + self.data[1][3],
            z: self.data[2][0] * o.x + self.data[2][1] * o.y + self.data[2][2] * o.z + self.data[2][3],
        }
    }
}

impl<E> Mul<Mat4x4<E>> for Mat4x4<E> where E: VectorElement {
    type Output = Mat4x4<E>;
    fn mul(self, o: Mat4x4<E>) -> Mat4x4<E> {
        let mut m: Mat4x4<E>;
        for i in 0..4 {
            for j in 0..4 {
                m.data[i][j] = E::zero();
                for k in 0..4 {
                    m.data[i][j] += self.data[i][k] * o.data[k][j];
                }
            }
        }
        m
    }
}

impl<E> Mat4<E> for Mat4x4<E> where E: VectorElement {}
