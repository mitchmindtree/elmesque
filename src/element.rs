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
use form::{self, Form};
use graphics::character::CharacterCache;
use graphics::{DrawState, Graphics};
use self::Three::{P, Z, N};
use std::path::PathBuf;
use transform_2d::{self, Matrix2d, Transform2D};


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
    pub id: Guid,
    pub width: i32,
    pub height: i32,
    pub opacity: f32,
    pub color: Option<Color>,
}


/// Graphical elements that snap together to build complex widgets and layouts.
///
/// Each element is a rectangle with a known width and height, making them easy to combine and
/// position.
#[derive(Clone, Debug)]
pub struct Element {
    pub props: Properties,
    pub element: Prim,
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
        new_element(w, h, Prim::Container(pos, Box::new(self)))
    }

    /// Put an element in a cleared wrapper. The color provided will be the color that clears the
    /// screen before rendering the contained element.
    #[inline]
    pub fn clear(self, color: Color) -> Element {
        new_element(self.get_width(), self.get_height(),
            Prim::Cleared(color, Box::new(self)))
    }

    /// Stack elements vertically. To put `a` above `b` you would say: `a.above(b)`
    #[inline]
    pub fn above(self, other: Element) -> Element {
        new_element(::std::cmp::max(self.get_width(), other.get_width()),
                    self.get_height() + other.get_height(),
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
        new_element(self.get_width() + other.get_width(),
                    ::std::cmp::max(self.get_height(), other.get_height()),
                    Prim::Flow(right(), vec![self, other]))
    }

    /// Return the width of the Element.
    pub fn get_width(&self) -> i32 { self.props.width }

    /// Return the height of the Element.
    pub fn get_height(&self) -> i32 { self.props.height }

    /// Return the size of the Element's bounding rectangle.
    pub fn get_size(&self) -> (i32, i32) { (self.props.width, self.props.height) }

    /// Draw the form with some given graphics backend.
    #[inline]
    pub fn draw<'a, C, G>(self, renderer: &mut Renderer<'a, C, G>)
        where
            C: CharacterCache,
            G: Graphics<Texture=C::Texture>,
    {
        use graphics::{Context, Transformed};
        use transform_2d::scale_y;
        let Renderer {
            ref width,
            ref height,
            ref mut backend,
            ref mut maybe_character_cache,
        } = *renderer;
        let context = Context::abs(*width, *height).trans(*width / 2.0, *height / 2.0);
        let Transform2D(matrix) = Transform2D(context.transform).multiply(scale_y(-1.0));
        draw_element(self, matrix, *backend, maybe_character_cache, &context.draw_state);
    }

    /// Return whether or not a point is over the element.
    pub fn is_over(&self, x: i32, y: i32) -> bool {
        unimplemented!();
    }

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
    Image(ImageStyle, i32, i32, PathBuf),
    Container(Position, Box<Element>),
    Flow(Direction, Vec<Element>),
    Collage(i32, i32, Vec<Form>),
    Cleared(Color, Box<Element>),
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
pub fn image(w: i32, h: i32, path: PathBuf) -> Element {
    new_element(w, h, Prim::Image(ImageStyle::Plain, w, h, path))
}

/// Create a fitted image given a width, height and texture. This will crop the picture to best
/// fill the given dimensions.
pub fn fitted_image(w: i32, h: i32, path: PathBuf) -> Element {
    new_element(w, h, Prim::Image(ImageStyle::Fitted, w, h, path))
}

/// Create a cropped image. Take a rectangle out of the picture starting at the given top left
/// coordinate.
pub fn cropped_image(x: i32, y: i32, w: i32, h: i32, path: PathBuf) -> Element {
    new_element(w, h, Prim::Image(ImageStyle::Cropped(x, y), w, h, path))
}

/// Create a tiled image given a width, height and texture.
pub fn tiled_image(w: i32, h: i32, path: PathBuf) -> Element {
    new_element(w, h, Prim::Image(ImageStyle::Tiled, w, h, path))
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
    let max_w = elements.iter().map(|e| e.get_width()).max().unwrap();
    let max_h = elements.iter().map(|e| e.get_height()).max().unwrap();
    let sum_w = elements.iter().fold(0, |total, e| total + e.get_width());
    let sum_h = elements.iter().fold(0, |total, e| total + e.get_height());
    let new_flow = |w: i32, h: i32| new_element(w, h, Prim::Flow(dir, elements));
    match dir {
        Direction::Up | Direction::Down    => new_flow(max_w, sum_h),
        Direction::Left | Direction::Right => new_flow(sum_w, max_h),
        Direction::In | Direction::Out     => new_flow(max_w, max_h),
    }
}

/// Layer elements on top of each other, starting from the bottom.
pub fn layers(elements: Vec<Element>) -> Element {
    let max_w = elements.iter().map(|e| e.get_width()).max().unwrap();
    let max_h = elements.iter().map(|e| e.get_height()).max().unwrap();
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








/// 
/// CUSTOM NON-ELM FUNCTIONS
///



/// Used for rendering elmesque `Element`s.
pub struct Renderer<'a, C: 'a, G: 'a> {
    width: f64,
    height: f64,
    backend: &'a mut G,
    maybe_character_cache: Option<&'a mut C>,
}

impl<'a, C, G> Renderer<'a, C, G> {

    /// Construct a renderer, used for rendering elmesque `Element`s.
    pub fn new(width: f64, height: f64, backend: &'a mut G) -> Renderer<'a, C, G> {
        Renderer {
            width: width,
            height: height,
            backend: backend,
            maybe_character_cache: None,
        }
    }

    /// Builder method for constructing a Renderer with a GlyphCache for drawing text.
    pub fn character_cache(self, character_cache: &'a mut C) -> Renderer<'a, C, G> {
        Renderer { maybe_character_cache: Some(character_cache), ..self }
    }

}



/// Draw an Element.
pub fn draw_element<'a, C: CharacterCache, G: Graphics<Texture=C::Texture>>(
    element: Element,
    matrix: Matrix2d,
    backend: &mut G,
    maybe_character_cache: &mut Option<&mut C>,
    draw_state: &DrawState
) {
    let Element { props, element } = element;
    match element {

        Prim::Image(style, w, h, path) => {
            let Properties { id, width, height, opacity, color } = props;
            match style {
                ImageStyle::Plain => {
                    // let image = graphics::Image {
                    //     color: None,
                    //     rectangle: None,
                    //     source_rectangle: Some([src_x, src_y, w, h]),
                    // };
                    // let image = Image::new();
                    // let texture: &Texture = ::std::ops::Deref::deref(&texture);
                    // image.draw(texture, draw_state, matrix, backend);
                    unimplemented!();
                },
                ImageStyle::Fitted => {
                    unimplemented!();
                },
                ImageStyle::Cropped(x, y) => {
                    unimplemented!();
                },
                ImageStyle::Tiled => {
                    unimplemented!();
                },
            }
        },

        Prim::Container(position, element) => {
            let Position { horizontal, vertical, x, y } = position;
            let Transform2D(matrix) = match (x, y) {
                (Pos::Relative(x), Pos::Relative(y)) => {
                    Transform2D(matrix).multiply(transform_2d::translation(x as f64, y as f64))
                },
                (Pos::Absolute(x), Pos::Relative(y)) => {
                    transform_2d::matrix(1.0, 0.0, 0.0, 1.0, x as f64, 0.0)
                        .multiply(transform_2d::translation(0.0, y as f64))
                },
                (Pos::Relative(x), Pos::Absolute(y)) => {
                    transform_2d::matrix(1.0, 0.0, 0.0, 1.0, 0.0, y as f64)
                        .multiply(transform_2d::translation(x as f64, 0.0))
                },
                (Pos::Absolute(x), Pos::Absolute(y)) => {
                    transform_2d::matrix(1.0, 0.0, 0.0, 1.0, x as f64, y as f64)
                },
            };
            let new_opacity = props.opacity * element.props.opacity;
            let element = element.opacity(new_opacity);
            draw_element(element, matrix, backend, maybe_character_cache, draw_state);
        }

        Prim::Flow(direction, elements) => {
            let mut matrix = matrix;
            match direction {
                Direction::Up | Direction::Down => {
                    let multi = if let Direction::Up = direction { 1.0 } else { -1.0 };
                    let mut half_prev_height = 0.0;
                    for element in elements.into_iter() {
                        let new_opacity = props.opacity * element.props.opacity;
                        let element = element.opacity(new_opacity);
                        let half_height = element.get_height() as f64 / 2.0;
                        draw_element(element, matrix, backend, maybe_character_cache, draw_state);
                        let y_trans = half_height + half_prev_height;
                        let Transform2D(new_matrix) = Transform2D(matrix)
                            .multiply(transform_2d::translation(0.0, y_trans * multi));
                        matrix = new_matrix;
                        half_prev_height = half_height;
                    }
                },
                Direction::Left | Direction::Right => {
                    let multi = if let Direction::Right = direction { 1.0 } else { -1.0 };
                    let mut half_prev_width = 0.0;
                    for element in elements.into_iter() {
                        let new_opacity = props.opacity * element.props.opacity;
                        let element = element.opacity(new_opacity);
                        let half_width = element.get_width() as f64 / 2.0;
                        draw_element(element, matrix, backend, maybe_character_cache, draw_state);
                        let x_trans = half_width + half_prev_width;
                        let Transform2D(new_matrix) = Transform2D(matrix)
                            .multiply(transform_2d::translation(x_trans * multi, 0.0));
                        matrix = new_matrix;
                        half_prev_width = half_width;
                    }
                },
                Direction::Out => {
                    for element in elements.into_iter() {
                        let new_opacity = props.opacity * element.props.opacity;
                        let element = element.opacity(new_opacity);
                        draw_element(element, matrix, backend, maybe_character_cache, draw_state);
                    }
                }
                Direction::In => {
                    for element in elements.into_iter().rev() {
                        let new_opacity = props.opacity * element.props.opacity;
                        let element = element.opacity(new_opacity);
                        draw_element(element, matrix, backend, maybe_character_cache, draw_state);
                    }
                }
            }
        },

        Prim::Collage(w, h, forms) => {
            for form in forms {
                let original_alpha = form.alpha;
                let form = form.alpha(original_alpha * props.opacity);
                form::draw_form(form, matrix, backend, maybe_character_cache, draw_state);
            }
        },

        Prim::Cleared(color, element) => {
            backend.clear_color(color.to_fsa());
            draw_element(*element, matrix, backend, maybe_character_cache, draw_state);
        },

        Prim::Spacer => {},

    }
}

