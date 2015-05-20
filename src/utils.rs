
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

/// Clamp a value to a range.
#[inline]
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val < min { min } else { if val > max { max } else { val } }
}

/// Map a value from a given range to a new given range.
pub fn map_range<X: NumCast, Y: NumCast>
(val: X, in_min: X, in_max: X, out_min: Y, out_max: Y) -> Y {
    let val_f: f64 = NumCast::from(val).unwrap();
    let in_min_f: f64 = NumCast::from(in_min).unwrap();
    let in_max_f: f64 = NumCast::from(in_max).unwrap();
    let out_min_f: f64 = NumCast::from(out_min).unwrap();
    let out_max_f: f64 = NumCast::from(out_max).unwrap();
    NumCast::from(
        (val_f - in_min_f) / (in_max_f - in_min_f) * (out_max_f - out_min_f) + out_min_f
    ).unwrap()
}


