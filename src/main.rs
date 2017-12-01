// pub mod io;
// pub mod math;
// pub mod render;
// pub mod texture;
// pub mod ui;

// fn main() {
//     let v1 = math::vector::Vec3 {
//         x: 2.0f64,
//         y: 3.0f64,
//         z: 4.0f64,
//     };
//     let v2 = math::vector::Vec3 {
//         x: -3.0f64,
//         y: -1.0f64,
//         z:  1.0f64
//     };
//     let mut v3 = math::vector::Vec3 {
//         x: 0.0,
//         y: 0.0,
//         z: 0.0,
//     };
//     let sc1 = 45.0f64;
//     let ry1 = math::ray::Ray3 {
//         o:    v1.clone(),
//         d:    v2.clone(),
//         invd: v3.clone()
//     };
//     // println!("{:?}", ((v1 + v2) * 3.9).dot(v2));
//     v3 += v1 + v2;

//     println!("{:?}", v3 * sc1);
//     println!("{:?}", v3);
//     println!("{:?}", sc1);
//     println!("{:?}", v3.dot(&v1));
//     println!("{:?}", v1);
//     println!("{:?}", v3);
//     println!("{:?}", ry1);

//     let mut file = io::file::Stream::new(&("/home/thany/Temporary/1.gx3d".to_string()));

//     let mut textures_manager = texture::textures_manager::TexturesManager::new();
//     let mut scenes_manager = render::scenes_manager::ScenesManager::new();
//     textures_manager.read_table(&mut file);
//     scenes_manager.read_table(&mut file);
//     let mut scene = scenes_manager.get_scene(&"Scene".to_string(), &mut file);

//     let ui_manager = match ui::UiManager::new() {
//         Some(u) => u,
//         None    => return,
//     };
//     ui_manager.run();
// }
//! This example behaves the same as the `all_winit_glium` example while demonstrating how to run
//! the `conrod` loop on a separate thread.

#[macro_use]
extern crate conrod;
extern crate glium;
extern crate winit;
mod support {
    //! This module is used for sharing a few items between the `all_widgets.rs`, `glutin_glium.rs`
    //! and glutin_gfx.rs` examples.
    //!
    //! The module contains:
    //!
    //! - `pub struct DemoApp` as a demonstration of some state we want to change.
    //! - `pub fn gui` as a demonstration of all widgets, some of which mutate our `DemoApp`.
    //! - `pub struct Ids` - a set of all `widget::Id`s used in the `gui` fn.
    //!
    //! By sharing these items between these examples, we can test and ensure that the different
    //! events and drawing backends behave in the same manner.

    extern crate rand;

    use std;

    use conrod::backend::glium::glium;


    /// In most of the examples the `glutin` crate is used for providing the window context and
    /// events while the `glium` crate is used for displaying `conrod::render::Primitives` to the
    /// screen.
    ///
    /// This `Iterator`-like type simplifies some of the boilerplate involved in setting up a
    /// glutin+glium event loop that works efficiently with conrod.

    pub struct EventLoop {
        ui_needs_update: bool,
        last_update: std::time::Instant,
    }

    impl EventLoop {
        pub fn new() -> Self {
            EventLoop {
                last_update: std::time::Instant::now(),
                ui_needs_update: true,
            }
        }

        /// Produce an iterator yielding all available events.
        pub fn next(
            &mut self,
            events_loop: &mut glium::glutin::EventsLoop,
        ) -> Vec<glium::glutin::Event> {
            // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
            // since the last yield.
            let last_update = self.last_update;
            let sixteen_ms = std::time::Duration::from_millis(16);
            let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
            if duration_since_last_update < sixteen_ms {
                std::thread::sleep(sixteen_ms - duration_since_last_update);
            }

            // Collect all pending events.
            let mut events = Vec::new();
            events_loop.poll_events(|event| events.push(event));

            // If there are no events and the `Ui` does not need updating, wait for the next event.
            if events.is_empty() && !self.ui_needs_update {
                events_loop.run_forever(|event| {
                    events.push(event);
                    glium::glutin::ControlFlow::Break
                });
            }

            self.ui_needs_update = false;
            self.last_update = std::time::Instant::now();

            events
        }
    }

}

extern crate find_folder;
extern crate image;
use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};
use conrod::backend::glium::glium::Surface;

pub fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Image Widget Demonstration")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The `WidgetId` for our background and `Image` widgets.
    widget_ids!(struct Ids { background, rust_logo });
    let ids = Ids::new(ui.widget_id_generator());

    // Create our `conrod::image::Map` which describes each of our widget->image mappings.
    // In our case we only have one image, however the macro may be used to list multiple.
    let rust_logo = load_rust_logo(&display);
    let (w, h) = (rust_logo.get_width(), rust_logo.get_height().unwrap());
    let mut image_map = conrod::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {
        // Handle all events.
        for event in event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                ui.handle_event(event);
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::Closed
                    | glium::glutin::WindowEvent::KeyboardInput {
                        input:
                            glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // Instantiate the widgets.
        {
            let ui = &mut ui.set_widgets();
            // Draw a light blue background.
            widget::Canvas::new()
                .color(color::DARK_CHARCOAL)
                .set(ids.background, ui);
            // Instantiate the `Image` at its full size in the middle of the window.
            widget::Image::new(rust_logo)
                .w_h(w as f64, h as f64)
                .middle()
                .set(ids.rust_logo, ui);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn load_rust_logo(display: &glium::Display) -> glium::texture::Texture2d {
    let image_dimensions = (128, 128);
    let mut image_data = [255u8; 128 * 128 * 4];
    for i in 0..128 {
        for j in 0..128 {
            if i > j {
                image_data[(i * 128 + j) * 4 + 2] = 0;
                image_data[(i * 128 + j) * 4 + 1] = 0;
            } else {
                image_data[(i * 128 + j) * 4] = 0;
                image_data[(i * 128 + j) * 4 + 1] = 0;
            }
        }
    }
    let raw_image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image_data, image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}
