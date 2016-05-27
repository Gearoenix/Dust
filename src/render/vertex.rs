extern crate num;

use ::math::vector::{
    Vec3,
    Vec2,
    MathVector,
    VectorElement,
};

use ::io::file::Stream;

pub trait Vertex<E>: Sized + Clone + Copy where E: VectorElement {
    fn elements_count() -> usize;
    fn new() -> Self;
    fn read(&mut self, s: &mut Stream);
}

pub trait HasPosition<E>: Vertex<E> where E: VectorElement {
    fn get_pos(&self) -> &Vec3<E>;
}

pub trait HasNormal<E>: Vertex<E> where E: VectorElement {
    fn get_nrm(&self) -> &Vec3<E>;
}

pub trait HasUV<E>: Vertex<E> where E: VectorElement {
    fn get_uv(&self) -> &Vec2<E>;
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

macro_rules! nc {
    ($e:expr) => {
        num::cast($e).unwrap()
    }
}

impl<E> Vertex<E> for PosNrmUV<E> where E: VectorElement {
    fn new() -> PosNrmUV<E> {
        PosNrmUV {
            pos: Vec3::new(nc!(0)),
            nrm: Vec3::new(nc!(0)),
            uv:  Vec2::new(nc!(0)),
        }
    }

    fn read(&mut self, s: &mut Stream) {
        self.pos.read(s);
        self.nrm.read(s);
        self.uv.read(s);
    }

    fn elements_count() -> usize {
        8
    }
}

at3!(get_pos, HasPosition, pos, PosNrmUV);
at3!(get_nrm, HasNormal, nrm, PosNrmUV);
at2!(get_uv, HasUV, uv, PosNrmUV);

#[derive(Debug, Clone, Copy)]
pub struct PosNrm<T> where T: VectorElement {
    pub pos: Vec3<T>,
    pub nrm: Vec3<T>,
}

impl<E> PosNrm<E> where E: VectorElement {
    fn elements_count() -> usize {
        6
    }
}

impl<E> Vertex<E> for PosNrm<E> where E: VectorElement {
    fn new() -> PosNrm<E> {
        PosNrm {
            pos: Vec3::new(nc!(0)),
            nrm: Vec3::new(nc!(0)),
        }
    }

    fn read(&mut self, s: &mut Stream) {
        self.pos.read(s);
        self.nrm.read(s);
    }

    fn elements_count() -> usize {
        6
    }
}

at3!(get_pos, HasPosition, pos, PosNrm);
at3!(get_nrm, HasNormal, nrm, PosNrm);
