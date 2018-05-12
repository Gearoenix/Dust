extern crate cgmath;
extern crate image;

use std::sync::{Arc, RwLock, Mutex};

use cgmath::{
    dot,
    Vector3,
};

struct Renderer {
    aspect: isize,
    aspect_square: isize,
    half_aspect: isize,
    samples: isize,
    samples_square: isize,
    step_aspect: f64,
    step_sample: f64,
    v1: Vector3<f64>,
    v2: Vector3<f64>,
    v3: Vector3<f64>,
    edge1: Vector3<f64>,
    edge2: Vector3<f64>,
    ray_origin: Vector3<f64>,
}

fn main() {
    let aspect: isize = 1024;
    let aspect_square: isize = aspect * aspect;
    let half_aspect: isize = aspect >> 1;
    let samples: isize = 4;
    let samples_square: isize = samples * samples;
    let step_aspect: f64 = 1f64 / half_aspect as f64;
    let step_sample: f64 = step_aspect / samples as f64;
    let v1: Vector3<f64> = Vector3::new(-1f64, 0f64, 0f64);
    let v2: Vector3<f64> = Vector3::new(0f64, 1f64, 0f64);
    let v3: Vector3<f64> = Vector3::new(1f64, 0f64, 0f64);
    let edge1: Vector3<f64> = v2 - v1;
    let edge2: Vector3<f64> = v3 - v1;
    let ray_origin: Vector3<f64> = Vector3::new(0f64, 0f64, 2f64);
    let renderer = Renderer {
        aspect,
        aspect_square,
        half_aspect,
        samples,
        samples_square,
        step_aspect,
        step_sample,
        v1,
        v2,
        v3,
        edge1,
        edge2,
        ray_origin,
    };
    let renderer = Arc::new(RwLock::new(renderer));
    // let mut pixels = [
    //     Arc::new(Mutex::new([0u8; aspect_square as usize])), 
    //     Arc::new(Mutex::new([0u8; aspect_square as usize])),
    //     Arc::new(Mutex::new([0u8; aspect_square as usize])),
    //     Arc::new(Mutex::new([0u8; aspect_square as usize])),
    // ];
    let mut pixels = vec![0u8; (aspect_square * 4) as usize];
    let mut y = 1f64;
    let mut px_index = 0;
    for _ in 0..aspect {
        let mut x = -1f64;
        for _ in 0..aspect {
            let mut samples = [0u64; 3];
            let mut sy = y;
            for _ in 0..samples {
                let mut sx = x;
                for _ in 0..samples {
                    let ray_dir = Vector3::new(sx, sy, -2f64);
                    let pvec = ray_dir.cross(edge2);
                    let det = dot(edge1, pvec);
                    if det < 0.0001 && det > -0.0001 {
                        sx += step_sample;
                        continue;
                    }
                    let inv_det = 1f64 / det;
                    let tvec = ray_origin - v1;
                    let u = dot(tvec, pvec) * inv_det;
                    if u < 0.0001f64 || u > 0.9999f64 {
                        sx += step_sample;
                        continue;
                    }
                    let qvec = tvec.cross(edge1);
                    let v = dot(ray_dir, qvec) * inv_det;
                    if v < 0.0001f64 || u + v > 0.9999f64 {
                        sx += step_sample;
                        continue;
                    }
                    let t = dot(edge2, qvec) * inv_det;
                    if t > 0.0001 {
                        samples[0] += 255;
                        samples[1] += 255;
                        samples[2] += 255;
                    }
                    sx += step_sample;
                }
                sy -= step_sample;
            }
            pixels[px_index] = (samples[0] / samples_square as u64) as u8;
            px_index += 1;
            pixels[px_index] = (samples[1] / samples_square as u64) as u8;
            px_index += 1;
            pixels[px_index] = (samples[0] / samples_square as u64) as u8;
            px_index += 1;
            pixels[px_index] = 255u8;
            px_index += 1;
            x += step_aspect;
        }
        y -= step_aspect;
    }
    image::save_buffer("image.png", &pixels, aspect as u32, aspect as u32, image::RGBA(8)).unwrap();
}
