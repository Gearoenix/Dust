use ::materials::material::Material;
use ::math::vector::VectorElement;

pub trait SolidMaterial: Material {

}

pub struct BasicSolidMaterial<E> where E: VectorElement {
    diffuse: E,
}

impl<E> BasicSolidMaterial<E> where E: VectorElement {
    pub fn new(diffuse: &E) -> BasicSolidMaterial<E> {
        BasicSolidMaterial {
            diffuse: *diffuse,
        }
    }
}

impl<E> Material for BasicSolidMaterial<E> where E: VectorElement {}

impl<E> SolidMaterial for BasicSolidMaterial<E> where E: VectorElement {}
