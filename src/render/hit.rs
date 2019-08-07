use super::super::math::vector::Vec3;
use super::super::math::ray::Ray3;
use super::material::Material;

pub struct Info<'a> {
    pub t: f64,  
    pub p: Vec3,
    pub n: Vec3, 
    pub m: &'a dyn Material,
}

pub trait Hitable {
    fn hit<'a>(&'a self, r: &Ray3, t_min: f64, t_max: f64) -> Option<Info<'a>>;
}