extern crate num;

use ::materials::material::Material;
use ::math::vector::VectorElement;

pub trait TexturedMaterial<E>: Material<E> where E: VectorElement {

}


#[derive(Debug, Clone)]
pub struct BasicTexturedMaterial<E> where E: VectorElement {
    diffuse: E,
}

impl<E> BasicTexturedMaterial<E> where E: VectorElement {
    pub fn new() -> BasicTexturedMaterial<E> {
        BasicTexturedMaterial {
            diffuse: num::cast(0).unwrap(),
        }
    }
}

impl<E> Material<E> for BasicTexturedMaterial<E> where E: VectorElement {
}

impl<E> TexturedMaterial<E> for BasicTexturedMaterial<E> where E: VectorElement {}
