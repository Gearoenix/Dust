
extern crate cairo;
extern crate gtk;

use std::f64::consts::PI;

use ::gtk::prelude::*;
use ::gtk::DrawingArea;

use self::cairo::Context;

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

        let drawing_area = Box::new(DrawingArea::new)();

        drawing_area.connect_draw( |_, cr| {
            cr.set_dash(&[3., 2., 1.], 1.);
            assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

            cr.scale(500f64, 500f64);

            cr.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
            cr.paint();

            cr.set_line_width(0.05);

            // border
            cr.set_source_rgb(0.3, 0.3, 0.3);
            cr.rectangle(0.0, 0.0, 1.0, 1.0);
            cr.stroke();

            cr.set_line_width(0.03);

            // draw circle
            cr.arc(0.5, 0.5, 0.4, 0.0, PI * 2.);
            cr.stroke();


            // mouth
            let mouth_top = 0.68;
            let mouth_width = 0.38;

            let mouth_dx = 0.10;
            let mouth_dy = 0.10;

            cr.move_to( 0.50 - mouth_width/2.0, mouth_top);
            cr.curve_to(0.50 - mouth_dx,        mouth_top + mouth_dy,
                         0.50 + mouth_dx,        mouth_top + mouth_dy,
                         0.50 + mouth_width/2.0, mouth_top);

            println!("Extents: {:?}", cr.fill_extents());

            cr.stroke();

            let eye_y = 0.38;
            let eye_dx = 0.15;
            cr.arc(0.5 - eye_dx, eye_y, 0.05, 0.0, PI * 2.);
            cr.fill();

            cr.arc(0.5 + eye_dx, eye_y, 0.05, 0.0, PI * 2.);
            cr.fill();

            Inhibit(false)
        });
        window.add(&drawing_area);

        MainWindow { win: window }
    }

    pub fn show(&self) {
        self.win.show_all();
    }
}
