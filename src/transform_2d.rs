//!
//! Ported from [elm-lang's Transform2D module]
//! (https://github.com/elm-lang/core/blob/62b22218c42fb8ccc996c86bea450a14991ab815/src/Transform2D.elm)
//!
//!
//! A library for performing 2D matrix transformations. It is used primarily with the
//! `group_transform` function from the `form` module and allows you to do things like rotation,
//! scaling, translation, shearing and reflection.
//!
//! Note that all the matrices in this library are 3*3 matrices of homogeneous coordinates, used
//! for affine transformations. Since the bottom row is always `0 0 1` in these matrices, it is
//! omitted in the diagrams below.
//!


use vecmath::{mat2x3_id, Matrix2x3, row_mat2x3_mul};

pub type Matrix2d = Matrix2x3<f64>;

/// Represents a 2D transform.
#[derive(Clone, Debug)]
pub struct Transform2D(pub Matrix2d);


/// Create an identity transform. Transforming by the identity does not change anything, but it can
/// come in handy as a default or base case.
///
///     / 1 0 0 \
///     \ 0 1 0 /
///
#[inline]
pub fn identity() -> Transform2D {
    Transform2D(mat2x3_id())
}

/// Creates a transformation matrix. This lets you create transforms such as scales, shears,
/// reflections and translations.
///
///     / a b x \
///     \ c d y /
///
#[inline]
pub fn matrix(a: f64, b: f64, c: f64, d: f64, x: f64, y: f64) -> Transform2D {
    Transform2D([ [a, b, x], [c, d, y] ])
}

/// Create a [rotation matrix](http://en.wikipedia.org/wiki/Rotation_matrix). Given an angle t, it
/// creates a counterclockwise rotation matrix.
///
///     / cos t  -sin t  0 \
///     \ sin t   cos t  0 /
///
#[inline]
pub fn rotation(t: f64) -> Transform2D {
    Transform2D([ [t.cos(), -t.sin(), 0.0], [t.sin(), t.cos(), 0.0] ])
}

/// Creates a transformation matrix for translation.
///
///     / 1 0 x \
///     \ 0 1 y /
///
#[inline]
pub fn translation(x: f64, y: f64) -> Transform2D {
    matrix(1.0, 0.0, 0.0, 1.0, x, y)
}

/// Creates a transformation matrix for scaling by all directions.
///
///     / s 0 0 \
///     \ 0 s 0 /
///
#[inline]
pub fn scale(s: f64) -> Transform2D {
    matrix(s, 0.0, 0.0, s, 0.0, 0.0)
}

/// Creates a transformation for horizontal scaling.
#[inline]
pub fn scale_x(s: f64) -> Transform2D {
    matrix(s, 0.0, 0.0, 1.0, 0.0, 0.0)
}

/// Creates a transformation for vertical scaling.
#[inline]
pub fn scale_y(s: f64) -> Transform2D {
    matrix(1.0, 0.0, 0.0, s, 0.0, 0.0)
}

/// Multiply two transforms together.
///
///     / ma mb mx \   / na nb nx \
///     | mc md my | . | nc nd ny |
///     \  0  0  1 /   \  0  0  1 /
///
#[inline]
pub fn multiply(Transform2D(m): Transform2D, Transform2D(n): Transform2D) -> Transform2D {
    Transform2D(row_mat2x3_mul(m, n))
}

