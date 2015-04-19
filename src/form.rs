//!
//! Ported from [elm-lang's `Graphics.Collage` module]
//! (https://github.com/elm-lang/core/blob/62b22218c42fb8ccc996c86bea450a14991ab815/src/Graphics/Collage.elm).
//!
//!
//! This module is for freeform graphics. You can shift, rotate, scale etc. All sorts of forms
//! including lines, shapes, images and elements.
//!
//! Collages use the same coordinate system you might see in an algebra or physics problem. The
//! origin (0, 0) is at the center of the collage, not the top left corner as in some other
//! graphics libraries. Furthermore, the y-axis points up, so shifting a form 10 units in the
//! y-axis will move it up screen.
//!
//! # Creating Forms
//! to_form, filled, textured, gradient, outlined, traced, text, outlined_text
//!
//! # Transforming Forms
//! shift, shift_x, shift_y, scale, rotate, alpha
//!
//! # Grouping Forms
//! Grouping forms makes it easier to write modular graphics code. You can create a form that is a
//! composite of many subforms. From there it is easy to transform it as a single unit.
//! group, group_transform
//!
//! # Shapes
//! rect, oval, square, circle, ngon, polygon
//!
//! # Paths
//! segment, path
//!
//! # Line Styles
//! solid, dashed, dotted, LineStyle, LineCap, LineJoin
//!


use color::{Color, Gradient};
use element::{self, Element, new_element};
use graphics::{self, DrawState, Graphics};
use graphics::character::CharacterCache;
use std::f64::consts::PI;
use num::Float;
use std::path::PathBuf;
use text::Text;
use transform_2d::{self, Matrix2d, Transform2D};


/// A general, freeform 2D graphics structure.
#[derive(Clone, Debug)]
pub struct Form {
    pub theta: f64,
    pub scale: f64,
    pub x: f64,
    pub y: f64,
    pub alpha: f32,
    pub form: BasicForm,
}


#[derive(Clone, Debug)]
pub enum FillStyle {
    Solid(Color),
    Texture(PathBuf),
    Grad(Gradient),
}


#[derive(Copy, Clone, Debug)]
pub enum LineCap {
    Flat,
    Round,
    Padded,
}


#[derive(Copy, Clone, Debug)]
pub enum LineJoin {
    Smooth,
    Sharp(f64),
    Clipped,
}


#[derive(Clone, Debug)]
pub struct LineStyle {
    pub color: Color,
    pub width: f64,
    pub cap: LineCap,
    pub join: LineJoin,
    pub dashing: Vec<i64>,
    pub dash_offset: i64,
}


impl LineStyle {

    /// The default LineStyle.
    pub fn default() -> LineStyle {
        LineStyle {
            color: ::color::black(),
            width: 1.0,
            cap: LineCap::Flat,
            join: LineJoin::Sharp(10.0),
            dashing: Vec::new(),
            dash_offset: 0,
        }
    }

    /// The LineStyle with some given width.
    #[inline]
    pub fn width(self, w: f64) -> LineStyle {
        LineStyle { width: w, ..self }
    }

}


/// Create a solid line style with a given color.
pub fn solid(color: Color) -> LineStyle {
    LineStyle { color: color, ..LineStyle::default() }
}

/// Create a dashed line style with a given color. Dashing equals `[8, 4]`.
pub fn dashed(color: Color) -> LineStyle {
    LineStyle { color: color, dashing: vec![8, 4], ..LineStyle::default() }
}

/// Create a dotted line style with a given color. Dashing equals `[3, 3]`.
pub fn dotted(color: Color) -> LineStyle {
    LineStyle { color: color, dashing: vec![3, 3], ..LineStyle::default() }
}


/// The basic variants a Form can consist of.
#[derive(Clone, Debug)]
pub enum BasicForm {
    PointPath(LineStyle, PointPath),
    Shape(ShapeStyle, Shape),
    OutlinedText(LineStyle, Text),
    Text(Text),
    Image(i32, i32, (i32, i32), PathBuf),
    Element(Element),
    Group(Transform2D, Vec<Form>),
}


/// Whether a shape is outlined or filled.
#[derive(Clone, Debug)]
pub enum ShapeStyle {
    Line(LineStyle),
    Fill(FillStyle),
}


impl Form {

    fn new(basic_form: BasicForm) -> Form {
        Form {
            theta: 0.0,
            scale: 1.0,
            x: 0.0,
            y: 0.0,
            alpha: 1.0,
            form: basic_form,
        }
    }

    /// Move a form by the given amount. this is a relative translation so `shift(10.0, 10.0, form)
    /// would move `form` ten pixels up and ten pixels to the right.
    #[inline]
    pub fn shift(self, x: f64, y: f64) -> Form {
        Form { x: self.x + x, y: self.y + y, ..self }
    }


    /// Move a shape in the x direction. This is relative so `shift_x(10.0, form)` moves `form` 10
    /// pixels to the right.
    #[inline]
    pub fn shift_x(self, x: f64) -> Form {
        Form { x: self.x + x, ..self }
    }


    /// Move a shape in the y direction. This is relative so `shift_y(10.0, form)` moves `form
    /// upwards by 10 pixels.
    #[inline]
    pub fn shift_y(self, y: f64) -> Form {
        Form { y: self.y + y, ..self }
    }


    /// Scale a form by a given factor. Scaling by 2 doubles both dimensions and quadruples the
    /// area.
    #[inline]
    pub fn scale(self, scale: f64) -> Form {
        Form { scale: self.scale * scale, ..self }
    }


    /// Rotate a form by a given angle. Rotate takes radians and turns things counterclockwise.
    /// So to turn `form` 30 degrees to the left you would say `rotate(degrees(30), form)`.
    #[inline]
    pub fn rotate(self, theta: f64) -> Form {
        Form { theta: self.theta + theta, ..self }
    }


    /// Set the alpha of a Form. The default is 1 and 0 is totally transparent.
    #[inline]
    pub fn alpha(self, alpha: f32) -> Form {
        Form { alpha: alpha, ..self }
    }

}


/// Turn any `Element` into a `Form`. This lets you use text, gifs, and video in your collage. This
/// means you can move, rotate, and scale an `Element` however you want.
pub fn to_form(element: Element) -> Form {
    Form::new(BasicForm::Element(element))
}


/// Flatten many forms into a single `Form`. This lets you move and rotate them as a single unit,
/// making it possible to build small, modular components.
pub fn group(forms: Vec<Form>) -> Form {
    Form::new(BasicForm::Group(transform_2d::identity(), forms))
}


/// Flatten many forms into a single `Form` and then apply a matrix transformation.
pub fn group_transform(matrix: Transform2D, forms: Vec<Form>) -> Form {
    Form::new(BasicForm::Group(matrix, forms))
}


/// Trace a path with a given line style.
pub fn traced(style: LineStyle, path: PointPath) -> Form {
    Form::new(BasicForm::PointPath(style, path))
}


/// Create a line with a given line style.
pub fn line(style: LineStyle, x1: f64, y1: f64, x2: f64, y2: f64) -> Form {
    traced(style, segment((x1, y1), (x2, y2)))
}


/// Create a sprite from a sprite sheet. It cuts out a rectangle at a given position.
pub fn sprite(w: i32, h: i32, pos: (i32, i32), path: PathBuf) -> Form {
    Form::new(BasicForm::Image(w, h, pos, path))
}


/// A collage is a collection of 2D forms. There are no strict positioning relationships between
/// forms, so you are free to do all kinds of 2D graphics.
pub fn collage(w: i32, h: i32, forms: Vec<Form>) -> Element {
    new_element(w, h, element::Prim::Collage(w, h, forms))
}


/// A path described by a sequence of points.
#[derive(Clone, Debug)]
pub struct PointPath(pub Vec<(f64, f64)>);


/// Create a PointPath that follows a sequence of points.
pub fn point_path(points: Vec<(f64, f64)>) -> PointPath {
    PointPath(points)
}


/// Create a PointPath along a given line segment. 
pub fn segment(a: (f64, f64), b: (f64, f64)) -> PointPath {
    PointPath(vec![a, b])
}


/// A shape described by its edges.
#[derive(Clone, Debug)]
pub struct Shape(pub Vec<(f64, f64)>);


impl Shape {

    #[inline]
    fn fill(self, style: FillStyle) -> Form {
        Form::new(BasicForm::Shape(ShapeStyle::Fill(style), self))
    }


    /// Create a filled-in shape.
    #[inline]
    pub fn filled(self, color: Color) -> Form {
        self.fill(FillStyle::Solid(color))
    }


    /// Create a textured shape.
    /// The texture is described by some path and is tiled to fill the entire shape.
    #[inline]
    pub fn textured(self, path: PathBuf) -> Form {
        self.fill(FillStyle::Texture(path))
    }


    /// Fill a shape with a gradient.
    #[inline]
    pub fn gradient(self, grad: Gradient) -> Form {
        self.fill(FillStyle::Grad(grad))
    }


    /// Outline a shape with a given line style.
    #[inline]
    pub fn outlined(self, style: LineStyle) -> Form {
        Form::new(BasicForm::Shape(ShapeStyle::Line(style), self))
    }

}


/// Create an arbitrary polygon by specifying its corners in order. `polygon` will automatically
/// close all shapes, so the given list of points does not need to start and end with the same
/// position.
pub fn polygon(points: Vec<(f64, f64)>) -> Shape {
    Shape(points)
}


/// A rectangle with a given width and height.
pub fn rect(w: f64, h: f64) -> Shape {
    let hw = w / 2.0;
    let hh = h / 2.0;
    Shape(vec![ (0.0-hw, 0.0-hh), (0.0-hw, hh), (hw, hh), (hw, 0.0-hh) ])
}


/// A square with a given edge length.
pub fn square(n: f64) -> Shape {
    rect(n, n)
}


/// An oval with a given width and height.
pub fn oval(w: f64, h: f64) -> Shape {
    let n: usize = 50;
    let t = 2.0 * PI / n as f64;
    let hw = w / 2.0;
    let hh = h / 2.0;
    let f = |i: f64| (hw * (t*i).cos(), hh * (t*i).sin());
    let points = (0..n-1).map(|i| f(i as f64)).collect();
    Shape(points)
}


/// A circle with a given radius.
pub fn circle(r: f64) -> Shape {
    let d = 2.0 * r;
    oval(d, d)
}


/// A regular polygon with N sides. The first argument specifies the number of sides and the second
/// is the radius. So to create a pentagon with radius 30, you would say `ngon(5, 30.0)`
pub fn ngon(n: usize, r: f64) -> Shape {
    let t = 2.0 * PI / n as f64;
    let f = |i: f64| (r * (t*i).cos(), r * (t*i).sin());
    let points = (0..n).map(|i| f(i as f64)).collect();
    Shape(points)
}


/// Create some text. Details like size and color are part of the `Text` value itself, so you can
/// mix colors and sizes and fonts easily.
pub fn text(t: Text) -> Form {
    Form::new(BasicForm::Text(t))
}









/// 
/// CUSTOM NON-ELM FUNCTIONS.
/// 
/// Normally Elm renders to html and javascript, however the aim of elmesque is to render to GL.
///


/// This function draws a form with some given transform using the generic [Piston graphics]
/// (https://github.com/PistonDevelopers/graphics) backend.
pub fn draw_form<'a, C: CharacterCache, G: Graphics<Texture=C::Texture>>(
    form: Form,
    matrix: Matrix2d,
    backend: &mut G,
    maybe_character_cache: &mut Option<&mut C>,
    draw_state: &DrawState
) {
    let Form { theta, scale, x, y, alpha, form } = form;
    let Transform2D(matrix) = Transform2D(matrix)
        .multiply(transform_2d::translation(x, y))
        .multiply(transform_2d::scale(scale))
        .multiply(transform_2d::rotation(theta));
    match form {

        BasicForm::PointPath(line_style, PointPath(points)) => {
            // NOTE: join, dashing and dash_offset are not yet handled properly.
            let LineStyle { color, width, cap, join, dashing, dash_offset } = line_style;
            let color = convert_color(color, alpha);
            let mut draw_line = |(x1, y1), (x2, y2)| {
                if dashing.is_empty() {
                    let line = match cap {
                        LineCap::Flat => graphics::Line::new(color, width / 2.0),
                        LineCap::Round => graphics::Line::new_round(color, width / 2.0),
                        LineCap::Padded => unimplemented!(),
                    };
                    line.draw([x1, y1, x2, y2], draw_state, matrix, backend);
                } else {
                    unimplemented!();
                }
            };
            for window in points.windows(2) {
                let (a, b) = (window[0], window[1]);
                draw_line(a, b);
            }
        },

        BasicForm::Shape(shape_style, Shape(points)) => {
            match shape_style {
                ShapeStyle::Line(line_style) => {
                    // NOTE: join, dashing and dash_offset are not yet handled properly.
                    let LineStyle { color, width, cap, join, dashing, dash_offset } = line_style;
                    let color = convert_color(color, alpha);
                    let mut draw_line = |(x1, y1), (x2, y2)| {
                        let line = match cap {
                            LineCap::Flat => graphics::Line::new(color, width / 2.0),
                            LineCap::Round => graphics::Line::new_round(color, width / 2.0),
                            LineCap::Padded => unimplemented!(),
                        };
                        line.draw([x1, y1, x2, y2], draw_state, matrix, backend);
                    };
                    for window in points.windows(2) {
                        let (a, b) = (window[0], window[1]);
                        draw_line(a, b);
                    }
                    if points.len() > 2 {
                        draw_line(points[points.len()-1], points[0])
                    }
                },
                ShapeStyle::Fill(fill_style) => match fill_style {
                    FillStyle::Solid(color) => {
                        let color = convert_color(color, alpha);
                        let polygon = graphics::Polygon::new(color);
                        let points: Vec<_> = points.into_iter().map(|(x, y)| [x, y]).collect();
                        polygon.draw(&points[..], draw_state, matrix, backend);
                    },
                    FillStyle::Texture(path) => {
                        unimplemented!();
                    },
                    FillStyle::Grad(gradient) => {
                        unimplemented!();
                    },
                },
            }
        },

        BasicForm::OutlinedText(line_style, text) => {
            unimplemented!();
        },

        BasicForm::Text(text) => {
            let Transform2D(matrix) = Transform2D(matrix).multiply(::transform_2d::scale_y(-1.0));
            if let Some(ref mut character_cache) = *maybe_character_cache {
                use text::Style as TextStyle;
                use text::TextUnit;
                let (total_width, max_height) = text.sequence.iter().fold((0.0, 0.0), |(w, h), unit| {
                    let TextUnit { ref string, ref style } = *unit;
                    let TextStyle { ref typeface, height, color, bold, italic, line, monospace } = *style;
                    let height = height.unwrap_or(16.0);
                    let new_total_width = w + character_cache.width(height as u32, &string);
                    let new_max_height = if height > h { height } else { h };
                    (new_total_width, new_max_height)
                });
                let Transform2D(matrix) = Transform2D(matrix)
                    .multiply(transform_2d::translation(-total_width / 2.0, max_height / 3.0)); // TODO: FIX THIS (3.0)
                for unit in text.sequence.iter() {
                    let TextUnit { ref string, ref style } = *unit;
                    let TextStyle { ref typeface, height, color, bold, italic, line, monospace } = *style;
                    let height = height.unwrap_or(16.0);
                    let color = convert_color(color, alpha);
                    graphics::text::Text::colored(color, height as u32)
                        .draw(&string[..], *character_cache, draw_state, matrix, backend);
                }
            }
        },

        BasicForm::Image(src_x, src_y, (w, h), path) => {
            // let image = graphics::Image {
            //     color: None,
            //     rectangle: None,
            //     source_rectangle: Some([src_x, src_y, w, h]),
            // };
            // let texture: &Texture = ::std::ops::Deref::deref(&texture);
            // image.draw(texture, draw_state, matrix, backend);
            unimplemented!();
        },

        BasicForm::Group(group_transform, forms) => {
            let Transform2D(matrix) = Transform2D(matrix.clone()).multiply(group_transform.clone());
            for form in forms.into_iter() {
                draw_form(form, matrix.clone(), backend, maybe_character_cache, draw_state);
            }
        },

        BasicForm::Element(element) =>
            element::draw_element(element, matrix, backend, maybe_character_cache, draw_state),
    }
}

/// Convert an elmesque color to a piston-graphics color.
fn convert_color(color: Color, alpha: f32) -> [f32; 4] {
    use color::hsl_to_rgb;
    let ((r, g, b), a) = match color {
        Color::Hsla(h, s, l, a) => (hsl_to_rgb(h, s, l), a),
        Color::Rgba(r, g, b, a) => ((r, g, b), a),
    };
    [r, g, b, a * alpha]
}

