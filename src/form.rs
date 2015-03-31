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


use color::{black, Color, Gradient};
use std::f64::consts::PI;
use std::num::Float;
use std::path::PathBuf;
use text::Text;
use transform_2d::{self, Transform2D};


/// A general, freeform 2D graphics structure.
#[derive(Clone, Debug)]
pub struct Form {
    pub theta: f64,
    pub scale: f64,
    pub x: f64,
    pub y: f64,
    pub alpha: f64,
    pub form: BasicForm,
}


#[derive(Clone, Debug)]
pub enum FillStyle {
    Solid(Color),
    Texture(String),
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
    pub fn default() -> LineStyle {
        LineStyle {
            color: black(),
            width: 1.0,
            cap: LineCap::Flat,
            join: LineJoin::Sharp(10.0),
            dashing: Vec::new(),
            dash_offset: 0,
        }
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
    Image(i64, i64, (i64, i64), PathBuf),
    //Element(Element),
    Group(Transform2D, Vec<Form>),
}


/// Whether a shape is outlined or filled.
#[derive(Clone, Debug)]
pub enum ShapeStyle {
    Line(LineStyle),
    Fill(FillStyle),
}


fn form(basic_form: BasicForm) -> Form {
    Form {
        theta: 0.0,
        scale: 1.0,
        x: 0.0,
        y: 0.0,
        alpha: 1.0,
        form: basic_form,
    }
}


fn fill(style: FillStyle, shape: Shape) -> Form {
    form(BasicForm::Shape(ShapeStyle::Fill(style), shape))
}


/// Create a filled-in shape.
pub fn filled(color: Color, shape: Shape) -> Form {
    fill(FillStyle::Solid(color), shape)
}


/// Create a textured shape.
/// The texture is described by some path and is tiled to fill the entire shape.
pub fn textured(src: String, shape: Shape) -> Form {
    fill(FillStyle::Texture(src), shape)
}


/// Fill a shape with a gradient.
pub fn gradient(grad: Gradient, shape: Shape) -> Form {
    fill(FillStyle::Grad(grad), shape)
}


/// Outline a shape with a given line style.
pub fn outlined(style: LineStyle, shape: Shape) -> Form {
    form(BasicForm::Shape(ShapeStyle::Line(style), shape))
}


/// Trace a path with a given line style.
pub fn traced(style: LineStyle, path: PointPath) -> Form {
    form(BasicForm::PointPath(style, path))
}


/// Create a sprite from a sprite sheet. It cuts out a rectangle at a given position.
pub fn sprite(w: i64, h: i64, pos: (i64, i64), src: PathBuf) -> Form {
    form(BasicForm::Image(w, h, pos, src))
}


// /// Turn any `Element` into a `Form`. This lets you use text, gifs, and video in your collage. This
// /// means you can move, rotate, and scale an `Element` however you want.
// pub fn to_form(element: Element) -> Form {
//     form(BasicForm::Element(element))
// }


/// Flatten many forms into a single `Form`. This lets you move and rotate them as a single unit,
/// making it possible to build small, modular components.
pub fn group(forms: Vec<Form>) -> Form {
    form(BasicForm::Group(transform_2d::identity(), forms))
}


/// Flatten many forms into a single `Form` and then apply a matrix transformation.
pub fn group_transform(matrix: Transform2D, forms: Vec<Form>) -> Form {
    form(BasicForm::Group(matrix, forms))
}


/// Move a form by the given amount. this is a relative translation so `shift(10.0, 10.0, form) would
/// move `form` ten pixels up and ten pixels to the right.
pub fn shift(x: f64, y: f64, form: Form) -> Form {
    Form { x: form.x + x, y: form.y + y, ..form }
}


/// Move a shape in the x direction. This is relative so `shift_x(10.0, form)` moves `form` 10 pixels
/// to the right.
pub fn shift_x(x: f64, form: Form) -> Form {
    Form { x: form.x + x, ..form }
}


/// Move a shape in the y direction. This is relative so `shift_y(10.0, form)` moves `form upwards
/// by 10 pixels.
pub fn shift_y(y: f64, form: Form) -> Form {
    Form { y: form.y + y, ..form }
}


/// Scale a form by a given factor. Scaling by 2 doubles both dimensions and quadruples the area.
pub fn scale(scale: f64, form: Form) -> Form {
    Form { scale: form.scale * scale, ..form }
}


/// Rotate a form by a given angle. Rotate takes radians and turns things counterclockwise. So to
/// turn `form` 30 degrees to the left you would say `rotate(degrees(30), form)`.
pub fn rotate(theta: f64, form: Form) -> Form {
    Form { theta: form.theta + theta, ..form }
}


/// Set the alpha of a Form. The default is 1 and 0 is totally transparent.
pub fn alpha(alpha: f64, form: Form) -> Form {
    Form { alpha: alpha, ..form }
}


// /// A collage is a collection of 2D forms. There are no strict positioning relationships between
// /// forms, so you are free to do all kinds of 2D graphics.
// pub fn collage(x: i64, y: i64, forms: Vec<Form>) -> Element {
//     unimplemented!()
// }


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
    let points = (0..n-1).map(|i| f(i as f64)).collect();
    Shape(points)
}


/// Create some text. Details like size and color are part of the `Text` value itself, so you can
/// mix colors and sizes and fonts easily.
pub fn text(t: Text) -> Form {
    form(BasicForm::Text(t))
}


