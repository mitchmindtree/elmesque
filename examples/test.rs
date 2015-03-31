#![feature(slice_patterns)]

extern crate elmesque;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate glutin_window;
extern crate graphics;
extern crate shader_version;
extern crate piston;

use elmesque::form::Form;
use gfx_device_gl::GlDevice;
use gfx_graphics::{gfx, G2D};
use gfx_graphics::gfx::traits::*;
use glutin_window::GlutinWindow;
use std::cell::RefCell;
use std::rc::Rc;
use piston::event::Event;
use piston::events::Events;
use piston::window::{Window, WindowSettings};

fn main() {

    let window = GlutinWindow::new(
        shader_version::opengl::OpenGL::_3_2,
        WindowSettings {
            title: "Elmesque".to_string(),
            size: [1180, 580],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    let mut device = GlDevice::new(|s| window.window.get_proc_address(s));
    let mut g2d = G2D::new(&mut device);



    let mut renderer = device.create_renderer();
    let [w, h] = window.size();
    let frame = gfx::Frame::new(w as u16, h as u16);

    let window_ref = Rc::new(RefCell::new(window));
    let event_iter = Events::new(window_ref).ups(180).max_fps(60);

    for event in event_iter {
        if let Event::Render(args) = event {
            g2d.draw(&mut renderer, &frame, |mut context, graphics| {
                use graphics::Transformed;
                graphics::clear([0.0, 0.0, 0.0, 0.5], graphics);
                let context = context.trans(args.width as f64 / 2.0, args.height as f64 / 2.0);

                elmesque_form().draw(context, graphics)

            });
            device.submit(renderer.as_buffer());
            renderer.reset();
        }
    }

}


pub fn elmesque_form() -> Form {
    use elmesque::color::{blue, light_purple};
    use elmesque::form::{group, oval, rect, solid};
    use elmesque::utils::{degrees};
    group(vec![
        rect(60.0, 40.0).filled(blue()).shift_x(50.0).rotate(degrees(45.0)),
        oval(40.0, 60.0).outlined(solid(light_purple())).shift(-50.0, 50.0)
    ])
}
