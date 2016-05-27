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

use ::math::vector::{
    Vec2,
    Vec3,
};
use ::io::file::Stream;

#[derive(Debug, Clone, Copy)]
pub struct Mat4x4 {
    pub data: [[f64; 4]; 4],
}

impl Mat4x4 {
    pub fn new() -> Mat4x4 {
        Mat4x4 {
            data: [
                [1f64, 0f64, 0f64, 0f64],
                [0f64, 1f64, 0f64, 0f64],
                [0f64, 0f64, 1f64, 0f64],
                [0f64, 0f64, 0f64, 1f64],
            ],
        }
    }

    pub fn rotation_transform(d: f64, v: &Vec3) -> Mat4x4 {
        let sinus: f64 = d.sin();
		let cosinus: f64 = d.cos();
		let oneminuscos = 1f64 - cosinus;
		let w = v;
		let wx2 = w.x * w.x;
		let wxy = w.x * w.y;
		let wxz = w.x * w.z;
		let wy2 = w.y * w.y;
		let wyz = w.y * w.z;
		let wz2 = w.z * w.z;
		let wxyonemincos = wxy * oneminuscos;
		let wxzonemincos = wxz * oneminuscos;
		let wyzonemincos = wyz * oneminuscos;
		let wxsin = w.x * sinus;
		let wysin = w.y * sinus;
		let wzsin = w.z * sinus;
		Mat4x4 {
            data: [
    		    [
                    cosinus + (wx2 * oneminuscos),
                    wxyonemincos - wzsin,
                    wysin + wxzonemincos,
                    0.0,
                ],
    		    [
                    wzsin + wxyonemincos,
                    cosinus + (wy2 * oneminuscos),
                    wyzonemincos - wxsin,
                    0.0,
                ],
    		    [
                    wxzonemincos - wysin,
                    wxsin + wyzonemincos,
                    cosinus + (wz2 * oneminuscos),
                    0.0,
                ],
    		    [
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                ],
            ],
        }
    }

    pub fn read(&mut self, s: &mut Stream) {
        for i in 0..4 {
            for j in 0..4 {
                self.data[j][i] = s.read(&0f32) as f64;
            }
        }
    }
}

impl Mul<Vec3> for Mat4x4 {
    type Output = Vec3;
    fn mul(self, o: Vec3) -> Vec3 {
        Vec3 {
            x: self.data[0][0] * o.x + self.data[0][1] * o.y + self.data[0][2] * o.z + self.data[0][3],
            y: self.data[1][0] * o.x + self.data[1][1] * o.y + self.data[1][2] * o.z + self.data[1][3],
            z: self.data[2][0] * o.x + self.data[2][1] * o.y + self.data[2][2] * o.z + self.data[2][3],
        }
    }
}

impl Mul<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;
    fn mul(self, o: Mat4x4) -> Mat4x4 {
        let mut m = Mat4x4::new();
        for i in 0..4 {
            for j in 0..4 {
                m.data[i][j] = 0.0;
                for k in 0..4 {
                    m.data[i][j] += self.data[i][k] * o.data[k][j];
                }
            }
        }
        m
    }
}
