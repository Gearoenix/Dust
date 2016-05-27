use ::io::file::Stream;
use ::math::matrix::{
    Mat4x4,
};
use ::math::ray::Ray3;
use ::math::vector::Vec3;

pub trait Camera {
    fn read(&mut self, s: &mut Stream);
    fn rotate_localy(&mut self, d: f64, v: &Vec3);
    fn get_ray(&self, x: f64, y: f64) -> Ray3;
}

struct Base {
    transform: Mat4x4,
    transformed: bool,
    screen_width: f64,
    screen_height: f64,
    screen_center: Vec3,
    screen_x_axis: Vec3,
    screen_y_axis: Vec3,
    screen_width_axis: Vec3,
    screen_height_axis: Vec3,
    screen_plane_constant: f64,
}

pub struct OrthoCamera {
    base: Base,
    screen_normal: Vec3,
}

pub struct PerspectiveCamera {
    position: Vec3,
    base: Base,
}

impl Base {
    fn new() -> Base {
        Base {
            transform: Mat4x4::new(),
            transformed: false,
            screen_width: 1.8f64,
            screen_height: 1.0,
            screen_center: Vec3::new(),
            screen_x_axis: Vec3 {x: 1.0, y: 0.0, z: 0.0},
            screen_y_axis: Vec3 {x: 0.0, y: 1.0, z: 0.0},
            screen_width_axis: Vec3 {x: 0.9, y: 0.0, z: 0.0},
            screen_height_axis: Vec3 {x: 0.0, y: 0.5, z: 0.0},
            screen_plane_constant: 0.0,
        }
    }

    // returns Position, Normal, Near, Far
    fn read(&mut self, s: &mut Stream) -> (Vec3, Vec3, f64, f64) {
        let mut position = Vec3::new();
        position.read(s);

        let x_3d_axis = Vec3 {x: 1.0, y: 0.0, z: 0.0};
        let y_3d_axis = Vec3 {x: 0.0, y: 1.0, z: 0.0};
        let z_3d_axis = Vec3 {x: 0.0, y: 0.0, z: 1.0};

        let rx = s.read(&0f32) as f64;
        let ry = s.read(&0f32) as f64;
        let rz = s.read(&0f32) as f64;

        let transform = Mat4x4::rotation_transform(rx, &x_3d_axis) * Mat4x4::rotation_transform(ry, &y_3d_axis) * Mat4x4::rotation_transform(rz, &z_3d_axis);

        let near = s.read(&0f32) as f64;

        let mut screen_center = Vec3 {x: 0.0, y: 0.0, z: -near};
        screen_center = transform * screen_center;

        let mut screen_normal = Vec3 {x: 0.0, y: 0.0, z: -1.0};
        screen_normal = transform * screen_normal;
        screen_normal.normalize();

        let mut screen_x_axis = Vec3 {x: 1.0, y: 0.0, z: 0.0};
        screen_x_axis = transform * screen_x_axis;
        screen_x_axis.normalize();

        let mut screen_y_axis = Vec3 {x: 0.0, y: 1.0, z: 0.0};
        screen_y_axis = transform * screen_y_axis;
        screen_y_axis.normalize();

        let screen_plane_constant = screen_center.dot(&screen_normal);

        let far = s.read(&0f32) as f64;

        self.screen_center = screen_center;
        self.screen_x_axis = screen_x_axis;
        self.screen_y_axis = screen_y_axis;
        self.screen_width_axis = screen_x_axis * (self.screen_width / 2.0);
        self.screen_height_axis = screen_y_axis * (self.screen_height / 2.0);
        self.screen_plane_constant = screen_plane_constant;

        (position, screen_normal, near, far)
    }
}

impl OrthoCamera {
    pub fn new() -> OrthoCamera {
        OrthoCamera {
            base: Base::new(),
            screen_normal: Vec3 {x: 0.0, y: 0.0, z: 1.0}
        }
    }
}

impl Camera for OrthoCamera {
    fn read(&mut self, s: &mut Stream) {
        let (_, screen_normal, _, _) = self.base.read(s);
        self.screen_normal = screen_normal;
    }

    fn rotate_localy(&mut self, d: f64, v: &Vec3) {
        // TODO
    }

    fn get_ray(&self, x: f64, y: f64) -> Ray3 {
        let screen_point = self.base.screen_width_axis * x + self.base.screen_y_axis * y;
        Ray3::new(&screen_point, &self.screen_normal)
    }
}

impl PerspectiveCamera {
    pub fn new() -> PerspectiveCamera {
        PerspectiveCamera {
            position: Vec3 {x: 0.0, y: 0.0, z: 1.0},
            base: Base::new(),
        }
    }
}

impl Camera for PerspectiveCamera {
    fn read(&mut self, s: &mut Stream) {
        let (position, _, near, _) = self.base.read(s);
        let tan_field_of_view = (s.read(&0f32) as f64).tan();
        self.base.screen_width = tan_field_of_view * near;
        self.base.screen_height = self.base.screen_width / 1.8;
        self.base.screen_width_axis = self.base.screen_x_axis * (self.base.screen_width / 2.0);
        self.base.screen_height_axis = self.base.screen_y_axis * (self.base.screen_height / 2.0);
        self.position = position;
    }

    fn rotate_localy(&mut self, d: f64, v: &Vec3) {
        // TODO
    }

    fn get_ray(&self, x: f64, y: f64) -> Ray3 {
        let screen_point = self.base.screen_width_axis * x + self.base.screen_y_axis * y;
        Ray3::new(&screen_point, &(screen_point - self.position).normalized())
    }
}

pub fn camera_reader(s: &mut Stream) -> Box<Camera> {
    let camera_type = s.read(&0u8);
    let mut camera: Box<Camera>;
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
