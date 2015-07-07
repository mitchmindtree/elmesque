extern crate elmesque;
extern crate piston;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate num;

use elmesque::{Form, Renderer};
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::{UpdateEvent, Events, EventLoop, RenderEvent};
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

fn main() {

    // Construct the window.
    let window: Window =
        WindowSettings::new("Elmesque", [1180, 580])
            .exit_on_esc(true)
            .vsync(true)
            .samples(4)
            .into();

    let mut gl = GlGraphics::new(OpenGL::_3_2);

    // Construct the GlyphCache.
    let mut glyph_cache = {
        GlyphCache::from_bytes(include_bytes!("../assets/NotoSans/NotoSans-Regular.ttf")).unwrap()
    };

    // We'll use this to animate our graphics.
    let mut secs = 0.0;

    // Poll events from the window.
    for event in window.events().max_fps(60).ups(60) {
        if let Some(r) = event.render_args() {
            gl.draw(r.viewport(), |context, g| {
                let view_dim = context.get_view_size();
                let (w, h) = (view_dim[0], view_dim[1]);

                // Construct the elmesque Renderer with our graphics backend and glyph cache.
                let mut renderer = Renderer::new(context, g).character_cache(&mut glyph_cache);

                // Construct some freeform graphics aka a `Form`.
                let form = elmesque_demo_form(secs);

                // Convert the form to an `Element` for rendering.
                let a = elmesque::form::collage(w as i32, h as i32, vec![form])
                    .clear(elmesque::color::black());

                a.draw(&mut renderer);
            });
        }
        if let Some(u) = event.update_args() {
            secs += u.dt
        }
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
