use super::super::math::ray::Ray3;
use super::super::math::vector::Vec3;

pub trait Camera: Sync + Send {
    fn rotate_localy(&mut self, d: f64, v: &Vec3);
    fn get_ray(&self, x: f64, y: f64) -> Ray3;
}

pub struct Base {
    screen_ratio: f64,
    location: Vec3,
    screen_x_axis: Vec3,
    screen_y_axis: Vec3,
    screen_z_axis: Vec3,
}

impl Base {
    pub fn new(location: &Vec3, target: &Vec3, up: &Vec3, screen_ratio: f64) -> Base {
        let screen_z_axis = (target - location).normalized();
        let screen_x_axis = screen_z_axis.cross(up).normalized();
        let screen_y_axis = screen_x_axis.cross(&screen_z_axis).normalized();
        println!("screen_x_axis: {:?},\nscreen_y_axis: {:?},\nscreen_z_axis: {:?}",
            screen_x_axis, screen_y_axis, screen_z_axis,
        );
        Base {
            screen_ratio: screen_ratio,
            location: *location,
            screen_x_axis: screen_x_axis,
            screen_y_axis: screen_y_axis,
            screen_z_axis: screen_z_axis,
        }
    }
}

pub struct OrthoCamera {
    base: Base,
}

impl OrthoCamera {
    pub fn new(base: Base) -> OrthoCamera {
        OrthoCamera {
            base: base,
        }
    }
}

impl Camera for OrthoCamera {
    fn rotate_localy(&mut self, _d: f64, _v: &Vec3) {
        unimplemented!();
    }

    fn get_ray(&self, x: f64, y: f64) -> Ray3 {
        let screen_point = &(&self.base.screen_x_axis * (x * self.base.screen_ratio)) +
            &(&self.base.screen_y_axis * y);
        Ray3::new(&screen_point, &self.base.screen_z_axis)
    }
}

pub struct PerspectiveCamera {
    base: Base,
}

impl PerspectiveCamera {
    pub fn new(base: Base) -> Self {
        PerspectiveCamera {
            base: base,
        }
    }
}

impl Camera for PerspectiveCamera {
    fn rotate_localy(&mut self, _d: f64, _v: &Vec3) {
        unimplemented!();
    }

    fn get_ray(&self, x: f64, y: f64) -> Ray3 {
        let screen_point = &(&(&self.base.screen_x_axis * (x * self.base.screen_ratio)) +
            &(&self.base.screen_y_axis * y)) + &self.base.screen_z_axis;
        Ray3::new(&screen_point, &(&screen_point - &self.base.location).normalized())
    }
}
