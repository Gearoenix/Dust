pub mod io;
pub mod materials;
pub mod math;
pub mod render;
pub mod texture;
pub mod ui;

use math::vector::MathVector;

fn main() {
    let v1 = math::vector::Vec3 {
        x: 2.0f64,
        y: 3.0f64,
        z: 4.0f64,
    };
    let v2 = math::vector::Vec3 {
        x: -3.0f64,
        y: -1.0f64,
        z:  1.0f64
    };
    let mut v3 = math::vector::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let sc1 = 45.0f64;
    let ry1 = math::ray::Ray3 {
        o:    v1.clone(),
        d:    v2.clone(),
        invd: v3.clone()
    };
    // println!("{:?}", ((v1 + v2) * 3.9).dot(v2));
    v3 += v1 + v2;

    println!("{:?}", v3 * sc1);
    println!("{:?}", v3);
    println!("{:?}", sc1);
    println!("{:?}", v3.dot(&v1));
    println!("{:?}", v1);
    println!("{:?}", v3);
    println!("{:?}", ry1);

    let mut file = io::file::Stream::new(&("1.gx3d".to_string()));

    let mut textures_manager = texture::textures_manager::TexturesManager::new();
    let mut scenes_manager = render::scenes_manager::ScenesManager::new();
    textures_manager.read_table(&mut file);

    let ui_manager = match ui::UiManager::new() {
        Some(u) => u,
        None    => return,
    };
    ui_manager.run();
}
