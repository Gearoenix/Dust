use std::ops::{
    Add,
    Sub,
    Mul,
    AddAssign,
    SubAssign,
    MulAssign
};

use std::convert::{
    Into
};

#[derive(Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Add for Vec3<T> where T: Add<Output=T> {
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign for Vec3<T> where T: AddAssign {
    fn add_assign(&mut self, other: Vec3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> Sub for Vec3<T> where T: Sub<Output=T> {
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> SubAssign for Vec3<T> where T: SubAssign {
    fn sub_assign(&mut self, other: Vec3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<T> Mul for Vec3<T> where T: Mul<Output=T> {
    type Output = Vec3<T>;

    fn mul(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<T, F> Mul<F> for Vec3<T> where T: Mul<Output=T> + Copy, F: Into<T> + Copy {
    type Output = Vec3<T>;

    fn mul(self, other: F) -> Vec3<T> {
        let ast: T = other.into();  // I don't know compiler will do castability in compile time or in run time
        Vec3 {
            x: self.x * ast,
            y: self.y * ast,
            z: self.z * ast,
        }
    }
}

impl<T> MulAssign for Vec3<T> where T: MulAssign {
    fn mul_assign(&mut self, other: Vec3<T>) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T, F> MulAssign<F> for Vec3<T> where T: MulAssign + Copy, F: Into<T> + Copy {
    fn mul_assign(&mut self, other: F) {
        let ast: T = other.into();
        self.x *= ast;
        self.y *= ast;
        self.z *= ast;
    }
}
