extern crate elmesque;
extern crate glutin_window;
extern crate glium_graphics;
extern crate graphics;
extern crate num;
extern crate shader_version;
extern crate piston;

use elmesque::{Form, Renderer};
use graphics::context::Context;
use glium_graphics::{GliumWindow, GlyphCache, Glium2d, GliumGraphics};
use glutin_window::{GlutinWindow, OpenGL};
use piston::event::{UpdateEvent, Events, RenderEvent};
use piston::window::{Size, WindowSettings};
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;


fn main() {
    const GLVERSION: OpenGL = OpenGL::_3_2;
    // Construct the window.
    let window = Rc::new(RefCell::new(GlutinWindow::new(
        GLVERSION,
        WindowSettings::new("Elmesque".to_string(), Size { width: 1180, height: 580 })
            .exit_on_esc(true)
            .samples(4)
    )));

    // Construct the GlyphCache.
    let glium_window = GliumWindow::new(&window).unwrap();
    let mut g2d = Glium2d::new(GLVERSION, &glium_window);
    let mut glyph_cache = GlyphCache::new(
        Path::new("assets/NotoSans/NotoSans-Regular.ttf"),
        glium_window.clone()
    ).unwrap();

    // We'll use this to animate our graphics.
    let mut secs = 0.0;

    // Poll events from the window.
    for event in window.events() {
        event.render(|args| {
            let mut target = glium_window.draw();
            {
                // Construct the elmesque Renderer with our graphics backend and glyph cache.
                let mut backend = GliumGraphics::new(&mut g2d, &mut target);
                let ctx = Context::abs(args.width as f64, args.height as f64);
                let mut renderer = Renderer::new(ctx, &mut backend)
                    .character_cache(&mut glyph_cache);

                // Construct some freeform graphics aka a `Form`.
                let form = elmesque_demo_form(secs);

                // Convert the form to an `Element` for rendering.
                elmesque::form::collage(args.width as i32, args.height as i32, vec![form])
                    //.crop((secs / 2.0).sin() * (w / 2.0), (secs / 3.0).sin() * (h / 2.0), 400.0, 400.0)
                    .clear(elmesque::color::black())
                    .draw(&mut renderer);
            }
            target.finish();
        });
        event.update(|args| secs += args.dt);
    }
}


/// Demo of grouping multiple forms into a new single form, transformable at any stage.
pub fn elmesque_demo_form(secs: f64) -> Form {
    use elmesque::color::{blue, dark_blue, light_blue, dark_purple, white};
    use elmesque::form::{circle, group, ngon, oval, point_path, rect, solid, text, traced};
    use elmesque::text::Text;
    use elmesque::utils::{degrees};
    use num::Float;

    // Time to get creative!
    group(vec![

        rect(60.0, 40.0).filled(blue())
            .shift(secs.sin() * 50.0, secs.cos() * 50.0)
            .alpha(((secs * 200.0).cos() * 0.5 + 0.5) as f32)
            .rotate(-secs),

        rect(100.0, 10.0).filled(dark_blue())
            .shift((secs * 5.0).sin() * 200.0, (secs * 5.0).cos() * 200.0)
            .alpha(((secs * 2.0).cos() * 0.5 + 0.5) as f32)
            .rotate(-(secs * 5.0)),

        rect(10.0, 300.0).filled(blue())
            .alpha(((secs * 3.0).sin() * 0.25 + 0.75) as f32)
            .rotate(-(secs * 1.5)),

        rect(5.0, (secs * 0.1).sin() * 600.0 + 300.0).filled(light_blue())
            .alpha(((secs).cos() * 0.25 + 0.75) as f32)
            .rotate(secs * 0.75),

        rect(3.0, 2000.0).filled(dark_blue())
            .alpha(((secs * 100.0).cos() * 0.5 + 0.25) as f32)
            .rotate(-(secs * 0.5)),

        oval(3.0, 2000.0 * (secs * 60.0).sin()).filled(light_blue())
            .alpha(((secs * 100.0).cos() * 0.5 + 0.25) as f32)
            .rotate(-(secs * 0.6)),

        rect(10.0, 750.0).filled(blue())
            .alpha(((secs * 2.0).cos() * 0.5 + 0.25) as f32)
            .rotate(-(secs * 1.85)),

        circle((secs * 0.5).sin() * 1500.0).outlined(solid(dark_purple()))
            .alpha(((secs * 0.2).sin() * 0.25 + 0.35) as f32)
            .rotate(-(secs * 0.5)),

        ngon(12, (secs * 0.1).cos() * 100.0 + 300.0).filled(blue())
            .alpha((0.25 * secs.cos()) as f32)
            .rotate(secs * 0.5),

        ngon(9, (secs * 0.1).cos() * 200.0 + 250.0).outlined(solid(dark_blue()))
            .alpha(((0.33 * secs).sin() + 0.15) as f32)
            .rotate(secs * 0.2),

        rect(300.0, 20.0).filled(light_blue())
            .shift((secs * 1.5).cos() * 250.0, (secs * 1.5).sin() * 250.0)
            .alpha(((secs * 4.5).cos() * 0.25 + 0.35) as f32)
            .rotate(secs * 1.5 + degrees(90.0)),

        traced(
            solid(light_blue()),
            point_path(vec![(-500.0, 100.0), (0.0, 250.0 * secs.sin()), (500.0, 100.0)])
        ).alpha(((secs * 0.2).sin() * 0.25 + 0.35) as f32),
            
        traced(
            solid(blue()),
            point_path(vec![(-500.0, 0.0), (0.0, 0.0), (500.0, 0.0)])
        ).alpha(((secs * 4.5).cos() * 0.25 + 0.35) as f32),

        traced(
            solid(dark_blue()),
            point_path(vec![(-500.0, -100.0), (0.0, -250.0 * secs.sin()), (500.0, -100.0)])
        ).alpha(((secs * 0.15).cos() * 0.25 + 0.35) as f32),

        text(Text::from_string("elmesque".to_string()).color(white())),

    ]).rotate(degrees(secs.sin() * 360.0))
      .scale((secs * 0.05).cos() * 0.2 + 0.9)

}

