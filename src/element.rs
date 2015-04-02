//! 
//! Ported from [elm-lang's `Graphics.Element` module]
//! (https://github.com/elm-lang/core/blob/1.1.1/src/Graphics/Element.elm)
//!
//!
//! Graphical elements that snap together to build complex widgets and layouts.
//!
//! Each element is a rectangle with a known width and height, making them easy to combine and
//! position.
//!
//!
//! # Images
//!
//!   image, fitted_image, cropped_image, tiled_image
//!
//!
//! # Styling
//!
//!   width, height, size, color, opacity
//!
//!
//! # Inspection
//!
//!   width_of, height_of, size_of
//!
//!
//! # Layout
//!
//!   flow, up, down, left, right, inward, outward
//!
//! ## Layout Aliases
//!
//! There are some convenience functions for working with `flow` in specific cases:
//!
//!   layers, above, below, beside
//!
//!
//! # Positioning
//!   empty, spacer, container
//!
//! ## Specific Positions
//!
//! To create a `Position` you can use any of the built-in positions which cover nine common
//! positions:
//!
//!   middle, mid_top, mid_bottom, mid_left, mid_right, top_left, top_right, bottom_left,
//!   bottom_right
//!
//! If you need more precision, you can create custom positions:
//!
//!   absolute, relative, middle_at, mid_top_at, mid_bottom_at, mid_left_at, mid_right_at,
//!   top_left_at, top_right_at, bottom_left_at, bottom_right_at
//!

use color::Color;
use form::Form;
use self::Three::{P, Z, N};
use std::rc::Rc;
use Texture;


/// The global graphics unique identifier counter.
pub type Guid = u64;
pub static mut GUID: Guid = 0;


/// Increment the GUID and return the new GUID.
fn guid() -> Guid {
    unsafe {
        GUID += 1;
        GUID
    }
}


/// An Element's Properties.
#[derive(Clone, Debug)]
pub struct Properties {
    id: Guid,
    width: i32,
    height: i32,
    opacity: f32,
    color: Option<Color>,
}


/// Graphical elements that snap together to build complex widgets and layouts.
///
/// Each element is a rectangle with a known width and height, making them easy to combine and
/// position.
#[derive(Clone, Debug)]
pub struct Element {
    props: Properties,
    element: Prim,
}


impl Element {

    /// Create an `Element` with a given width.
    #[inline]
    pub fn width(self, new_width: i32) -> Element {
        let Element { props, element } = self;
        let new_props = match element {
            Prim::Image(_, w, h, _) | Prim::Collage(w, h, _) => {
                Properties {
                    height: (h as f32 / w as f32 * new_width as f32).round() as i32,
                    ..props
                }
            },
            _ => props,
        };
        Element { props: new_props, element: element }
    }

    /// Create an `Element` with a given height.
    #[inline]
    pub fn height(self, new_height: i32) -> Element {
        let Element { props, element } = self;
        let new_props = match element {
            Prim::Image(_, w, h, _) | Prim::Collage(w, h, _) => {
                Properties {
                    width: (w as f32 / h as f32 * new_height as f32).round() as i32,
                    ..props
                }
            },
            _ => props,
        };
        Element { props: new_props, element: element }
    }

    /// Create an `Element` with a given size.
    #[inline]
    pub fn size(self, new_w: i32, new_h: i32) -> Element {
        self.height(new_h).width(new_w)
    }

    /// Create an `Element` with a given opacity.
    #[inline]
    pub fn opacity(mut self, opacity: f32) -> Element {
        self.props.opacity = opacity;
        self
    }

    /// Create an `Element with a given background color.
    #[inline]
    pub fn color(mut self, color: Color) -> Element {
        self.props.color = Some(color);
        self
    }

    /// Put an element in a container. This lets you position the element really easily, and there are
    /// tons of ways to set the `Position`.
    #[inline]
    pub fn container(self, w: i32, h: i32, pos: Position) -> Element {
        new_element(w, h, Prim::Container(pos, box self))
    }

    /// Stack elements vertically. To put `a` above `b` you would say: `a.above(b)`
    #[inline]
    pub fn above(self, other: Element) -> Element {
        new_element(::std::cmp::max(width_of(&self), width_of(&other)),
                    height_of(&self) + height_of(&other),
                    Prim::Flow(down(), vec![self, other]))
    }

    /// Stack elements vertically. To put `a` below `b` you would say: `a.below(b)`
    #[inline]
    pub fn below(self, other: Element) -> Element {
        other.above(self)
    }

    /// Put elements beside each other horizontally. To put `b` to the right of `a` you would say:
    ///   `a.beside(b)`
    #[inline]
    pub fn beside(self, other: Element) -> Element {
        new_element(width_of(&self) + width_of(&other),
                    ::std::cmp::max(height_of(&self), height_of(&other)),
                    Prim::Flow(right(), vec![self, other]))
    }

}


/// Return the width of the Element.
pub fn width_of(e: &Element) -> i32 {
    e.props.width
}

/// Return the height of the Element.
pub fn height_of(e: &Element) -> i32 {
    e.props.height
}

/// Return the size of the Element.
pub fn size_of(e: &Element) -> (i32, i32) {
    (e.props.width, e.props.height)
}


/// Construct a new Element from width, height and some Prim.
/// Iterates the global GUID counter by one and returns that as the Element id.
#[inline]
pub fn new_element(w: i32, h: i32, element: Prim) -> Element {
    Element {
        props: Properties {
            id: guid(),
            width: w,
            height: h,
            opacity: 1.0,
            color: None,
        },
        element: element,
    }
}


/// Create an empty box. this is useful for getting your spacing right and making borders.
pub fn spacer(w: i32, h: i32) -> Element {
    new_element(w, h, Prim::Spacer)
}

/// An Element that takes up no space. Good for things that appear conditionally.
pub fn empty() -> Element {
    spacer(0, 0)
}


/// The various kinds of Elements.
#[derive(Clone, Debug)]
pub enum Prim {
    Image(ImageStyle, i32, i32, Rc<Texture>),
    Container(Position, Box<Element>),
    Flow(Direction, Vec<Element>),
    Collage(i32, i32, Vec<Form>),
    Spacer,
}


/// Styling for the Image Element.
#[derive(Copy, Clone, Debug)]
pub enum ImageStyle {
    Plain,
    Fitted,
    Cropped(i32, i32),
    Tiled,
}


/// Create an image given a width, height and texture.
pub fn image(w: i32, h: i32, texture: Rc<Texture>) -> Element {
    new_element(w, h, Prim::Image(ImageStyle::Plain, w, h, texture))
}

/// Create a fitted image given a width, height and texture. This will crop the picture to best
/// fill the given dimensions.
pub fn fitted_image(w: i32, h: i32, texture: Rc<Texture>) -> Element {
    new_element(w, h, Prim::Image(ImageStyle::Fitted, w, h, texture))
}

/// Create a cropped image. Take a rectangle out of the picture starting at the given top left
/// coordinate.
pub fn cropped_image(x: i32, y: i32, w: i32, h: i32, texture: Rc<Texture>) -> Element {
    new_element(w, h, Prim::Image(ImageStyle::Cropped(x, y), w, h, texture))
}

/// Create a tiled image given a width, height and texture.
pub fn tiled_image(w: i32, h: i32, texture: Rc<Texture>) -> Element {
    new_element(w, h, Prim::Image(ImageStyle::Tiled, w, h, texture))
}


#[derive(Copy, Clone, Debug)]
pub enum Three { P, Z, N }
#[derive(Copy, Clone, Debug)]
pub enum Pos { Absolute(i32), Relative(f32) }

/// An element's Position.
#[derive(Copy, Clone, Debug)]
pub struct Position {
    horizontal: Three,
    vertical: Three,
    x: Pos,
    y: Pos,
}

/// The direction for a flow of `Element`s.
#[derive(Copy, Clone, Debug)]
pub enum Direction { Up, Down, Left, Right, In, Out }


/// Have a list of elements flow in a particular direction. The `Direction` starts from the first
/// element in the list. The result is an `Element`.
pub fn flow(dir: Direction, elements: Vec<Element>) -> Element {
    if elements.is_empty() { return empty() }
    let max_w = elements.iter().map(|e| width_of(e)).max().unwrap();
    let max_h = elements.iter().map(|e| height_of(e)).max().unwrap();
    let sum_w = elements.iter().fold(0, |total, e| total + width_of(e));
    let sum_h = elements.iter().fold(0, |total, e| total + height_of(e));
    let new_flow = |w: i32, h: i32| new_element(w, h, Prim::Flow(dir, elements));
    match dir {
        Direction::Up | Direction::Down    => new_flow(max_w, sum_h),
        Direction::Left | Direction::Right => new_flow(sum_w, max_h),
        Direction::In | Direction::Out     => new_flow(max_w, max_h),
    }
}

/// Layer elements on top of each other, starting from the bottom.
pub fn layers(elements: Vec<Element>) -> Element {
    let max_w = elements.iter().map(|e| width_of(e)).max().unwrap();
    let max_h = elements.iter().map(|e| height_of(e)).max().unwrap();
    new_element(max_w, max_h, Prim::Flow(outward(), elements))
}


/// Repetitive things.
pub fn absolute(i: i32) -> Pos { Pos::Absolute(i) }
pub fn relative(f: f32) -> Pos { Pos::Relative(f) }

#[inline]
fn p(h: Three, v: Three, x: Pos, y: Pos) -> Position {
    Position { horizontal: h, vertical: v, x: x, y: y }
}

pub fn middle()       -> Position { p(Z, Z, relative(0.5), relative(0.5)) }
pub fn top_left()     -> Position { p(N, P, absolute(0), absolute(0)) }
pub fn top_right()    -> Position { p(P, P, absolute(0), absolute(0)) }
pub fn bottom_left()  -> Position { p(N, N, absolute(0), absolute(0)) }
pub fn bottom_right() -> Position { p(P, N, absolute(0), absolute(0)) }
pub fn mid_left()     -> Position { p(N, Z, absolute(0), relative(0.5)) }
pub fn mid_right()    -> Position { p(P, Z, absolute(0), relative(0.5)) }
pub fn mid_top()      -> Position { p(Z, P, relative(0.5), absolute(0)) }
pub fn mid_bottom()   -> Position { p(Z, N, relative(0.5), absolute(0)) }

pub fn middle_at(x: Pos, y: Pos)       -> Position { p(Z, Z, x, y) }
pub fn top_left_at(x: Pos, y: Pos)     -> Position { p(N, P, x, y) }
pub fn top_right_at(x: Pos, y: Pos)    -> Position { p(P, P, x, y) }
pub fn bottom_left_at(x: Pos, y: Pos)  -> Position { p(N, N, x, y) }
pub fn bottom_right_at(x: Pos, y: Pos) -> Position { p(P, N, x, y) }
pub fn mid_left_at(x: Pos, y: Pos)     -> Position { p(N, Z, x, y) }
pub fn mid_right_at(x: Pos, y: Pos)    -> Position { p(P, Z, x, y) }
pub fn mid_top_at(x: Pos, y: Pos)      -> Position { p(Z, P, x, y) }
pub fn mid_bottom_at(x: Pos, y: Pos)   -> Position { p(Z, N, x, y) }

pub fn up() -> Direction { Direction::Up }
pub fn down() -> Direction { Direction::Down }
pub fn left() -> Direction { Direction::Left }
pub fn right() -> Direction { Direction::Right }
pub fn inward() -> Direction { Direction::In }
pub fn outward() -> Direction { Direction::Out }

