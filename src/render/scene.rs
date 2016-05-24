extern crate num;

use std::collections::HashMap;

use ::io::file::Stream;
use ::math::ray::Ray3;
use ::math::vector::{
    Vec2,
    Vec3,
    MathVector,
    VectorElement,
};
use ::math::triangle::Triangle;
use ::render::camera::{
    Camera,
    camera_reader,
};
use ::render::geometry::BasicGeometry;

pub struct Scene {
    pub gs: Vec<BasicGeometry>,
    pub gm: HashMap<String, usize>,
    // pub gt: TODO KDTree for geometries in scene
    pub cameras: Vec<Box<Camera<f64>>>,
    pub active_camera_index: usize,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            gs: Vec::new(),
            gm: HashMap::new(),
            cameras: Vec::new(),
            active_camera_index: 0,
        }
    }

    pub fn read(&mut self, s: &mut Stream) {
        let objects_count = s.read(&0u16);
        for _ in 0..objects_count {
            s.read(&0u8);
            s.read_string();
            s.read(&0u32);
        }
        for _ in 0..objects_count {
            let object_type = s.read(&0u8);
            let name = s.read_string();
            match object_type {
                1 => {
                    let mut g = BasicGeometry::new();
                    g.read(s);
                    self.gm.insert(name, self.gs.len());
                    self.gs.push(g);
                }
                4 => {
                    self.active_camera_index = self.cameras.len();
                    self.cameras.push(camera_reader(s));
                }
                5 => {
                    panic!("Sun are not supported yet.");
                }
                _ => {
                    panic!("Try to import unknown object type.");
                }
            }
        }
        let copy_geoes_count = s.read(&0u16);
        if copy_geoes_count > 1 {
            panic!("Copy geometries are not supported yet.");
        }
    }

    fn trace_ray<E>(&self, r: &Ray3<E>, d: u32) -> Vec3<E> where E: VectorElement {
        // TODO search by kdtree for geometries
        for g in self.gs {

        }
        Vec3::new(num::cast(1).unwrap())
    }
}
