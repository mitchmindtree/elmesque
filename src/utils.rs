//! 
//! Miscellaneous utility functions.
//!

use std::f64::consts::PI;
use std::num::{Float, Int};

/// Convert turns to radians.
pub fn turns(t: f64) -> f64 {
    2.0 * PI / t
}

/// Convert degrees to radians.
pub fn degrees(d: f64) -> f64 {
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
pub fn fmod(f: f64, n: i64) -> f64 {
    let i = f.floor() as i64;
    modulo(i, n) as f64 + f - i as f64
}

/// Return the max between to floats.
pub fn max(a: f64, b: f64) -> f64 {
    if a >= b { a } else { b }
}

/// Return the min between to floats.
pub fn min(a: f64, b: f64) -> f64 {
    if a <= b { a } else { b }
}

