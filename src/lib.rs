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

extern crate graphics;
extern crate rand;
extern crate rustc_serialize;
extern crate vecmath;

pub use color as colour;
pub use element::{Element, Renderer};
pub use form::{Form};

pub mod color;
pub mod element;
pub mod form;
pub mod text;
pub mod transform_2d;
pub mod utils;
