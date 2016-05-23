use ::math::vector::{
    Vec3,
};
use ::render::scene::Scene;

pub struct Renderer {
    samples_count_sqrt: f64,
}

pub fn render_scene(samples_count_sqrt: &f64, scene: &scene, screen_width: &u32, screen_height: &u32) -> [[Vec3<f64>]] {
    let mut screen = [[Vec3<f64>; screen_width]; screen_height];
    screen
}
