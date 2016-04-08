extern crate gtk;

// use gtk::prelude::*
pub mod ui;
pub mod math;

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

    println!("{:?}", (v1 + v2) * 3.9);

    let ui_manager = match ui::UiManager::new() {
        Some(u) => u,
        None    => return,
    };
    ui_manager.run();
}
