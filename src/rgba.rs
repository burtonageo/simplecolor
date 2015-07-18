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

use num::{Float, NumCast, PrimInt, Unsigned};
use std::default::Default;
use std::ops::{Add, Div, Mul, Sub};

use super::{
    integral_to_float,
    Channel,
    Rgb
};

/// An Rgba color with 4 channels: red, green, blue, and alpha.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash)]
pub struct Rgba<T: Channel> {
    /// Red, Green, and Blue components
    rgb: Rgb<T>,

    /// Alpha component
    a: T
}

impl<T: Channel> Rgba<T> {
    /// Creates a new color, with the red, blue, and green components set to
    /// zero (black color), and a fully opaque alpha channel.
    pub fn new() -> Rgba<T> {
        Rgb::new().rgba(T::one())
    }

    /// Construct an Rgba color piecewise from individual components. Each
    /// component is clamped between zero and one.
    pub const fn with_components(r: T, g: T, b: T, a: T) -> Rgba<T> {
        Rgba {
            rgb: Rgb::with_components(r, g, b),
            a: a
        }
    }

    /// Construct an Rgba color from a 4-length slice of Floating numbers.
    /// Each component is clamped between zero and one.
    pub const fn from_slice(col: [T; 4]) -> Rgba<T> {
        Rgba::with_components(col[0], col[1], col[2], col[3])
    }

    /// Returns the red channel value.
    #[inline]
    pub const fn r(&self) -> T { self.rgb.r() }

    /// Returns the green channel value.
    #[inline]
    pub const fn g(&self) -> T { self.rgb.g() }

    /// Returns the blue channel value.
    #[inline]
    pub const fn b(&self) -> T { self.rgb.b() }

    /// Returns the alpha channel value.
    #[inline]
    pub const fn a(&self) -> T { self.a }

    /// Set the red channel value. The new value is clamped between zero and one.
    #[inline]
    pub fn set_r(&mut self, r: T) { self.rgb.set_r(r); }

    /// Set the green channel value. The new value is clamped between zero and one.
    #[inline]
    pub fn set_g(&mut self, g: T) { self.rgb.set_g(g); }

    /// Set the blue channel value. The new value is clamped between zero and one.
    #[inline]
    pub fn set_b(&mut self, b: T) { self.rgb.set_b(b); }

    /// Set the alpha channel value. The new value is clamped between zero and one.
    #[inline]
    pub fn set_a(&mut self, a: T) { self.a = a; }

    /// Create an Rgba color from this color, ignoring the alpha
    pub const fn rgb(&self) -> Rgb<T> { self.rgb }

    /// Return each component in a 4-element tuple. Useful for destructuring.
    ///
    /// ```rust
    /// let nice_shade_of_blue = Rgba::with_components(0.3f32, 0.3, 0.67, 1.0);
    /// let (r, g, b, a) = nice_shade_of_blue.components();
    ///
    /// assert_eq!(r, 0.3f32);
    /// assert_eq!(g, 0.3f32);
    /// assert_eq!(b, 0.67f32);
    /// assert_eq!(1, 1.0f32);
    /// ```
    pub const fn components(&self) -> (T, T, T, T) {
        (self.r(), self.g(), self.b(), self.a())
    }
    
    /// Return each component in a 4-element slice.
    pub const fn to_slice(&self) -> [T; 4] {
        [self.r(), self.g(), self.b(), self.a()]
    }
}

impl<T: Channel> Default for Rgba<T> {
    /// Identical to ```Rgba::new()```.
    fn default() -> Rgba<T> { Rgba::new() }
}

impl<F: Channel + Float + NumCast> Rgba<F> {
    /// Create an Rgba object from 4 unsigned primitive integers.
    /// The value of each component is the same as the percentage
    /// of the integral value between I::zero() and I::max_value().
    pub fn from_integral_components<I>(r: I, g: I, b: I, a: I) -> Rgba<F>
        where I: PrimInt + Unsigned {
        Rgba::with_components(
            integral_to_float(r),
            integral_to_float(g),
            integral_to_float(b),
            integral_to_float(a))
    }

    /// Create an Rgb object from a slice of 4 unsigned primitive integers.
    pub fn from_integral_slice<I>(col: [I; 4]) -> Rgba<F>
        where I: PrimInt + Unsigned {
        Rgba::with_components(
            integral_to_float(col[0]),
            integral_to_float(col[1]),
            integral_to_float(col[2]),
            integral_to_float(col[3]))
    }
}

impl_arith_operator! {
    impl<T: Channel> Add for Rgba<T>, where Output = Rgba<T> {
        #[doc = "Piecewise addition of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn add(self, other) {
            (self.rgb + other.rgb).rgba(self.a + other.a)
        }
    }
}


impl_arith_operator! {
    impl<T: Channel> Div for Rgba<T>, where Output = Rgba<T> {
        #[doc = "Piecewise division of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn div(self, other) {
            (self.rgb / other.rgb).rgba(self.a / other.a)
        }
    }
}

impl_arith_operator! {
    impl<T: Channel> Mul for Rgba<T>, where Output = Rgba<T> {
        #[doc = "Piecewise multiplication of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn mul(self, other) {
            (self.rgb * other.rgb).rgba(self.a * other.a)
        }
    }
}

impl_arith_operator!{
    impl<T: Channel> Sub for Rgba<T>, where Output = Rgba<T> {
        #[doc = "Piecewise subtraction of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn sub(self, other) {
            (self.rgb * other.rgb).rgba(self.a * other.a)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_creation_no_clamping() {
        let col = Rgba::with_components(0.5f32, 0.2, 0.2, 0.7);
        assert_col_components_are!(col => (0.5, 0.2, 0.2, 0.7));
    }
    #[test]
    fn addition_no_clamping_effects() {
        let col_a = Rgba::with_components(0.2f32, 0.2, 0.3, 0.3);
        let col_b = Rgba::with_components(0.3f32, 0.3, 0.2, 0.2);
    
        let col_c = col_a + col_b;
        assert_col_components_are!(col_c => (0.5, 0.5, 0.5, 0.5));
    }
}