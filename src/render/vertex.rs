extern crate num;

use ::math::vector::{
    Vec3,
    Vec2,
    MathVector,
    VectorElement,
};

use ::io::file::Stream;

pub trait Vertex {
}

pub trait HasPosition<T>: Vertex where T: VectorElement {
    fn get_pos(&self) -> &Vec3<T>;
}

pub trait HasNormal<T>: Vertex where T: VectorElement {
    fn get_nrm(&self) -> &Vec3<T>;
}

pub trait HasUV<T>: Vertex where T: VectorElement {
    fn get_uv(&self) -> &Vec2<T>;
}

macro_rules! at3 {
    ($func:ident, $tra:ident, $vatt:ident, $stru:ident) => (
        impl<T> $tra<T> for $stru<T> where T: VectorElement {
            fn $func(&self) -> &Vec3<T> {
                &self.$vatt
            }
        }
    )
}

macro_rules! at2 {
    ($func:ident, $tra:ident, $vatt:ident, $stru:ident) => (
        impl<T> $tra<T> for $stru<T> where T: VectorElement {
            fn $func(&self) -> &Vec2<T> {
                &self.$vatt
            }
        }
    )
}

#[derive(Debug, Clone, Copy)]
pub struct PosNrmUV<T> where T: VectorElement {
    pub pos: Vec3<T>,
    pub nrm: Vec3<T>,
    pub uv:  Vec2<T>,
}

impl<T> PosNrmUV<T> where T: VectorElement {
    pub fn new(e: T) -> PosNrmUV<T> {
        PosNrmUV {
            pos: Vec3::new(e),
            nrm: Vec3::new(e),
            uv:  Vec2::new(e),
        }
    }

    pub fn read(&mut self, s: &mut Stream) {
        self.pos.read(s);
        self.nrm.read(s);
        self.uv.read(s);
    }
}

impl<E> Vertex for PosNrmUV<E> where E: VectorElement {
}

at3!(get_pos, HasPosition, pos, PosNrmUV);
at3!(get_nrm, HasNormal, nrm, PosNrmUV);
at2!(get_uv, HasUV, uv, PosNrmUV);

#[derive(Debug, Clone, Copy)]
pub struct PosNrm<T> where T: VectorElement {
    pub pos: Vec3<T>,
    pub nrm: Vec3<T>,
}

impl<T> PosNrm<T> where T: VectorElement {
    pub fn new(e: T) -> PosNrm<T> {
        PosNrm {
            pos: Vec3::new(e),
            nrm: Vec3::new(e),
        }
    }

    pub fn read(s: &mut Stream) -> PosNrm<T> {
        let mut pos = Vec3::new(num::cast::<i8, T>(0).unwrap());
        pos.read(s);
        let mut nrm = Vec3::new(num::cast::<i8, T>(0).unwrap());
        nrm.read(s);
        PosNrm {
            pos: pos,
            nrm: nrm,
        }
    }
}

impl<E> Vertex for PosNrm<E> where E: VectorElement {
}

at3!(get_pos, HasPosition, pos, PosNrm);
at3!(get_nrm, HasNormal, nrm, PosNrm);
