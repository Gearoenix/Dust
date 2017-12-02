#[macro_use]
extern crate conrod;
extern crate find_folder;
extern crate image;
extern crate num_cpus;
extern crate rand;
extern crate winit;

pub mod math;
pub mod render;

use render::camera::{Base,PerspectiveCamera};
use render::engine::CpuEngine;
use render::engine::Data as EngineData;
use render::vertex::Vertex;
use math::vector::{Vec2, Vec3};
use math::triangle::Triangle;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;
use conrod::{color, widget, Colorable, Positionable, Sizeable, Widget};

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

pub fn main() {
    const WIDTH: u32 = 1300;
    const HEIGHT: u32 = 900;

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
    println!("number of cpu {:?}", num_cpus::get());
    // The `WidgetId` for our background and `Image` widgets.
    widget_ids!(struct Ids { background, rust_logo });
    let ids = Ids::new(ui.widget_id_generator());
    let vertices = vec![
        Vertex {
            ps: Vec3 {
                x: -1.0,
                y: -1.0,
                z: 0.0,
            },
            nr: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            uv: Vec2 { x: 0.0, y: 0.0 },
        },
        Vertex {
            ps: Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            nr: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            uv: Vec2 { x: 0.0, y: 0.0 },
        },
        Vertex {
            ps: Vec3 {
                x: 1.0,
                y: -1.0,
                z: 0.0,
            },
            nr: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            uv: Vec2 { x: 0.0, y: 0.0 },
        },
    ];
    let triangles = vec![Triangle::new(&[0, 1, 2], &vertices)];
    let data = EngineData {
        view_port_dimension: (WIDTH - 100, HEIGHT - 100),
        triangles: triangles,
        vertices: vertices,
        cameras: vec![Box::new(PerspectiveCamera::new(Base::new(
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            1.7
        )))],
    };
    // let ray = data.triangles[0].intersect(&data.cameras[0].get_ray(0.0, 0.0), 9000.0, &data.vertices);
    // panic!("{:?}", ray);
    let engine = CpuEngine::new(data);
    let rust_logo = load_rust_logo(&display, &engine);
    let (w, h) = (rust_logo.get_width(), rust_logo.get_height().unwrap());
    println!("{:?} {:?}", w, h);
    let mut image_map = conrod::image::Map::new();
    let rust_logo = image_map.insert(rust_logo);

    // Poll events from the window.
    let mut event_loop = EventLoop::new();
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

fn load_rust_logo(display: &glium::Display, engine: &CpuEngine) -> glium::texture::Texture2d {
    let image_dimensions = engine.data.read().unwrap().view_port_dimension;
    let image_data = engine.render();
    let raw_image = glium::texture::RawImage2d::from_raw_rgba(image_data, image_dimensions);
    let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();
    texture
}
