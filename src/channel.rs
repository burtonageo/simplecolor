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

use num::{Num, One};
use super::clamp_to_zero_one;

/// A trait which represents the numerical value
/// of a single channel of a color.
pub trait Channel: Copy + Num + PartialOrd {
    /// Invert this channel.
    fn inverted(self) -> Self;

    /// Normalise the channel. For unsigned integrals, this does nothing.
    /// For floating point channels, this clamps it between 1 and 0.
    fn normalised(self) -> Self;
}

/*
// These will come back when negative trait impls ara availible
impl<T: PrimInt + Unsigned + !Float> Channel for T {
    fn inverted(self) -> T { T::max_value() - self }
    fn normalised(self) -> T { self }
}

impl<T: Float + !PrimInt + !Unsigned> Channel for T {
    fn inverted(self) -> T { T::one() - self.normalised() }
    fn normalised(self) -> T { clamp_to_zero_one(self) }
}
*/

impl Channel for u8 {
    fn inverted(self) -> u8 { u8::max_value() - self }
    fn normalised(self) -> u8 { self }
}

impl Channel for u16 {
    fn inverted(self) -> u16 { u16::max_value() - self }
    fn normalised(self) -> u16 { self }
}

impl Channel for u32 {
    fn inverted(self) -> u32 { u32::max_value() - self }
    fn normalised(self) -> u32 { self }
}

impl Channel for u64 {
    fn inverted(self) -> u64 { u64::max_value() - self }
    fn normalised(self) -> u64 { self }
}

impl Channel for f32 {
    fn inverted(self) -> f32 { f32::one() - self.normalised() }
    fn normalised(self) -> f32 { clamp_to_zero_one(self) }
}

impl Channel for f64 {
    fn inverted(self) -> f64 { f64::one() - self.normalised() }
    fn normalised(self) -> f64 { clamp_to_zero_one(self) }
}