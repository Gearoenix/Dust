use ::math::vector::{
    Vec3,
};
use ::render::scene::Scene;

pub struct Renderer {
    samples_count_sqrt: f64,
}

pub fn render_scene(samples_count_sqrt: &u32, scene: &scene, screen_width: &u32, screen_height: &u32) -> [Vec3<f64>] {
    let mut screen = [Vec3<f64>; screen_width * screen_height];
    screen[0] = Vec3::new();
    let camera = scene.cameras[scene.active_camera_index];
    {
        let mut screen_index = 0u32;
        let mut pixel_sample_count = 0u32;
        let x_step = 1.0f64 / (samples_count_sqrt * screen_width);
        let y_step = 1.0f64 / (samples_count_sqrt * screen_height);
        let mut y = -1.0;
        loop {
            if y > 1.0 {
                break;
            }
            let mut x = -1.0;
            loop {
                if x > 1.0 {
                    break;
                }
                let ray = camera.get_ray(x, y);
                screen[screen_index] += scene.trace_ray(&r, 5u32);
                if pixel_sample_count > *samples_count_sqrt {
                    pixel_sample_count = 0;
                    screen_index++;
                    screen[screen_index] = Vec3::new();
                }
                pixel_sample_count++;
                x += x_step;
            }
            y += y_step;
        }
    }
    screen
}
