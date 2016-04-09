use ::math::vector::Vec3;

#[derive(Debug, Clone)]
pub struct Ray3<T> {
    pub o:    Vec3<T>,
    pub d:    Vec3<T>,
    pub invd: Vec3<T>,
}
