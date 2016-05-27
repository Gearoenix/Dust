use ::math::vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray3 {
    pub o:    Vec3,
    pub d:    Vec3,
    pub invd: Vec3,
}

impl Ray3 {
    pub fn new(o: &Vec3, d: &Vec3) -> Ray3 {
        Ray3 {
            o: *o,
            d: *d,
            invd: Vec3 {
                x: 1f64 / d.x,
                y: 1f64 / d.y,
                z: 1f64 / d.z,
            }
        }
    }
}
