extern crate gtk;

use gtk::prelude::*;

pub struct MainWindow {
    win : gtk::Window,
}

impl MainWindow {
    pub fn new() -> MainWindow {
        let window = gtk::Window::new(gtk::WindowType::Toplevel);
        window.set_title("Dust, Gearonix software");
        window.set_border_width(10);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(350, 70);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        MainWindow { win: window }
        // let button = gtk::Button::new_with_label("Click me!");
        // window.add(&button);
    }

    pub fn show(&self) {
        self.win.show_all();
    }
}
