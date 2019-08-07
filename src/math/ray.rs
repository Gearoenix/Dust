use super::vector::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray3 {
    pub o: Vec3,
    pub d: Vec3,
}

impl Ray3 {
    pub fn new(o: Vec3, d: Vec3) -> Self {
        Self {
            o: o,
            d: d,
        }
    }

    fn point_at_parameter(&self, t: f64) -> Vec3 { 
        &self.o + &(&self.d * t)
    }
}
