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
use graphics::{Context, Graphics, Transformed};
use self::Three::{P, Z, N};
use std::path::PathBuf;
use transform_2d;


/// An Element's Properties.
#[derive(Clone, Debug)]
pub struct Properties {
    pub width: i32,
    pub height: i32,
    pub opacity: f32,
    pub crop: Option<(f64, f64, f64, f64)>,
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

    /// Crops an `Element` with the given rectangle.
    #[inline]
    pub fn crop(self, x: f64, y: f64, w: f64, h: f64) -> Element {
        let Element { props, element } = self;
        let new_props = Properties { crop: Some((x, y, w, h)), ..props };
        Element { props: new_props, element: element }
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
    pub fn draw<'a, C, G>(&self, renderer: &mut Renderer<'a, C, G>)
        where
            C: CharacterCache,
            G: Graphics<Texture=C::Texture>,
    {
        let Renderer {
            context,
            ref mut backend,
            ref mut maybe_character_cache,
        } = *renderer;
        let view_size = context.get_view_size();
        let context = context.trans(view_size[0] / 2.0, view_size[1] / 2.0).scale(1.0, -1.0);
        draw_element(self, 1.0, *backend, maybe_character_cache, context);
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
            width: w,
            height: h,
            opacity: 1.0,
            color: None,
            crop: None,
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
    let max_w = elements.iter().map(|e| e.get_width()).max().unwrap_or(0);
    let max_h = elements.iter().map(|e| e.get_height()).max().unwrap_or(0);
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
    context: Context,
    backend: &'a mut G,
    maybe_character_cache: Option<&'a mut C>,
}

impl<'a, C, G> Renderer<'a, C, G> {

    /// Construct a renderer, used for rendering elmesque `Element`s.
    pub fn new(context: Context, backend: &'a mut G) -> Renderer<'a, C, G> {
        Renderer {
            context: context,
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
    element: &Element,
    opacity: f32,
    backend: &mut G,
    maybe_character_cache: &mut Option<&mut C>,
    context: Context,
) {
    let Element { ref props, ref element } = *element;

    // Crop the Element if some crop was given.
    // We'll use the `DrawState::scissor` method for this.
    //
    // Because `DrawState`'s `scissor` `Rect` uses bottom-left origin coords, we'll have to convert
    // from our centered-origin coordinate system.
    //
    // We'll also need to stretch our coords to match the correct viewport.draw_size.
    let context = match props.crop {
        Some((x, y, w, h)) => {
            use utils::{clamp, map_range};
            let Context { draw_state, .. } = context;

            // Our view_dim is our virtual window size which is consistent no matter the display.
            let view_dim = context.get_view_size();

            // Our draw_dim is the actual window size in pixels. Our target crop area must be
            // represented in this size.
            let draw_dim = match context.viewport {
                Some(viewport) => [viewport.draw_size[0] as f64, viewport.draw_size[1] as f64],
                None => view_dim,
            };

            // Calculate the distance to the edges of the window from the center.
            let left = -view_dim[0] / 2.0;
            let right = view_dim[0] / 2.0;
            let bottom = -view_dim[1] / 2.0;
            let top = view_dim[1] / 2.0;

            // We start with the x and y in the center of our crop area, however we need it to be
            // at the top left of the crop area.
            let left_x = x - w as f64 / 2.0;
            let top_y = y - h as f64 / 2.0;

            // Map the position at the top left of the crop area in view_dim to our draw_dim.
            let x = map_range(left_x, left, right, 0, draw_dim[0] as i32);
            let y = map_range(top_y, bottom, top, 0, draw_dim[1] as i32);
 
            // Convert the w and h from our view_dim to the draw_dim.
            let w_scale = draw_dim[0] / view_dim[0];
            let h_scale = draw_dim[1] / view_dim[1];
            let w = w * w_scale;
            let h = h * h_scale;

            // If we ended up with negative coords for the crop area, we'll use 0 instead as we
            // can't represent the negative coords with `u16` (the target DrawState dimension type).
            // We'll hold onto the lost negative values (x_neg and y_neg) so that we can compensate
            // with the width and height.
            let x_neg = if x < 0 { x } else { 0 };
            let y_neg = if y < 0 { y } else { 0 };
            let mut x = ::std::cmp::max(0, x) as u16;
            let mut y = ::std::cmp::max(0, y) as u16;
            let mut w = ::std::cmp::max(0, (w as i32 + x_neg)) as u16;
            let mut h = ::std::cmp::max(0, (h as i32 + y_neg)) as u16;
            
            // If there was already some scissor set, we must check for the intersection.
            if let Some(rect) = draw_state.scissor {
                if x + w < rect.x || rect.x + rect.w < x || y + h < rect.y || rect.y + rect.h < y {
                    // If there is no intersection, we have no scissor.
                    w = 0;
                    h = 0;
                } else {
                    // If there is some intersection, calculate the overlapping rect.
                    let (a_l, a_r, a_b, a_t) = (x, x+w, y, y+h);
                    let (b_l, b_r, b_b, b_t) = (rect.x, rect.x+rect.w, rect.y, rect.y+rect.h);
                    let l = if a_l > b_l { a_l } else { b_l };
                    let r = if a_r < b_r { a_r } else { b_r };
                    let b = if a_b > b_b { a_b } else { b_b };
                    let t = if a_t < b_t { a_t } else { b_t };
                    x = l;
                    y = b;
                    w = r - l;
                    h = t - b;
                }
            }

            Context { draw_state: draw_state.scissor(x, y, w, h), ..context }
        },
        None => context,
    };

    match *element {

        Prim::Image(style, w, h, ref path) => {
            let Properties { width, height, opacity, color, .. } = *props;
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

        Prim::Container(position, ref element) => {
            let Position { horizontal, vertical, x, y } = position;
            let context = match (x, y) {
                (Pos::Relative(x), Pos::Relative(y)) => context.trans(x as f64, y as f64),
                (Pos::Absolute(x), Pos::Relative(y)) => Context {
                    transform: transform_2d::matrix(1.0, 0.0, 0.0, 1.0, x as f64, 0.0).0,
                    ..context
                }.trans(0.0, y as f64),
                (Pos::Relative(x), Pos::Absolute(y)) => Context {
                    transform: transform_2d::matrix(1.0, 0.0, 0.0, 1.0, 0.0, y as f64).0,
                    ..context
                }.trans(x as f64, 0.0),
                (Pos::Absolute(x), Pos::Absolute(y)) => Context {
                    transform: transform_2d::matrix(1.0, 0.0, 0.0, 1.0, x as f64, y as f64).0,
                    ..context
                },
            };
            let new_opacity = opacity * props.opacity;
            draw_element(element, new_opacity, backend, maybe_character_cache, context);
        }

        Prim::Flow(direction, ref elements) => {
            let mut context = context;
            match direction {
                Direction::Up | Direction::Down => {
                    let multi = if let Direction::Up = direction { 1.0 } else { -1.0 };
                    let mut half_prev_height = 0.0;
                    for element in elements.iter() {
                        let half_height = element.get_height() as f64 / 2.0;
                        let new_opacity = opacity * props.opacity;
                        draw_element(element, new_opacity, backend, maybe_character_cache, context);
                        let y_trans = half_height + half_prev_height;
                        context = context.trans(0.0, y_trans * multi);
                        half_prev_height = half_height;
                    }
                },
                Direction::Left | Direction::Right => {
                    let multi = if let Direction::Right = direction { 1.0 } else { -1.0 };
                    let mut half_prev_width = 0.0;
                    for element in elements.iter() {
                        let half_width = element.get_width() as f64 / 2.0;
                        let new_opacity = opacity * props.opacity;
                        draw_element(element, new_opacity, backend, maybe_character_cache, context);
                        let x_trans = half_width + half_prev_width;
                        context = context.trans(x_trans * multi, 0.0);
                        half_prev_width = half_width;
                    }
                },
                Direction::Out => {
                    for element in elements.iter() {
                        let new_opacity = opacity * props.opacity;
                        draw_element(element, new_opacity, backend, maybe_character_cache, context);
                    }
                }
                Direction::In => {
                    for element in elements.iter().rev() {
                        let new_opacity = opacity * props.opacity;
                        draw_element(element, new_opacity, backend, maybe_character_cache, context);
                    }
                }
            }
        },

        Prim::Collage(w, h, ref forms) => {
            for form in forms.iter() {
                let new_opacity = opacity * props.opacity;
                form::draw_form(form, new_opacity, backend, maybe_character_cache, context);
            }
        },

        Prim::Cleared(color, ref element) => {
            backend.clear_color(color.to_fsa());
            draw_element(element, opacity, backend, maybe_character_cache, context);
        },

        Prim::Spacer => {},

    }
}

