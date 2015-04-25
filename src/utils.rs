
use num::{Float, NumCast};
use num::PrimInt as Int;
use num::traits::cast;
use std::f32::consts::PI;

/// Clamp a f32 between 0f32 and 1f32.
pub fn clampf32(f: f32) -> f32 {
    if f < 0f32 { 0f32 } else if f > 1f32 { 1f32 } else { f }
}

/// Convert degrees to radians.
pub fn degrees<F: Float + NumCast>(d: F) -> F {
    d * cast(PI / 180.0).unwrap()
}

/// Convert turns to radians.
pub fn turns<F: Float + NumCast>(t: F) -> F {
    let f: F = cast(2.0 * PI).unwrap();
    f * t
}

/// The modulo function.
#[inline]
pub fn modulo<I: Int>(a: I, b: I) -> I {
    match a % b {
        r if (r > I::zero() && b < I::zero())
          || (r < I::zero() && b > I::zero()) => r + b,
        r                                     => r,
    }
}

/// Modulo float.
pub fn fmod(f: f32, n: i32) -> f32 {
    let i = f.floor() as i32;
    modulo(i, n) as f32 + f - i as f32
}

/// Return the min between to floats.
pub fn min(a: f32, b: f32) -> f32 {
    if a <= b { a } else { b }
}

/// Return the max between to floats.
pub fn max(a: f32, b: f32) -> f32 {
    if a >= b { a } else { b }
}

