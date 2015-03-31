//! 
//! Ported from [elm-lang's color module]
//! (https://github.com/elm-lang/core/blob/62b22218c42fb8ccc996c86bea450a14991ab815/src/Color.elm)
//!
//!
//! Module for working with colors. Includes [RGB](https://en.wikipedia.org/wiki/RGB_color_model)
//! and [HSL](http://en.wikipedia.org/wiki/HSL_and_HSV) creation, gradients and built-in names.
//!


use utils::{degrees, fmod, min, max, turns};
use std::f32::consts::PI;
use std::num::Float;


/// Color supporting RGB and HSL variants.
#[derive(Copy, Clone, Debug)]
pub enum Color {
    Rgba(u8, u8, u8, f32),
    Hsla(f32, f32, f32, f32),
}
/// Regional spelling alias.
pub type Colour = Color;


/// Create RGB colors with an alpha component for transparency.
/// The alpha component is specified with numbers between 0 and 1.
pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Color {
    Color::Rgba(r, g, b, a)
}


/// Create RGB colors from numbers between 0 and 255 inclusive.
pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgba(r, g, b, 1.0)
}


/// Create [HSL colors](http://en.wikipedia.org/wiki/HSL_and_HSV) with an alpha component for
/// transparency.
pub fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Color {
    Color::Hsla(hue - turns((hue / (2.0 * PI)).floor()), saturation, lightness, alpha)
}


/// Create [HSL colors](http://en.wikipedia.org/wiki/HSL_and_HSV). This gives you access to colors
/// more like a color wheel, where all hues are arranged in a circle that you specify with radians.
/// 
///   red        = hsl(degrees(0.0)   , 1.0 , 0.5)
///   green      = hsl(degrees(120.0) , 1.0 , 0.5)
///   blue       = hsl(degrees(240.0) , 1.0 , 0.5)
///   pastel_red = hsl(degrees(0.0)   , 0.7 , 0.7)
///
/// To cycle through all colors, just cycle through degrees. The saturation level is how vibrant
/// the color is, like a dial between grey and bright colors. The lightness level is a dial between
/// white and black.
pub fn hsl(hue: f32, saturation: f32, lightness: f32) -> Color {
    hsla(hue, saturation, lightness, 1.0)
}


/// Produce a gray based on the input. 0.0 is white, 1.0 is black.
pub fn grayscale(p: f32) -> Color {
    Color::Hsla(0.0, 0.0, 1.0-p, 1.0)
}
/// Produce a gray based on the input. 0.0 is white, 1.0 is black.
pub fn greyscale(p: f32) -> Color {
    Color::Hsla(0.0, 0.0, 1.0-p, 1.0)
}


impl Color {

    /// Produce a complementary color. The two colors will accent each other. This is the same as
    /// rotating the hue by 180 degrees.
    #[inline]
    pub fn complement(self) -> Color {
        match self {
            Color::Hsla(h, s, l, a) => hsla(h + degrees(180.0), s, l, a),
            Color::Rgba(r, g, b, a) => {
                let (h, s, l) = rgb_to_hsl(r, g, b);
                hsla(h + degrees(180.0), s, l, a)
            },
        }
    }

    /// Extract the components of a color in the HSL format.
    #[inline]
    pub fn to_hsl(self) -> Hsla {
        match self {
            Color::Hsla(h, s, l, a) => Hsla { hue: h, saturation: s, lightness: l, alpha: a },
            Color::Rgba(r, g, b, a) => {
                let (h, s, l) = rgb_to_hsl(r, g, b);
                Hsla { hue: h, saturation: s, lightness: l, alpha: a }
            },
        }
    }

    /// Extract the components of a color in the RGB format.
    pub fn to_rgb(self) -> Rgba {
        match self {
            Color::Rgba(r, g, b, a) => Rgba { red: r, green: g, blue: b, alpha: a },
            Color::Hsla(h, s, l, a) => {
                let (r, g, b) = hsl_to_rgb(h, s, l);
                Rgba {
                    red: (255.0 * r).round() as u8,
                    green: (255.0 * g).round() as u8,
                    blue: (255.0 * b).round() as u8,
                    alpha: a,
                }
            },
        }
    }

}

/// The parts of HSL along with an alpha for transparency.
#[derive(Copy, Clone, Debug)]
pub struct Hsla {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub alpha: f32,
}



/// The parts of RGB along with an alpha for transparency.
#[derive(Copy, Clone, Debug)]
pub struct Rgba {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: f32,
}


pub fn rgb_to_hsl(red: u8, green: u8, blue: u8) -> (f32, f32, f32) {
    let r = red as f32 / 255.0;
    let g = green as f32 / 255.0;
    let b = blue as f32 / 255.0;
    let c_max = max(max(r, g), b);
    let c_min = min(min(r, g), b);
    let c = c_max - c_min;
    let hue = degrees(60.0) * if      c_max == r { fmod(((g - b) / c), 6) }
                              else if c_max == g { ((b - r) / c) + 2.0 }
                              else               { ((r - g) / c) + 4.0 };
    let lightness = (c_max + c_min) / 2.0;
    let saturation = if lightness == 0.0 { 0.0 }
                     else { c / (1.0 - (2.0 * lightness - 1.0).abs()) };
    (hue, saturation, lightness)
}


pub fn hsl_to_rgb(hue: f32, saturation: f32, lightness: f32) -> (f32, f32, f32) {
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let hue = hue / degrees(60.0);
    let x = chroma * (1.0 - (fmod(hue, 2) - 1.0).abs());
    let (r, g, b) = match hue {
        hue if hue < 0.0 => (0.0, 0.0, 0.0),
        hue if hue < 1.0 => (chroma, x, 0.0),
        hue if hue < 2.0 => (x, chroma, 0.0),
        hue if hue < 3.0 => (0.0, chroma, x),
        hue if hue < 4.0 => (0.0, x, chroma),
        hue if hue < 5.0 => (x, 0.0, chroma),
        hue if hue < 6.0 => (chroma, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };
    let m = lightness - chroma / 2.0;
    (r + m, g + m, b + m)
}


/// Linear or Radial Gradient.
#[derive(Clone, Debug)]
pub enum Gradient {
    Linear((f64, f64), (f64, f64), Vec<(f64, Color)>),
    Radial((f64, f64), f64, (f64, f64), f64, Vec<(f64, Color)>),
}


/// Create a linear gradient. Takes a start and end point and then a series of color stops that
/// indicate how to interpolate between the start and end points.
pub fn linear(start: (f64, f64), end: (f64, f64), colors: Vec<(f64, Color)>) -> Gradient {
    Gradient::Linear(start, end, colors)
}


/// Create a radial gradient. First takes a start point and inner radius. Then takes an end point
/// and outer radius. It then takes a series of color stops that indicate how to interpolate
/// between the inner and outer circles.
pub fn radial(start: (f64, f64), start_r: f64,
              end: (f64, f64), end_r: f64,
              colors: Vec<(f64, Color)>) -> Gradient {
    Gradient::Radial(start, start_r, end, end_r, colors)
}


/// Built-in colors.
///
/// These colors come from the
/// [Tango palette](http://tango.freedesktop.org/Tango_Icon_Theme_Guidelines) which provides
/// aesthetically reasonable defaults for colors. Each color also comes with a light and dark
/// version.

pub fn light_red()      -> Color { Color::Rgba(239 , 41  , 41  , 1.0) }
pub fn red()            -> Color { Color::Rgba(204 , 0   , 0   , 1.0) }
pub fn dark_red()       -> Color { Color::Rgba(164 , 0   , 0   , 1.0) }

pub fn light_orange()   -> Color { Color::Rgba(252 , 175 , 62  , 1.0) }
pub fn orange()         -> Color { Color::Rgba(245 , 121 , 0   , 1.0) }
pub fn dark_orange()    -> Color { Color::Rgba(206 , 92  , 0   , 1.0) }

pub fn light_yellow()   -> Color { Color::Rgba(255 , 233 , 79  , 1.0) }
pub fn yellow()         -> Color { Color::Rgba(237 , 212 , 0   , 1.0) }
pub fn dark_yellow()    -> Color { Color::Rgba(196 , 160 , 0   , 1.0) }

pub fn light_green()    -> Color { Color::Rgba(138 , 226 , 52  , 1.0) }
pub fn green()          -> Color { Color::Rgba(115 , 210 , 22  , 1.0) }
pub fn dark_green()     -> Color { Color::Rgba(78  , 154 , 6   , 1.0) }

pub fn light_blue()     -> Color { Color::Rgba(114 , 159 , 207 , 1.0) }
pub fn blue()           -> Color { Color::Rgba(52  , 101 , 164 , 1.0) }
pub fn dark_blue()      -> Color { Color::Rgba(32  , 74  , 135 , 1.0) }

pub fn light_purple()   -> Color { Color::Rgba(173 , 127 , 168 , 1.0) }
pub fn purple()         -> Color { Color::Rgba(117 , 80  , 123 , 1.0) }
pub fn dark_purple()    -> Color { Color::Rgba(92  , 53  , 102 , 1.0) }

pub fn light_brown()    -> Color { Color::Rgba(233 , 185 , 110 , 1.0) }
pub fn brown()          -> Color { Color::Rgba(193 , 125 , 17  , 1.0) }
pub fn dark_brown()     -> Color { Color::Rgba(143 , 89  , 2   , 1.0) }

pub fn black()          -> Color { Color::Rgba(0   , 0   , 0   , 1.0) }
pub fn white()          -> Color { Color::Rgba(255 , 255 , 255 , 1.0) }

pub fn light_gray()     -> Color { Color::Rgba(238 , 238 , 236 , 1.0) }
pub fn gray()           -> Color { Color::Rgba(211 , 215 , 207 , 1.0) }
pub fn dark_gray()      -> Color { Color::Rgba(186 , 189 , 182 , 1.0) }

pub fn light_grey()     -> Color { Color::Rgba(238 , 238 , 236 , 1.0) }
pub fn grey()           -> Color { Color::Rgba(211 , 215 , 207 , 1.0) }
pub fn dark_grey()      -> Color { Color::Rgba(186 , 189 , 182 , 1.0) }

pub fn light_charcoal() -> Color { Color::Rgba(136 , 138 , 133 , 1.0) }
pub fn charcoal()       -> Color { Color::Rgba(85  , 87  , 83  , 1.0) }
pub fn dark_charcoal()  -> Color { Color::Rgba(46  , 52  , 54  , 1.0) }

