use super::super::render::hit::{Hitable, Info as HitInfo};
use super::super::render::material::Material;
use super::ray::Ray3;
use super::vector::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

impl Hitable for Sphere {
    fn hit<'a>(&'a self, r: &Ray3, t_min: f64, t_max: f64) -> Option<HitInfo<'a>> {
        let oc = &r.o - &self.center;
        let a = r.d.dot(&r.d);
        let b = oc.dot(&r.d);
        let c = oc.dot(&oc) - (self.radius * self.radius);
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitInfo {
                    t: temp,
                    p,
                    n: &(&p - &self.center) / self.radius,
                    m: self.material.as_ref(),
                });
            }
            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = r.point_at_parameter(temp);
                return Some(HitInfo {
                    t: temp,
                    p,
                    n: &(&p - &self.center) / self.radius,
                    m: self.material.as_ref(),
                });
            }
        }
        None
    }
}
