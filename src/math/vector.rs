use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

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

        impl<'a> $tra<f64> for &'a Vec3 {
            type Output = Vec3;
            fn $func(self, f: f64) -> Vec3 {
                Vec3 {
                    x: as_expr!(self.x $opt f),
                    y: as_expr!(self.y $opt f),
                    z: as_expr!(self.z $opt f),
                }
            }
        }
    };
}

op3!(add, Add, +);
op3!(sub, Sub, -);
op3!(mul, Mul, *);
op3!(div, Div, /);

macro_rules! opasg3 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl<'a> $tra<&'a Vec3> for Vec3 {
            fn $func(&mut self, other: &'a Vec3) {
                as_expr!(self.x $opt other.x);
                as_expr!(self.y $opt other.y);
                as_expr!(self.z $opt other.z);
            }
        }

        impl $tra<f64> for Vec3 {
            fn $func(&mut self, f: f64) {
                as_expr!(self.x $opt f);
                as_expr!(self.y $opt f);
                as_expr!(self.z $opt f);
            }
        }
    };
}

opasg3!(add_assign, AddAssign, +=);
opasg3!(sub_assign, SubAssign, -=);
opasg3!(mul_assign, MulAssign, *=);
opasg3!(div_assign, DivAssign, /=);

impl<'a> Neg for &'a Vec3 {
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
    pub fn new() -> Self {
        Self {
            x: 0f64,
            y: 0f64,
            z: 0f64,
        }
    }

    pub fn dot(&self, o: &Self) -> f64 {
        (self.x * o.x) + (self.y * o.y) + (self.z * o.z)
    }

    pub fn cross(&self, o: &Vec3) -> Self {
        Self {
            x: self.y * o.z - self.z * o.y,
            y: self.z * o.x - self.x * o.z,
            z: self.x * o.y - self.y * o.x,
        }
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn absolute_length(&self) -> f64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn square_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

macro_rules! op2 {
    ($func:ident, $tra:ident, $opt:tt) => {
        impl<'a, 'b> $tra<&'b Vec2> for &'a Vec2 {
            type Output = Vec2;
            fn $func(self, other: &'b Vec2) -> Vec2 {
                Vec2 {
                    x: as_expr!(self.x $opt other.x),
                    y: as_expr!(self.y $opt other.y),
                }
            }
        }

        impl<'a> $tra<f64> for &'a Vec2 {
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
        impl<'a> $tra<&'a Vec2> for Vec2 {
            fn $func(&mut self, other: &'a Vec2) {
                as_expr!(self.x $opt other.x);
                as_expr!(self.y $opt other.y);
            }
        }

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

opasg2!(add_assign, AddAssign, +=);
opasg2!(sub_assign, SubAssign, -=);
opasg2!(mul_assign, MulAssign, *=);
opasg2!(div_assign, DivAssign, /=);

impl<'a> Neg for &'a Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vec2 {
    pub fn new() -> Self {
        Self { x: 0f64, y: 0f64 }
    }

    pub fn dot(&self, o: &Self) -> f64 {
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
        let len = self.length();
        self.x /= len;
        self.y /= len;
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
        }
    }
}
