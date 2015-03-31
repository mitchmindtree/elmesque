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

#![feature(core)]

extern crate vecmath;

pub use draw::{draw, draw_form, Renderer};

pub use color as colour;

pub mod color;
mod draw;
pub mod form;
pub mod text;
pub mod transform_2d;
pub mod utils;

