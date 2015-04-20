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

//! Crate-internal utility macros

/// Implement an arithmetic operator function for each combination of
/// object lifetimes, for a combination of types. Cannot currently be
/// used on fully generic types (```impl<T> add for...```).
///
///```
/// use num::Float;
/// struct Point<F: Float>(F, F);
///
/// impl_arith_operator! {
///     // can also be written as ```impl<X: Blah, Y: Wah> Add for (Type1<X>, Type2<Y>) where...```
///     // if the input types are different
///     impl<F: Float> Add for Point<F>, where Output = Point<F> {
///         // attributes are applied to all implementations of the function
///         #[doc = "Some documentation attribute"]
///         #[inline]
///         fn add(self, other) {
///             let (x1, y1) = self;
///             let (x2, y2) = other;
///             Point(x1 + x2, y1 + y2)
///         }
///     }
/// }
///
/// fn main() {
///     let p1 = Point(1.0f64, 5.0);
///     let p2 = &p1;
///     let p3 = p1 + p2;
///     let (x, y) = p3;
///     assert!(x == 2.0 && y == 10.0);
/// }
///```
macro_rules! impl_arith_operator {
    // base case: generics, tuple
    (impl< $($tnm:ident : $tr:ident),* > $op:ident for {$typa:ty, $typb:ty} where Output = $otyp:ty {
        $(#[$attr:meta])*
        fn $f:ident($slf:ident, $oth:ident)
            $body:block
    }) => {
        impl<$($tnm: $tr),*> $op for $typa {
            type Output = $otyp;
            $(#[$attr])*
            fn $f($slf, $oth: $typb) -> $otyp {
                $body
            }
        }

        impl<'a, $($tnm: $tr),*> $op<$typa> for &'a $typa {
            type Output = <$typa as Add<$typa>>::Output;
            $(#[$attr])*
            fn $f($slf, $oth: $typb) -> <$typa as Add<$typa>>::Output {
                $body
            }
        }

        impl<'a, $($tnm: $tr),*> $op<&'a $typa> for $typa {
            type Output = <$typa as Add<$typa>>::Output;
            $(#[$attr])*
            fn $f($slf, $oth: &'a $typb) -> <$typa as Add<$typa>>::Output {
                $body
            }
        }

        impl<'a, 'b, $($tnm: $tr),*> $op<&'a $typa> for &'b $typa {
            type Output = <$typa as Add<$typa>>::Output;
            $(#[$attr])*
            fn $f($slf, $oth: &'a $typb) -> <$typa as Add<$typa>>::Output {
                $body
            }
        }
    };

    // generics, single type
    (impl< $($tnm:ident : $tr:ident),* > $op:ident for $typ:ty, where Output = $otyp:ty {
        $(#[$attr:meta])*
        fn $f:ident($slf:ident, $oth:ident)
            $body:block
    }) => {
        impl_arith_operator! {
            impl< $($tnm : $tr),* > $op for {$typ, $typ} where Output = $otyp {
                $(#[$attr])*
                fn $f($slf, $oth)
                    $body
            }
        }
    };

    // no generics, tuple
    (impl $op:ident for ($typa:ty, $typb:ty) where Output = $otyp:ty {
        $(#[$attr:meta])*
        fn $f:ident($slf:ident, $oth:ident)
            $body:block
    }) => {
        impl_arith_operator! {
            impl<> $op for {$typa, $typb} where Output = $otyp {
                $(#[$attr])*
                fn $f($slf, $oth)
                    $body
            }
        }
    };

    // no generics, single type
    (impl $op:ident for $typ:ty, where Output = $otyp:ty {
        $(#[$attr:meta])*
        fn $f:ident($slf:ident, $oth:ident)
            $body:block
    }) => {
        impl_arith_operator! {
            impl $op for {$typ, $typ} where Output = $otyp {
                $(#[$attr])*
                fn $f($slf, $oth)
                    $body
            }
        }
    };
}

/// Assert that each component of a color object is equal to
/// the provided channels.
macro_rules! assert_col_components_are(
    (
        $col:expr => ($red:expr, $green:expr, $blue:expr)
    ) => {{
        assert_eq!($col.r(), $red);
        assert_eq!($col.g(), $green);
        assert_eq!($col.b(), $blue);
    }};

    (
        $col:expr => ($red:expr, $green:expr, $blue:expr, $alpha:expr)
    ) => {{
        assert_col_components_are!($col => ($red, $green, $blue));
        assert_eq!($col.a(), $alpha);
    }};
);