extern crate num;

use ::materials::material::Material;
use ::math::vector::VectorElement;

pub trait SolidMaterial<E>: Material<E> where E: VectorElement {

}

#[derive(Debug, Clone)]
pub struct BasicSolidMaterial<E> where E: VectorElement {
    diffuse: E,
}

impl<E> BasicSolidMaterial<E> where E: VectorElement {
    pub fn new() -> BasicSolidMaterial<E> {
        BasicSolidMaterial {
            diffuse: num::cast(0).unwrap(),
        }
    }
}

impl<E> Material<E> for BasicSolidMaterial<E> where E: VectorElement {
}

impl<E> SolidMaterial<E> for BasicSolidMaterial<E> where E: VectorElement {}
