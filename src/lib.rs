// Copyright (c) 2015 George Burton
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

#![feature(cmp_partial)]
#![deny(warnings)]
#![warn(missing_docs)]

//! Generic Rgb and Rgba colors, and common functions for
//! manipulating them. Intended to be used standalone, or
//! to complement matrix/vector math libraries like CGMath,
//! Vecmath, or Nalgebra in graphics programming.
//!
//! Requires rust-nightly.

extern crate num;

#[cfg(test)]
extern crate quickcheck;

pub use channel::Channel;
pub use rgb::Rgb;
pub use rgba::Rgba;

use num::{Float, NumCast, One, PrimInt, Unsigned, Zero};
use num::traits::cast;

use std::cmp::{partial_min, partial_max};

#[macro_use]
mod simplecolor_macros;

mod channel;
mod rgb;
mod rgba;

/// A generic color.
pub trait Color<T: Channel> {
    /// Clamp each component between two scalar values.
    fn clamp_scalar(&self, min: T, max: T) -> Self;

    /// Clamp each component piecewise between zero, and the
    /// corresponding channel for the other color.
    fn clamp_color(&self, min: &Self, max: &Self) -> Self;

    /// For floating point channels, clamp each channel between
    /// 0 and 1. For integer channels, this does nothing.
    fn normalise(&self) -> Self;

    /// Invert the color.
    fn invert(&self) -> Self;

    /// Get the relative brightness of a color.
    fn luminance(&self) -> T;

    /// Mix two colors together using the standard Rgb
    /// color model.
    fn mix(&self, other: &Self) -> Self;

    /// Convert a color to greyscale.
    fn to_greyscale(&self) -> Self;
}

// crate-private functions

/// Clamp a value x between 2 other values. The minimum
/// value must be smaller than the maximum value. May
/// panic if either the min or max value cannot be compared
/// to the value to be clamped(e.g. if they are NaNs).
#[inline]
fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
    assert!(max >= min);
    partial_min(x, max)
        .and_then(|y| partial_max(y, min))
        .unwrap()
}

/// Clamp a floating value between zero and one.
#[inline]
fn clamp_to_zero_one<T: PartialOrd + Float>(x: T) -> T {
    clamp(x, T::zero(), T::one())
}

/// Convert a primitive integer type to a generic floating point value,
/// scaled from [0, integer::max_value()] to [0.0, 1.0].
#[inline]
fn integral_to_float<I, F>(x: I) -> F
    where I: PrimInt + Unsigned,
          F: Float + NumCast {
    cast::<I, F>(x).unwrap() / cast(I::max_value()).unwrap()
}

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::{clamp, integral_to_float};
    use quickcheck::{quickcheck, TestResult};

    macro_rules! discard_if(
        ($cond:expr) => (
            if $cond {
                use $crate::quickcheck::TestResult;
                return TestResult::discard();
            }
        )
    );

    pub fn is_between<T: PartialOrd>(x: T, lower: T, higher: T) -> bool {
        lower <= x && x <= higher
    }

    #[test]
    fn test_clamping_number_is_clamped_properly() {
        fn prop_is_clamped_between<T: PartialOrd + Copy>(x: T, mn: T, mx: T) -> TestResult {
            discard_if!(mn >= mx);
            TestResult::from_bool({
                let a = clamp(x, mn, mx);
                is_between(a, mn, mx)
            })
        }
        quickcheck(prop_is_clamped_between::<i32> as fn(i32, i32, i32) -> TestResult);
        quickcheck(prop_is_clamped_between::<f64> as fn(f64, f64, f64) -> TestResult);
    }

    #[test]
    fn test_conversion_from_integral_to_float() {
        assert_eq!(integral_to_float::<_, f32>(u8::max_value()), 1.0);
        assert_eq!(integral_to_float::<_, f32>(u8::min_value()), 0.0);
    }
}