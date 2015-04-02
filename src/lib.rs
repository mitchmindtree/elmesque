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

#![feature(box_syntax, core)]

extern crate gfx_device_gl;
extern crate gfx_texture;
extern crate graphics;
extern crate vecmath;

pub use color as colour;
pub mod color;
pub mod element;
pub mod form;
pub mod text;
pub mod transform_2d;
pub mod utils;

pub type Texture = ::gfx_texture::Texture<::gfx_device_gl::GlResources>;
