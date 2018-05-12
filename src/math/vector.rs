use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// use ::io::file::Stream;

pub enum Axis {
    X,
    Y,
    Z,
    W,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

macro_rules! as_expr {
    ($e:expr) => {
        $e
    };
}

macro_rules! op3 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl<'a, 'b> $tra<&'b Vec3> for &'a Vec3 {
            type Output = Vec3;
            fn $func(self, other: &'b Vec3) -> Vec3 {
                Vec3 {
                    x: as_expr!(self.x $opt other.x),
                    y: as_expr!(self.y $opt other.y),
                    z: as_expr!(self.z $opt other.z),
                }
            }
        }
    };
}

op3!(add, Add, +);
op3!(sub, Sub, -);
op3!(mul, Mul, *);
op3!(div, Div, /);

macro_rules! sop3 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl<'a> $tra<f64> for &'a Vec3 {
            type Output = Vec3;
            fn $func(self, other: f64) -> Vec3 {
                Vec3 {
                    x: as_expr!(self.x $opt other),
                    y: as_expr!(self.y $opt other),
                    z: as_expr!(self.z $opt other),
                }
            }
        }
    };
}

sop3!(add, Add, +);
sop3!(sub, Sub, -);
sop3!(mul, Mul, *);
sop3!(div, Div, /);

macro_rules! opasg3 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl<'a> $tra<&'a Vec3> for Vec3 {
            fn $func(&mut self, other: &'a Vec3) {
                as_expr!(self.x $opt other.x);
                as_expr!(self.y $opt other.y);
                as_expr!(self.z $opt other.z);
            }
        }
    };
}

opasg3!(add_assign, AddAssign, +=);
opasg3!(sub_assign, SubAssign, -=);
opasg3!(mul_assign, MulAssign, *=);
opasg3!(div_assign, DivAssign, /=);

macro_rules! sopasg3 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl $tra<f64> for Vec3 {
            fn $func(&mut self, other: f64) {
                as_expr!(self.x $opt other);
                as_expr!(self.y $opt other);
                as_expr!(self.z $opt other);
            }
        }
    };
}

sopasg3!(add_assign, AddAssign, +=);
sopasg3!(sub_assign, SubAssign, -=);
sopasg3!(mul_assign, MulAssign, *=);
sopasg3!(div_assign, DivAssign, /=);

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    pub fn dot(&self, o: &Vec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(&self, o: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn absolute_length(&self) -> f64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn square_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn normalized(&self) -> Vec3 {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    // pub fn read(&mut self, s: &mut Stream) {
    //     self.x = s.read(&0f32) as f64;
    //     self.y = s.read(&0f32) as f64;
    //     self.z = s.read(&0f32) as f64;
    // }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

macro_rules! op2 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl $tra for Vec2 {
            type Output = Vec2;
            fn $func(self, other: Vec2) -> Vec2 {
                Vec2 {
                    x: as_expr!(self.x $opt other.x),
                    y: as_expr!(self.y $opt other.y),
                }
            }
        }
    };
}

macro_rules! sop2 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl $tra<f64> for Vec2 {
            type Output = Vec2;
            fn $func(self, other: f64) -> Vec2 {
                Vec2 {
                    x: as_expr!(self.x $opt other),
                    y: as_expr!(self.y $opt other),
                }
            }
        }
    };
}

macro_rules! opasg2 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl $tra for Vec2 {
            fn $func(&mut self, other: Vec2) {
                as_expr!(self.x $opt other.x);
                as_expr!(self.y $opt other.y);
            }
        }
    };
}

macro_rules! sopasg2 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl $tra<f64> for Vec2 {
            fn $func(&mut self, other: f64) {
                as_expr!(self.x $opt other);
                as_expr!(self.y $opt other);
            }
        }
    };
}

op2!(add, Add, +);
op2!(sub, Sub, -);
op2!(mul, Mul, *);
op2!(div, Div, /);

sop2!(add, Add, +);
sop2!(sub, Sub, -);
sop2!(mul, Mul, *);
sop2!(div, Div, /);

opasg2!(add_assign, AddAssign, +=);
opasg2!(sub_assign, SubAssign, -=);
opasg2!(mul_assign, MulAssign, *=);
opasg2!(div_assign, DivAssign, /=);

sopasg2!(add_assign, AddAssign, +=);
sopasg2!(sub_assign, SubAssign, -=);
sopasg2!(mul_assign, MulAssign, *=);
sopasg2!(div_assign, DivAssign, /=);

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec2 {
    pub fn new() -> Vec2 {
        Vec2 { x: 0f64, y: 0f64 }
    }

    pub fn dot(&self, o: &Vec2) -> f64 {
        self.x * o.x + self.y * o.y
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn absolute_length(&self) -> f64 {
        self.x.abs() + self.y.abs()
    }

    pub fn square_length(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&mut self) {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        self.x /= len;
        self.y /= len;
    }

    pub fn normalized(&self) -> Vec2 {
        let len = (self.x * self.x + self.y * self.y).sqrt();
        Vec2 {
            x: self.x / len,
            y: self.y / len,
        }
    }

    // pub fn read(&mut self, s: &mut Stream) {
    //     self.x = s.read(&0f32) as f64;
    //     self.y = s.read(&0f32) as f64;
    // }
}
