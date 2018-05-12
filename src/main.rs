extern crate cgmath;
extern crate image;

use cgmath::{
    dot,
    Vector3,
};

fn main() {
    const ASPECT: isize = 1024;
    const HALF_ASPECT: isize = ASPECT >> 1;
    const SAMPLES: isize = 4;
    const SAMPLES_SQUAR: isize = SAMPLES * SAMPLES;
    const STEP_ASPECT: f64 = 1f64 / HALF_ASPECT as f64;
    const STEP_SAMPLE: f64 = STEP_ASPECT / SAMPLES as f64;
    let mut pixels = [0u8; (ASPECT * ASPECT * 4) as usize];
    let v1 = Vector3::new(-1f64, 0f64, 0f64);
    let v2 = Vector3::new(0f64, 1f64, 0f64);
    let v3 = Vector3::new(1f64, 0f64, 0f64);
    let edge1 = v2 - v1;
    let edge2 = v3 - v1;
    let ray_origin = Vector3::new(0f64, 0f64, 2f64);
    let mut y = 1f64;
    let mut px_index = 0;
    for _ in 0..ASPECT {
        let mut x = -1f64;
        for _ in 0..ASPECT {
            let mut samples = [0u64; 3];
            let mut sy = y;
            for _ in 0..SAMPLES {
                let mut sx = x;
                for _ in 0..SAMPLES {
                    let ray_dir = Vector3::new(sx, sy, -2f64);
                    let pvec = ray_dir.cross(edge2);
                    let det = dot(edge1, pvec);
                    if det < 0.0001 && det > -0.0001 {
                        sx += STEP_SAMPLE;
                        continue;
                    }
                    let inv_det = 1f64 / det;
                    let tvec = ray_origin - v1;
                    let u = dot(tvec, pvec) * inv_det;
                    if u < 0.0001f64 || u > 0.9999f64 {
                        sx += STEP_SAMPLE;
                        continue;
                    }
                    let qvec = tvec.cross(edge1);
                    let v = dot(ray_dir, qvec) * inv_det;
                    if v < 0.0001f64 || u + v > 0.9999f64 {
                        sx += STEP_SAMPLE;
                        continue;
                    }
                    let t = dot(edge2, qvec) * inv_det;
                    if t > 0.0001 {
                        samples[0] += 255;
                        samples[1] += 255;
                        samples[2] += 255;
                    }
                    sx += STEP_SAMPLE;
                }
                sy -= STEP_SAMPLE;
            }
            pixels[px_index] = (samples[0] / SAMPLES_SQUAR as u64) as u8;
            px_index += 1;
            pixels[px_index] = (samples[1] / SAMPLES_SQUAR as u64) as u8;
            px_index += 1;
            pixels[px_index] = (samples[0] / SAMPLES_SQUAR as u64) as u8;
            px_index += 1;
            pixels[px_index] = 255u8;
            px_index += 1;
            x += STEP_ASPECT;
        }
        y -= STEP_ASPECT;
    }
    image::save_buffer("image.png", &pixels, ASPECT as u32, ASPECT as u32, image::RGBA(8)).unwrap();
}
