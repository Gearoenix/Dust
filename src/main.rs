extern crate gtk;

// use gtk::prelude::*
pub mod ui;
pub mod math;
use math::vector::Vec;

fn main() {
    let v1 = math::vector::Vec3 {
        x: 2.0,
        y: 3.0,
        z: 4.0
    };
    let v2 = math::vector::Vec3 {
        x: -3.0,
        y: -1.0,
        z:  1.0
    };
    let mut v3 = math::vector::Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0
    };
    let sc1 = 45.0;

    // println!("{:?}", ((v1 + v2) * 3.9).dot(v2));
    v3 += &(&v1 + &v2);
    println!("{:?}", &v3 * &sc1);
    println!("{:?}", v3);
    println!("{:?}", sc1);
    println!("{:?}", (&v3).dot(&v1));
    println!("{:?}", v1);
    println!("{:?}", v3);

    let ui_manager = match ui::UiManager::new() {
        Some(u) => u,
        None    => return,
    };
    ui_manager.run();
}
