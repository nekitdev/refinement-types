//! Predicates for integers.

use core::fmt;

use paste::paste;
use thiserror::Error;

use crate::{
    core::Predicate,
    logic::{And, Not},
};

macro_rules! inverse {
    (<) => {
        ">="
    };
    (<=) => {
        ">"
    };
    (>) => {
        "<="
    };
    (>=) => {
        "<"
    };
    (==) => {
        "!="
    };
    (!=) => {
        "=="
    };
}

macro_rules! human {
    (<) => {
        "less than"
    };
    (<=) => {
        "less than or equal to"
    };
    (>) => {
        "greater than"
    };
    (>=) => {
        "greater than or equal to"
    };
    (==) => {
        "equal to"
    };
    (!=) => {
        "not equal to"
    };
}

macro_rules! human_inverse {
    (<) => {
        "greater than or equal to"
    };
    (<=) => {
        "greater than"
    };
    (>) => {
        "less than or equal to"
    };
    (>=) => {
        "less than"
    };
    (==) => {
        "not equal to"
    };
    (!=) => {
        "equal to"
    };
}

macro_rules! cmp {
    ($type: ty, $prefix: ident, $name: ident, $operation: tt) => {
        paste! {
            #[doc = concat!(
                "Represents errors that occur when provided value is ",
                human_inverse!($operation),
                " some bound.",
            )]
            #[derive(Debug, Error)]
            #[error("received value {operation} {against}", operation = inverse!($operation))]
            pub struct [< $prefix $name Error >] {
                /// The value against which the check was performed (the `N`).
                pub against: $type,
            }

            impl [< $prefix $name Error >] {
                /// Constructs [`Self`].
                pub const fn new(against: $type) -> Self {
                    Self { against }
                }
            }

            #[doc = concat!("Checks whether the given value is ", human!($operation), " `N`.")]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
            pub struct [< $prefix $name >]<const N: $type>;

            impl<const N: $type> Predicate<$type> for [< $prefix $name >]<N> {
                type Error = [< $prefix $name Error >];

                fn check(value: &$type) -> Result<(), Self::Error> {
                    if *value $operation N {
                        Ok(())
                    } else {
                        Err(Self::Error::new(N))
                    }
                }

                fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(formatter, "int {operation} {N}", operation = stringify!($operation))
                }
            }
        }
    };
}

macro_rules! lt {
    ($type: ty, $prefix: ident) => {
        cmp!($type, $prefix, Lt, <);
    };
}

macro_rules! le {
    ($type: ty, $prefix: ident) => {
        cmp!($type, $prefix, Le, <=);
    };
}

macro_rules! gt {
    ($type: ty, $prefix: ident) => {
        cmp!($type, $prefix, Gt, >);
    };
}

macro_rules! ge {
    ($type: ty, $prefix: ident) => {
        cmp!($type, $prefix, Ge, >=);
    };
}

macro_rules! eq {
    ($type: ty, $prefix: ident) => {
        cmp!($type, $prefix, Eq, ==);
    };
}

macro_rules! ne {
    ($type: ty, $prefix: ident) => {
        cmp!($type, $prefix, Ne, !=);
    };
}

macro_rules! comparisons {
    ($type: ty, $prefix: ident) => {
        lt!($type, $prefix);
        le!($type, $prefix);
        gt!($type, $prefix);
        ge!($type, $prefix);
        eq!($type, $prefix);
        ne!($type, $prefix);
    };
}

macro_rules! interval {
    ($type: ty, $prefix: ident, $name: ident, $start: ident, $end: ident, $interval: literal) => {
        paste! {
            #[doc = concat!("Represents ", $interval, " intervals.")]
            pub type [< $prefix $name >]<const M: $type, const N: $type> = And<[< $prefix $start >]<M>, [< $prefix $end >]<N>>;
        }
    };
}

macro_rules! open {
    ($type: ty, $prefix: ident) => {
        interval!($type, $prefix, Open, Gt, Lt, "`(M, N)`");
    };
}

macro_rules! closed_open {
    ($type: ty, $prefix: ident) => {
        interval!($type, $prefix, ClosedOpen, Ge, Lt, "`[M, N)`");
    };
}

macro_rules! open_closed {
    ($type: ty, $prefix: ident) => {
        interval!($type, $prefix, OpenClosed, Gt, Le, "`(M, N]`");
    };
}

macro_rules! closed {
    ($type: ty, $prefix: ident) => {
        interval!($type, $prefix, Closed, Ge, Le, "`[M, N]`");
    };
}

macro_rules! intervals {
    ($type: ty, $prefix: ident) => {
        open!($type, $prefix);
        closed_open!($type, $prefix);
        open_closed!($type, $prefix);
        closed!($type, $prefix);
    };
}

macro_rules! zeros {
    ($type: ty, $prefix: ident) => {
        paste! {
            /// Checks whether the given value is zero.
            pub type [< $prefix Zero >] = [< $prefix Eq >]<0>;

            /// Checks whether the given value is non-zero.
            pub type [< $prefix NonZero >] = [< $prefix Ne >]<0>;
        }
    };
}

macro_rules! div_mod {
    ($type: ty, $prefix: ident) => {
        paste! {
            /// Represents errors when the provided value divided by [`divisor`] does not equal [`modulo`].
            ///
            /// [`divisor`]: Self::divisor
            /// [`modulo`]: Self::modulo
            #[derive(Debug, Error)]
            #[error("received value % {divisor} != {modulo}")]
            pub struct [< $prefix Mod Error >] {
                /// The divisor that the value should be divided by (the `D`).
                pub divisor: $type,
                /// The expected modulo of the division (the `M`).
                pub modulo: $type,
            }

            impl [< $prefix Mod Error >] {
                /// Constructs [`Self`].
                pub const fn new(divisor: $type, modulo: $type) -> Self {
                    Self { divisor, modulo }
                }
            }

            /// Checks whether the given value divided by `D` has modulo `M`.
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
            pub struct [< $prefix Mod >]<const D: $type, const M: $type>;

            impl<const D: $type, const M: $type> Predicate<$type> for [< $prefix Mod >]<D, M> {
                type Error = [< $prefix Mod Error >];

                fn check(value: &$type) -> Result<(), Self::Error> {
                    if *value % D == M {
                        Ok(())
                    } else {
                        Err(Self::Error::new(D, M))
                    }
                }

                fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(formatter, "int % {D} == {M}")
                }
            }

            /// Checks whether the given value is divisible by `D`.
            pub type [< $prefix Div >]<const D: $type> = [< $prefix Mod >]<D, 0>;

            /// Checks whether the given value is even.
            pub type [< $prefix Even >] = [< $prefix Div >]<2>;

            /// Checks whether the given value is odd.
            pub type [< $prefix Odd >] = Not<[< $prefix Even >]>;
        }
    };
}

macro_rules! common_predicates {
    ($type: ty, $prefix: ident) => {
        comparisons!($type, $prefix);

        intervals!($type, $prefix);

        zeros!($type, $prefix);

        div_mod!($type, $prefix);
    };
}

macro_rules! unsigned_predicates {
    ($type: ty, $prefix: ident) => {
        common_predicates!($type, $prefix);
    };
}

macro_rules! signed {
    ($type: ty, $prefix: ident) => {
        paste! {
            /// Checks whether the given value is positive.
            pub type [< $prefix Positive >] = [< $prefix Gt >]<0>;

            /// Checks whether the given value is negative.
            pub type [< $prefix Negative >] = [< $prefix Lt >]<0>;

            /// Checks whether the given value is non-positive (negative or zero).
            pub type [< $prefix NonPositive >] = [< $prefix Le >]<0>;

            /// Checks whether the given value is non-negative (positive or zero).
            pub type [< $prefix NonNegative >] = [< $prefix Ge >]<0>;
        }
    };
}

macro_rules! signed_predicates {
    ($type: ty, $prefix: ident) => {
        common_predicates!($type, $prefix);

        signed!($type, $prefix);
    };
}

unsigned_predicates!(u8, U8);
unsigned_predicates!(u16, U16);
unsigned_predicates!(u32, U32);
unsigned_predicates!(u64, U64);
unsigned_predicates!(u128, U128);
unsigned_predicates!(usize, Usize);

signed_predicates!(i8, I8);
signed_predicates!(i16, I16);
signed_predicates!(i32, I32);
signed_predicates!(i64, I64);
signed_predicates!(i128, I128);
signed_predicates!(isize, Isize);
