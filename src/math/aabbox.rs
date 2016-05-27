use ::math::vector::{
    Vec3,
    Axis,
};

use ::math::ray::Ray3;

pub trait ExpandableToOther {
    fn expand(&mut self, o: &Self);
}

pub trait ExpandableToPoint3 {
    fn expand(&mut self, o: &Vec3);
}

#[derive(Debug, Clone, Copy)]
pub struct AABBox3 {
    pub blf: Vec3,
    pub trr: Vec3,
}

impl ExpandableToOther for AABBox3 {
    fn expand(&mut self, o : &Self) {
        if o.blf.x < self.blf.x { self.blf.x = o.blf.x; }
        if o.blf.y < self.blf.y { self.blf.y = o.blf.y; }
        if o.blf.z < self.blf.z { self.blf.z = o.blf.z; }

        if o.trr.x > self.trr.x { self.trr.x = o.trr.x; }
        if o.trr.y > self.trr.y { self.trr.y = o.trr.y; }
        if o.trr.z > self.trr.z { self.trr.z = o.trr.z; }
    }
}

impl ExpandableToPoint3 for AABBox3 {
    fn expand(&mut self, p : &Vec3) {
        if p.x < self.blf.x { self.blf.x = p.x; }
        if p.y < self.blf.y { self.blf.y = p.y; }
        if p.z < self.blf.z { self.blf.z = p.z; }
    }
}

impl AABBox3 {
    pub fn new() -> AABBox3 {
        AABBox3 {
            blf: Vec3::new(),
            trr: Vec3::new(),
        }
    }

    pub fn get_longest_axis(&self) -> Axis {
        let diff = self.trr - self.blf; // TODO check for occurance, if it is too much store it in box
        if diff.x > diff.y && diff.x > diff.z { return Axis::X; }
        if diff.y > diff.x && diff.y > diff.z { return Axis::Y; }
        return Axis::Z;
    }

    // Check if ray intersects with box. Returns true/false and stores distance in t
    pub fn intersection(&self, r: &Ray3) -> (bool, f64) {

        let tx1 = (self.blf.x - r.o.x) * r.invd.x;
        let tx2 = (self.trr.x - r.o.x) * r.invd.x;

        let mut tmin = tx1.min(tx2);
        let mut tmax = tx1.max(tx2);

        let ty1 = (self.blf.y - r.o.y) * r.invd.y;
        let ty2 = (self.trr.y - r.o.y) * r.invd.y;

        tmin = tmin.max(ty1.min(ty2));
        tmax = tmax.min(ty1.max(ty2));

        let tz1 = (self.blf.z - r.o.z) * r.invd.z;
        let tz2 = (self.trr.z - r.o.z) * r.invd.z;

        tmin = tmin.max(tz1.min(tz2));
        tmax = tmax.min(tz1.max(tz2));

        let t = tmin;

        return (tmax >= tmin, t);
    }
}
