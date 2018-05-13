extern crate cgmath;
extern crate image;
extern crate num_cpus;

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use cgmath::{dot, Vector3};

struct Renderer {
    width: isize,
    height: isize,
    area: isize,
    half_width: isize,
    half_height: isize,
    samples: isize,
    samples_square: isize,
    step_x: f64,
    step_y: f64,
    step_sample_x: f64,
    step_sample_y: f64,
    v1: Vector3<f64>,
    v2: Vector3<f64>,
    v3: Vector3<f64>,
    edge1: Vector3<f64>,
    edge2: Vector3<f64>,
    ray_origin: Vector3<f64>,
}

fn main() {
    let threads_count = num_cpus::get();
    let width: isize = 1920;
    let height: isize = 1080;
    let height_per_thread = height / threads_count as isize;
    let height = height_per_thread * threads_count as isize;
    let area: isize = width * height;
    let half_width: isize = width >> 1;
    let half_height: isize = height >> 1;
    let samples: isize = 128;
    let samples_square: isize = samples * samples;
    let step_x: f64 = 1f64 / half_width as f64;
    let step_y: f64 = 1f64 / half_height as f64;
    let step_sample_x: f64 = step_x / samples as f64;
    let step_sample_y: f64 = step_y / samples as f64;
    let v1: Vector3<f64> = Vector3::new(-1f64, 0f64, 0f64);
    let v2: Vector3<f64> = Vector3::new(0f64, 1f64, 0f64);
    let v3: Vector3<f64> = Vector3::new(1f64, 0f64, 0f64);
    let edge1: Vector3<f64> = v2 - v1;
    let edge2: Vector3<f64> = v3 - v1;
    let ray_origin: Vector3<f64> = Vector3::new(0f64, 0f64, 2f64);
    let renderer = Renderer {
        width,
        height,
        area,
        half_width,
        half_height,
        samples,
        samples_square,
        step_x,
        step_y,
        step_sample_x,
        step_sample_y,
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
    let bytes_count = area * 4;
    let bytes_for_thread = bytes_count as usize / threads_count;
    let mut pixels_threads = Vec::new();
    let mut threads = Vec::new();
    let mut last_height = 0;
    let mut last_y = 1f64;
    let y_step_thread = -2f64 / threads_count as f64;
    for _ in 0..threads_count {
        let pixels = Arc::new(RwLock::new(vec![0u8; bytes_for_thread]));
        pixels_threads.push(pixels.clone());
        let starting_height = last_height;
        last_height += height_per_thread;
        let ending_height = last_height;
        let starting_y = last_y;
        last_y += y_step_thread;
        let renderer = renderer.clone();
        threads.push(thread::spawn(move || {
            let renderer = renderer.read().unwrap();
            let mut pixels = pixels.write().unwrap();
            let mut y = starting_y;
            let mut px_index = 0;
            // println!("starting_height: {}, ending_height: {}, lenpixels: {}", starting_height, ending_height, pixels.len());
            for _ in starting_height..ending_height {
                let mut x = -1f64;
                for _ in 0..width {
                    let mut samples = [0u64; 3];
                    let mut sy = y;
                    for _ in 0..renderer.samples {
                        let mut sx = x;
                        for _ in 0..renderer.samples {
                            let ray_dir = Vector3::new(sx, sy, -2f64);
                            let pvec = ray_dir.cross(edge2);
                            let det = dot(edge1, pvec);
                            if det < 0.0001 && det > -0.0001 {
                                sx += step_sample_x;
                                continue;
                            }
                            let inv_det = 1f64 / det;
                            let tvec = ray_origin - v1;
                            let u = dot(tvec, pvec) * inv_det;
                            if u < 0.0001f64 || u > 0.9999f64 {
                                sx += step_sample_x;
                                continue;
                            }
                            let qvec = tvec.cross(edge1);
                            let v = dot(ray_dir, qvec) * inv_det;
                            if v < 0.0001f64 || u + v > 0.9999f64 {
                                sx += step_sample_x;
                                continue;
                            }
                            let t = dot(edge2, qvec) * inv_det;
                            if t > 0.0001 {
                                samples[0] += 255;
                                samples[1] += 255;
                                samples[2] += 255;
                            }
                            sx += step_sample_x;
                        }
                        sy -= step_sample_y;
                    }
                    pixels[px_index] = (samples[0] / samples_square as u64) as u8;
                    px_index += 1;
                    pixels[px_index] = (samples[1] / samples_square as u64) as u8;
                    px_index += 1;
                    pixels[px_index] = (samples[0] / samples_square as u64) as u8;
                    px_index += 1;
                    pixels[px_index] = 255u8;
                    px_index += 1;
                    x += step_x;
                }
                y -= step_y;
            }
        }));
    }
    let mut pixels = Vec::new();
    for t in threads {
        t.join().unwrap();
    }
    for thread_index in 0..threads_count {
        pixels.append(&mut *pixels_threads[thread_index].write().unwrap());
    }
    image::save_buffer(
        "image.png",
        &pixels,
        width as u32,
        height as u32,
        image::RGBA(8),
    ).unwrap();
}
