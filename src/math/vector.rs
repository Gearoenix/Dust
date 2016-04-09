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

// use std::num::{
//     sqrt,
//     abs
// };

use std::convert::{
    Into
};

pub enum Axis {
    X,
    Y,
    Z,
    W,
}

pub trait Vec <Output> {
    fn dot<'a, 'b>(&'a self, o: &'b Self) -> Output;
    fn cross<'a, 'b>(&'a self, o: &'b Self) -> Self;
    fn length<'a>(&'a self) -> Output;
    fn absolute_length<'a>(&'a self) -> Output;
    fn square_length<'a>(&'a self) -> Output;
    fn normalize<'a>(&'a mut self);
    fn normalized<'a>(&'a self) -> Self;
}

#[derive(Debug, Clone)]
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

impl<'a, 'b, T> Mul<&'b Vec3<T>> for &'a Vec3<T> where T: Mul<Output=T> + Copy {
    type Output = Vec3<T>;

    fn mul(self, other: &'b Vec3<T>) -> Vec3<T> {
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

impl<'a, 'b, T> Div<&'b Vec3<T>> for &'a Vec3<T> where T: Div<Output=T> + Copy {
    type Output = Vec3<T>;

    fn div(self, other: &'b Vec3<T>) -> Vec3<T> {
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

impl<T> Vec<T> for Vec3<T> where T: Mul<Output=T> + Add<Output=T> + Sub<Output=T> + Div<Output=T> + DivAssign + Into<f64> + From<f64> + Copy {
    fn dot<'a, 'b>(&'a self, o: &'b Vec3<T>) -> T {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    fn cross<'a, 'b>(&'a self, o: &'b Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x
        }
    }

    fn length<'a>(&'a self) -> T {
        ((self.x * self.x + self.y * self.y + self.z * self.z).into()).sqrt().into()
    }

    fn absolute_length<'a>(&'a self) -> T {
        (self.x.into().abs() + self.y.into().abs() + self.z.into().abs()).into()
    }

    fn square_length<'a>(&'a self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn normalize<'a>(&'a mut self) {
        let len = ((self.x * self.x + self.y * self.y + self.z * self.z).into()).sqrt().into();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    fn normalized<'a>(&'a self) -> Vec3<T> {
        let len = ((self.x * self.x + self.y * self.y + self.z * self.z).into()).sqrt().into();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }
}
