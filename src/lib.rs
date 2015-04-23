//!
//! This crate is an attempt at porting Elm's incredibly useful std graphics modules.
//!
//! Visit [elm-lang.org](http://elm-lang.org/).
//!
//!
//! All credit goes to Evan Czaplicki for all algorithms included within.
//!
//! Ported to Rust by Mitchell Nordine.
//!

extern crate color as color_lib;
extern crate graphics;
extern crate num;
extern crate vecmath;

pub use element::{Element, Renderer};
pub use form::{Form};

pub use color_lib as color;
pub use color_lib as colour;
pub mod element;
pub mod form;
pub mod text;
pub mod transform_2d;
pub mod utils;
