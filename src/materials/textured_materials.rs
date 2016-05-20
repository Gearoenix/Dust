use ::materials::material::Material;
use ::math::vector::VectorElement;

pub trait TexturedMaterial: Material {

}

pub struct BasicTexturedMaterial<E> where E: VectorElement {
    diffuse: E,
}

impl<E> BasicTexturedMaterial<E> where E: VectorElement {
    pub fn new(diffuse: &E) -> BasicTexturedMaterial<E> {
        BasicTexturedMaterial {
            diffuse: *diffuse,
        }
    }
}

impl<E> Material for BasicTexturedMaterial<E> where E: VectorElement {}

impl<E> TexturedMaterial for BasicTexturedMaterial<E> where E: VectorElement {}
