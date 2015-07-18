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

use num::{Float, NumCast, PrimInt, Unsigned, Zero};
use std::default::Default;
use std::ops::{Add, Div, Mul, Sub};

use channel::Channel;
use super::{
    clamp,
    integral_to_float,
    Rgba,
    Color
};

/// An Rgb color with 3 channels: red, green and blue. All
/// channels are always normalised.
#[repr(C)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub struct Rgb<T: Channel> {
    /// Red component
    r: T,

    /// Green component
    g: T,

    /// Blue component
    b: T
}

impl<T: Channel> Rgb<T> {
    /// Construct an Rgb color from a 3-length slice of Channels.
    /// Each component is clamped between zero and one.
    pub const fn from_slice(col: [T; 3]) -> Rgb<T> {
        Rgb::with_components(col[0], col[1], col[2])
    }
    
    /// Return each component in a 3-element slice.
    pub const fn to_slice(&self) -> [T; 3] {
        [self.r, self.g, self.b]
    }

    /// Creates a new color, with every component set to zero (black color).
    pub fn new() -> Rgb<T> {
        Rgb::with_components(T::zero(), T::zero(), T::zero())
    }

    /// Construct an Rgb color piecewise from individual components. Each
    /// component is clamped between zero and one.
    pub const fn with_components(r: T, g: T, b: T) -> Rgb<T> {
        Rgb {
            r: r,
            g: g,
            b: b
        }
    }

    /// Returns the red channel value.
    #[inline]
    pub const fn r(&self) -> T { self.r }

    /// Returns the blue channel value.
    #[inline]
    pub const fn g(&self) -> T { self.g }

    /// Returns the green channel value.
    #[inline]
    pub const fn b(&self) -> T { self.b }

    /// Set the red channel value. The new value is clamped between zero and one.
    #[inline]
    pub fn set_r(&mut self, r: T) { self.r = r; }

    /// Set the green channel value. The new value is clamped between zero and one.
    #[inline]
    pub fn set_g(&mut self, g: T) { self.g = g; }

    /// Set the blue channel value. The new value is clamped between zero and one.
    #[inline]
    pub fn set_b(&mut self, b: T) { self.b = b; }

    /// Create an Rgba color from this color, using the supplied alpha.
    pub const fn rgba(&self, a: T) -> Rgba<T> {
        Rgba::with_components(self.r, self.g, self.b, a)
    }

    /// Return each component in a 3-element tuple. Useful for destructuring.
    ///
    /// ```rust
    /// let nice_shade_of_blue = Rgb::with_components(0.3f32, 0.3, 0.67);
    /// let (r, g, b) = nice_shade_of_blue.components();
    ///
    /// assert_eq!(r, 0.3f32);
    /// assert_eq!(g, 0.3f32);
    /// assert_eq!(b, 0.67f32);
    /// ```
    pub const fn components(&self) -> (T, T, T) {
        (self.r, self.g, self.b)
    }
}

impl<F: Channel + Float + NumCast> Rgb<F> {
    /// Create an Rgb object from 3 unsigned primitive integers.
    /// The value of each component is the same as the percentage
    /// of the integral value between I::zero() and I::max_value().
    pub fn from_integral_components<I>(r: I, g: I, b: I) -> Rgb<F>
        where I: PrimInt + Unsigned {
        Rgb::with_components(
            integral_to_float(r),
            integral_to_float(g),
            integral_to_float(b))
    }

    /// Create an Rgb object from a slice of 3 unsigned primitive integers.
    pub fn from_integral_slice<I>(col: [I; 3]) -> Rgb<F>
        where I: PrimInt + Unsigned {
        Rgb::with_components(
            integral_to_float(col[0]),
            integral_to_float(col[1]),
            integral_to_float(col[2]))
    }
}

impl<T: Channel> Color<T> for Rgb<T> {
    /// Clamp each component between two scalar values.
    fn clamp_scalar(&self, min: T, max: T) -> Rgb<T> {
        Rgb::with_components(
            clamp(self.r(), min, max),
            clamp(self.g(), min, max),
            clamp(self.b(), min, max))
    }

    /// Clamp each component piecewise between zero, and the
    /// corresponding channel for the other color.
    fn clamp_color(&self, min: &Rgb<T>, max: &Rgb<T>) -> Rgb<T> {
        Rgb::with_components(
            clamp(self.r(), min.r(), max.r()),
            clamp(self.g(), min.g(), max.g()),
            clamp(self.b(), min.b(), max.b()))
    }

    /// Invert the color.
    fn normalise(&self) -> Rgb<T> {
        Rgb::with_components(
            self.r.normalised(),
            self.g.normalised(),
            self.b.normalised())
    }
    
    /// Invert the color.
    fn invert(&self) -> Rgb<T> {
        Rgb::with_components(
            self.r.inverted(),
            self.g.inverted(),
            self.b.inverted())
    }

    /// Get the relative brightness of a color.
    fn luminance(&self) -> T {
        T::zero() // TODO: implement
    }

    /// Mix two colors together using the standard Rgb
    /// color model.
    fn mix(&self, other: &Rgb<T>) -> Rgb<T> {
        self + other
    }

    /// Convert a color to greyscale.
    fn to_greyscale(&self) -> Rgb<T> {
        self.clone() // TODO: implementt
    }
}

impl<T: Channel> Default for Rgb<T> {
    /// Identical to ```Rgb::new()```.
    fn default() -> Rgb<T> { Rgb::new() }
}

impl_arith_operator! {
    impl<T: Channel> Add for Rgb<T>, where Output = Rgb<T> {
        #[doc = "Piecewise addition of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn add(self, other) {
            Rgb::with_components(
                self.r() + other.r(),
                self.g() + other.g(),
                self.b() + other.b())
        }
    }
}

impl_arith_operator! {
    impl<T: Channel> Div for Rgb<T>, where Output = Rgb<T> {
        #[doc = "Piecewise division of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn div(self, other) {
            Rgb::with_components(
                self.r() / other.r(),
                self.g() / other.g(),
                self.b() / other.b())
        }
    }
}

impl_arith_operator! {
    impl<T: Channel> Mul for Rgb<T>, where Output = Rgb<T> {
        #[doc = "Piecewise multiplication of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn mul(self, other) {
            Rgb::with_components(
                self.r() * other.r(),
                self.g() * other.g(),
                self.b() * other.b())
        }
    }
}

impl_arith_operator!{
    impl<T: Channel> Sub for Rgb<T>, where Output = Rgb<T> {
        #[doc = "Piecewise subtraction of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn sub(self, other) {
            Rgb::with_components(
            self.r() - other.r(),
            self.g() - other.g(),
            self.b() - other.b())
        }
    }
}
/*
impl_arith_operator! {
    impl<T: Channel> Add for {Rgb<T>, T} where Output = Rgb<T> {
        #[doc = "Piecewise addition of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn add(self, other) {
            Rgb::with_components(
                self.r() + other,
                self.g() + other,
                self.b() + other)
        }
    }
}

impl_arith_operator! {
    impl<T: Channel> Div for {Rgb<T>, T} where Output = Rgb<T> {
        #[doc = "Piecewise division of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn div(self, other) {
            Rgb::with_components(
                self.r / other,
                self.g / other,
                self.b / other)
        }
    }
}

impl_arith_operator! {
    impl<T: Channel> Mul for {Rgb<T>, T} where Output = Rgb<T> {
        #[doc = "Piecewise multiplication of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn mul(self, other) {
            Rgb::with_components(
                self.r * other,
                self.g * other,
                self.b * other)
        }
    }
}

impl_arith_operator!{
    impl<T: Channel> Sub for {Rgb<T>, T} where Output = Rgb<T> {
        #[doc = "Piecewise subtraction of each component. Each channel
                 of the result is clamped between zero and one."]
        #[inline]
        fn sub(self, other) {
            Rgb::with_components(
                self.r - other,
                self.g - other,
                self.b - other)
        }
    }
}
*/
#[cfg(test)]
mod test {
    use super::*;
    use num::{Float, Num, Zero};
    use ::test::is_between;
    use ::{Channel, Color};
    use quickcheck::{quickcheck, TestResult};
    use quickcheck::{Arbitrary, Gen};

    impl<T: Channel + Arbitrary> Arbitrary for Rgb<T>
        where <T as Num>::FromStrRadixErr: 'static {
        fn arbitrary<G: Gen>(g: &mut G) -> Rgb<T> {
            Rgb::with_components(
                    T::arbitrary(g),
                    T::arbitrary(g),
                    T::arbitrary(g))
        }
    }

/*
    #[test]
    fn test_color_greyscale_conversion() {
        fn prop_components_are_equal_on_greyscale_conversion<T: Channel>(col: Rgb<T>) -> TestResult {
            let gscale = col.to_greyscale();
            TestResult::from_bool(
                (gscale.r() == gscale.g()) &&
                (gscale.g() == gscale.b()) &&
                (gscale.b() == gscale.r()))
        }
        quickcheck(prop_components_are_equal_on_greyscale_conversion::<f64> as fn(Rgb<f64>) -> TestResult)
    }
*/

    #[test]
    fn test_color_addition_works() {
        fn prop_components_are_added_pieceiwise<T: Channel>(col1: Rgb<T>, col2: Rgb<T>) -> TestResult {
            let result = col1 + col2;
            TestResult::from_bool(
                result.r() == col1.r() + col2.r() &&
                result.g() == col1.g() + col2.g() &&
                result.b() == col1.b() + col2.b())
        }
        quickcheck(prop_components_are_added_pieceiwise::<f64> as fn(Rgb<f64>, Rgb<f64>) -> TestResult);
        quickcheck(prop_components_are_added_pieceiwise::<u32> as fn(Rgb<u32>, Rgb<u32>) -> TestResult);
    }

    #[test]
    fn test_color_division_works() {
        fn prop_components_are_divided_pieceiwise<T: Channel>(col1: Rgb<T>, col2: Rgb<T>) -> TestResult {
            if col2.r() == T::zero() || col2.g() == T::zero() || col2.b() == T::zero() {
                return TestResult::discard(); // avoid divide by zero errors
            }
            let result = col1 / col2;
            TestResult::from_bool(
                result.r() == col1.r() / col2.r() &&
                result.g() == col1.g() / col2.g() &&
                result.b() == col1.b() / col2.b())
        }
        quickcheck(prop_components_are_divided_pieceiwise::<f64> as fn(Rgb<f64>, Rgb<f64>) -> TestResult);
        quickcheck(prop_components_are_divided_pieceiwise::<u32> as fn(Rgb<u32>, Rgb<u32>) -> TestResult);
    }

    #[test]
    fn test_color_multiplication_works() {
        fn prop_components_are_multiplied_pieceiwise<T: Channel>(col1: Rgb<T>, col2: Rgb<T>) -> TestResult {
            let result = col1 * col2;
            TestResult::from_bool(
                result.r() == col1.r() * col2.r() &&
                result.g() == col1.g() * col2.g() &&
                result.b() == col1.b() * col2.b())
        }
        quickcheck(prop_components_are_multiplied_pieceiwise::<f64> as fn(Rgb<f64>, Rgb<f64>) -> TestResult);
        quickcheck(prop_components_are_multiplied_pieceiwise::<u32> as fn(Rgb<u32>, Rgb<u32>) -> TestResult);
    }

    #[test]
    fn test_color_subtraction_works() {
        fn prop_components_are_subtracted_pieceiwise<T: Channel>(col1: Rgb<T>, col2: Rgb<T>) -> TestResult {
            if (col1.r() < col2.r()) || (col1.g() < col2.g()) || (col1.b() < col2.b()) {
                return TestResult::discard(); // Avoid overflows
            }
            let result = col1 - col2;
            TestResult::from_bool(
                result.r() == col1.r() - col2.r() &&
                result.g() == col1.g() - col2.g() &&
                result.b() == col1.b() - col2.b())
        }
        quickcheck(prop_components_are_subtracted_pieceiwise::<f64> as fn(Rgb<f64>, Rgb<f64>) -> TestResult);
        quickcheck(prop_components_are_subtracted_pieceiwise::<u32> as fn(Rgb<u32>, Rgb<u32>) -> TestResult);
    }

    #[test]
    fn test_floating_color_normalisation_works() {
        fn prop_normalised_color_components_are_between_zero_and_one<T: Channel + Float>(col: Rgb<T>) -> TestResult {
            let normalised = col.normalise();
            TestResult::from_bool(
                is_between(normalised.r(), T::zero(), T::one()) &&
                is_between(normalised.g(), T::zero(), T::one()) &&
                is_between(normalised.b(), T::zero(), T::one()))
        }
        quickcheck(prop_normalised_color_components_are_between_zero_and_one::<f32> as fn(Rgb<f32>) -> TestResult);
        quickcheck(prop_normalised_color_components_are_between_zero_and_one::<f64> as fn(Rgb<f64>) -> TestResult);
    }

    #[test]
    fn test_color_scalar_clamping_works() {
        fn prop_color_components_clamped_to_scalar_are_no_higher_than_scalar<T: Channel>(col: Rgb<T>,
                                                                                         min: T,
                                                                                         max: T) -> TestResult {
            if min > max {return TestResult::discard();}
            let clamped = col.clamp_scalar(min, max);
            TestResult::from_bool(
                is_between(clamped.r(), min, max) &&
                is_between(clamped.g(), min, max) &&
                is_between(clamped.b(), min, max))
        }
        quickcheck(prop_color_components_clamped_to_scalar_are_no_higher_than_scalar::<f32>
                   as fn(Rgb<f32>, f32, f32) -> TestResult);
        quickcheck(prop_color_components_clamped_to_scalar_are_no_higher_than_scalar::<u8>
                   as fn(Rgb<u8>, u8, u8) -> TestResult);
    }

    #[test]
    fn test_color_color_clamping_works() {
        fn prop_color_components_clamped_to_color_is_no_higher_than_other_color_components<T: Channel>(
                col: Rgb<T>,
                min: Rgb<T>,
                max: Rgb<T>) -> TestResult {
            if min.r() > max.r() || min.g() > max.g() || min.b() > max.b() {
                return TestResult::discard();
            }
            let clamped = col.clamp_color(&min, &max);
            TestResult::from_bool(
                is_between(clamped.r(), min.r(), max.r()) &&
                is_between(clamped.g(), min.g(), max.g()) &&
                is_between(clamped.b(), min.b(), max.b()))
        }
        quickcheck(prop_color_components_clamped_to_color_is_no_higher_than_other_color_components::<f32>
                   as fn(Rgb<f32>, Rgb<f32>, Rgb<f32>) -> TestResult);
        quickcheck(prop_color_components_clamped_to_color_is_no_higher_than_other_color_components::<u8>
                   as fn(Rgb<u8>, Rgb<u8>, Rgb<u8>) -> TestResult);
    }
}