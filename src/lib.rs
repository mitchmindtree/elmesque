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

#![feature(box_patterns, box_syntax, core)]

extern crate graphics;
extern crate num;
extern crate vecmath;

pub use element::{Element, Renderer};
pub use form::{Form};

pub use color as colour;
pub mod color;
pub mod element;
pub mod form;
pub mod text;
pub mod transform_2d;
pub mod utils;

