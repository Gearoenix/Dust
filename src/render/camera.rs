extern crate num;

use ::io::file::Stream;
use ::math::matrix::{
    Mat4x4,
};
use ::math::ray::Ray3;
use ::math::vector::{
    Vec3,
    MathVector,
    VectorElement,
};

macro_rules! nc {
    ($var:expr) => (
        num::cast($var).unwrap()
    )
}

pub trait Camera<E> where E: VectorElement {
    fn read(&mut self, s: &mut Stream);
    fn rotate_localy(&mut self, d: E, v: Vec3<E>);
    fn get_ray(&self, x: &E, y: &E) -> Ray3<E>;
}

struct Base<E> where E: VectorElement {
    transform: Mat4x4<E>,
    transformed: bool,
    screen_width: E,
    screen_height: E,
    screen_center: Vec3<E>,
    screen_x_axis: Vec3<E>,
    screen_y_axis: Vec3<E>,
    screen_width_axis: Vec3<E>,
    screen_height_axis: Vec3<E>,
    screen_plane_constant: E,
}

pub struct OrthoCamera<E> where E: VectorElement {
    base: Base<E>,
    screen_normal: Vec3<E>,
}

pub struct PerspectiveCamera<E> where E: VectorElement {
    position: Vec3<E>,
    base: Base<E>,
}

impl<E> Base<E> where E: VectorElement {
    fn new() -> Base<E> {
        Base {
            transform: Mat4x4::new(),
            transformed: false,
            screen_width: nc!(1.8f64),
            screen_height: nc!(1),
            screen_center: Vec3::new(nc!(0)),
            screen_x_axis: Vec3 {x: nc!(1), y: nc!(0), z: nc!(0)},
            screen_y_axis: Vec3 {x: nc!(0), y: nc!(1), z: nc!(0)},
            screen_width_axis: Vec3 {x: nc!(0.9), y: nc!(0), z: nc!(0)},
            screen_height_axis: Vec3 {x: nc!(0), y: nc!(0.5), z: nc!(0)},
            screen_plane_constant: nc!(0),
        }
    }

    // returns Position, Normal, Near, Far
    fn read(&mut self, s: &mut Stream) -> (Vec3<E>, Vec3<E>, E, E) {
        let mut position = Vec3::new(num::cast::<i8, E>(0).unwrap());
        position.read(s);

        let x_3d_axis: Vec3<E> = Vec3 {x: nc!(1), y: nc!(0), z: nc!(0)};
        let y_3d_axis: Vec3<E> = Vec3 {x: nc!(0), y: nc!(1), z: nc!(0)};
        let z_3d_axis: Vec3<E> = Vec3 {x: nc!(0), y: nc!(0), z: nc!(1)};

        let rx:   E = nc!(s.read(&0f32));
        let ry:   E = nc!(s.read(&0f32));
        let rz:   E = nc!(s.read(&0f32));

        let transform = Mat4x4::rotation_transform(&rx, &x_3d_axis) * Mat4x4::rotation_transform(&ry, &y_3d_axis) * Mat4x4::rotation_transform(&rz, &z_3d_axis);

        let near: E = nc!(s.read(&0f32));

        let mut screen_center: Vec3<E> = Vec3 {x: nc!(0), y: nc!(0), z: -near};
        screen_center = transform * screen_center;

        let mut screen_normal: Vec3<E> = Vec3 {x: nc!(0), y: nc!(0), z: nc!(-1)};
        screen_normal = transform * screen_normal;
        screen_normal.normalize();

        let mut screen_x_axis: Vec3<E> = Vec3 {x: nc!(1), y: nc!(0), z: nc!(0)};
        screen_x_axis = transform * screen_x_axis;
        screen_x_axis.normalize();

        let mut screen_y_axis: Vec3<E> = Vec3 {x: nc!(0), y: nc!(1), z: nc!(0)};
        screen_y_axis = transform * screen_y_axis;
        screen_y_axis.normalize();

        let screen_plane_constant = screen_center.dot(&screen_normal);

        let far:  E = nc!(s.read(&0f32));

        self.screen_center = screen_center;
        self.screen_x_axis = screen_x_axis;
        self.screen_y_axis = screen_y_axis;
        self.screen_width_axis = screen_x_axis * (self.screen_width / nc!(2.0));
        self.screen_height_axis = screen_y_axis * (self.screen_height / nc!(2.0));
        self.screen_plane_constant = screen_plane_constant;

        (position, screen_normal, near, far)
    }
}

impl<E> OrthoCamera<E> where E: VectorElement {
    pub fn new() -> OrthoCamera<E> {
        OrthoCamera {
            base: Base::new(),
            screen_normal: Vec3 {x: nc!(0), y: nc!(0), z: nc!(1)}
        }
    }
}

impl<E> Camera<E> for OrthoCamera<E> where E: VectorElement {
    fn read(&mut self, s: &mut Stream) {
        let (_, screen_normal, _, _) = self.base.read(s);
        self.screen_normal = screen_normal;
    }

    fn rotate_localy(&mut self, d: E, v: Vec3<E>) {
        // TODO
    }

    fn get_ray(&self, x: &E, y: &E) -> Ray3<E> {
        let screen_point = self.base.screen_width_axis * *x + self.base.screen_y_axis * *y;
        Ray3::new(screen_point, self.screen_normal)
    }
}

impl<E> PerspectiveCamera<E> where E: VectorElement {
    pub fn new() -> PerspectiveCamera<E> {
        PerspectiveCamera {
            position: Vec3 {x: nc!(0), y: nc!(0), z: nc!(1)},
            base: Base::new(),
        }
    }
}

impl<E> Camera<E> for PerspectiveCamera<E> where E: VectorElement {
    fn read(&mut self, s: &mut Stream) {
        let (position, _, near, _) = self.base.read(s);
        let tan_field_of_view: E = nc!((s.read(&0f32) as f64).tan());
        self.base.screen_width = tan_field_of_view * near;
        self.base.screen_height = self.base.screen_width / nc!(1.8);
        self.base.screen_width_axis = self.base.screen_x_axis * (self.base.screen_width / nc!(2.0));
        self.base.screen_height_axis = self.base.screen_y_axis * (self.base.screen_height / nc!(2.0));
        self.position = position;
    }

    fn rotate_localy(&mut self, d: E, v: Vec3<E>) {
        // TODO
    }

    fn get_ray(&self, x: &E, y: &E) -> Ray3<E> {
        let screen_point = self.base.screen_width_axis * *x + self.base.screen_y_axis * *y;
        Ray3::new(screen_point, (screen_point - self.position).normalized())
    }
}

pub fn camera_reader<'a, E>(s: &mut Stream) -> Box<Camera<E> + 'a> where E: VectorElement + 'a {
    let camera_type = s.read(&0u8);
    let mut camera: Box<Camera<E> + 'a>;
    match camera_type {
        1 => {
            camera = Box::new(OrthoCamera::new());
        },
        2 => {
            camera = Box::new(PerspectiveCamera::new());
        },
        _ => {
            panic!("Unknown camera type.");
        },
    }
    camera.read(s);
    camera
}
