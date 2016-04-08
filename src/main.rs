extern crate gtk;

// use gtk::prelude::*
pub mod ui;

fn main() {
    let ui_manager = match ui::UiManager::new() {
        Some(u) => u,
        None    => return,
    };
    ui_manager.run();
}
