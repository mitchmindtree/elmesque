//! 
//! Miscellaneous utility functions.
//!

use std::f32::consts::PI;
use std::num::{Float, Int};

/// Convert turns to radians.
pub fn turns(t: f32) -> f32 {
    2.0 * PI / t
}

/// Convert degrees to radians.
pub fn degrees(d: f32) -> f32 {
    d * PI / 180.0
}

/// The modulo function.
#[inline]
pub fn modulo<I: Int>(a: I, b: I) -> I {
    match a % b {
        r if (r > Int::zero() && b < Int::zero())
          || (r < Int::zero() && b > Int::zero()) => (r + b),
        r                                         => r,
    }
}

/// Modulo float.
pub fn fmod(f: f32, n: i32) -> f32 {
    let i = f.floor() as i32;
    modulo(i, n) as f32 + f - i as f32
}

/// Return the max between to floats.
pub fn max(a: f32, b: f32) -> f32 {
    if a >= b { a } else { b }
}

/// Return the min between to floats.
pub fn min(a: f32, b: f32) -> f32 {
    if a <= b { a } else { b }
}

