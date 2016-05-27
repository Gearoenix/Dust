use ::math::vector::{
    Vec3,
    Vec2,
};
use ::io::file::Stream;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub ps: Vec3,
    pub nr: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            ps: Vec3::new(),
            nr: Vec3::new(),
            uv: Vec2::new(),
        }
    }

    pub fn read(&mut self, s: &mut Stream, has_nr: bool, has_texture_coordinate: bool) {
        self.ps.read(s);
        if has_nr {
            self.nr.read(s);
        }
        if has_texture_coordinate {
            self.uv.read(s);
        }
    }
}
