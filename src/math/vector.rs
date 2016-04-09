use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign
};

use std::convert::{
    Into
};

pub trait Vec <VecType, Output> {
    fn dot(&self, o: VecType) -> Output;
}

#[derive(Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<'a, T> Add for &'a Vec3<T> where T: Add<Output=T> + Copy {
    type Output = Vec3<T>;

    fn add(self, other: &'a Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<'a, T> AddAssign<&'a Vec3<T>> for Vec3<T> where T: AddAssign + Copy {
    fn add_assign(&mut self, other: &'a Vec3<T>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<'a, T> Sub for &'a Vec3<T> where T: Sub<Output=T> + Copy {
    type Output = Vec3<T>;

    fn sub(self, other: &'a Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<'a, T> SubAssign<&'a Vec3<T>> for Vec3<T> where T: SubAssign + Copy {
    fn sub_assign(&mut self, other: &'a Vec3<T>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<'a, T> Mul for &'a Vec3<T> where T: Mul<Output=T> + Copy {
    type Output = Vec3<T>;

    fn mul(self, other: &'a Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl<'a, 'b, T, F> Mul<&'b F> for &'a Vec3<T> where T: Mul<Output=T> + Copy, F: Into<T> + Copy {
    type Output = Vec3<T>;

    fn mul(self, other: &'b F) -> Vec3<T> {
        let ast: T = (*other).into();
        Vec3 {
            x: self.x * ast,
            y: self.y * ast,
            z: self.z * ast,
        }
    }
}

impl<'a, T> MulAssign<&'a Vec3<T>> for Vec3<T> where T: MulAssign + Copy {
    fn mul_assign(&mut self, other: &'a Vec3<T>) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<'b, T, F> MulAssign<&'b F> for Vec3<T> where T: MulAssign + Copy, F: Into<T> + Copy {
    fn mul_assign(&mut self, other: &'b F) {
        let ast: T = (*other).into();
        self.x *= ast;
        self.y *= ast;
        self.z *= ast;
    }
}

impl<'a, T> Div for &'a Vec3<T> where T: Div<Output=T> + Copy {
    type Output = Vec3<T>;

    fn div(self, other: &'a Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl<'a, 'b, T, F> Div<&'b F> for &'a Vec3<T> where T: Div<Output=T> + Copy, F: Into<T> + Copy {
    type Output = Vec3<T>;

    fn div(self, other: &'b F) -> Vec3<T> {
        let ast: T = (*other).into();
        Vec3 {
            x: self.x / ast,
            y: self.y / ast,
            z: self.z / ast,
        }
    }
}

impl<'a, T> DivAssign<&'a Vec3<T>> for Vec3<T> where T: DivAssign + Copy {
    fn div_assign(&mut self, other: &'a Vec3<T>) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl<'b, T, F> DivAssign<&'b F> for Vec3<T> where T: DivAssign + Copy, F: Into<T> + Copy {
    fn div_assign(&mut self, other: &F) {
        let ast: T = (*other).into();
        self.x /= ast;
        self.y /= ast;
        self.z /= ast;
    }
}

impl<'a, 'b, T> Vec<&'b Vec3<T>, T> for &'a Vec3<T> where T: Mul<Output=T> + Add<Output=T> + Copy {
    fn dot(&self, o: &'b Vec3<T>) -> T {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
}
