use ::math::vector::{
    Vec3,
    MathVector,
    VectorElement,
};

#[derive(Debug, Clone, Copy)]
pub struct Ray3<T> where T: VectorElement, Vec3<T>: MathVector<T> {
    pub o:    Vec3<T>,
    pub d:    Vec3<T>,
    pub invd: Vec3<T>,
}
