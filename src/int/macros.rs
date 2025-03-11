pub(crate) mod import {
    pub use core::{fmt, marker::PhantomData};

    pub use paste::paste;
    pub use thiserror::Error;

    #[cfg(feature = "diagnostics")]
    pub use miette::Diagnostic;
}

macro_rules! string_inverse {
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

pub(crate) use string_inverse;

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

pub(crate) use human;

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

pub(crate) use human_inverse;

macro_rules! reference {
    ($int: ty) => {
        concat!("[`prim@", stringify!($int), "`]")
    };
}

pub(crate) use reference;

macro_rules! compare {
    ($int: ty => $name: ident [$code: ident] ($operation: tt)) => {
        $crate::int::macros::import::paste! {
            #[doc = concat!(
                "Represents errors that occur when the provided ",
                $crate::int::macros::reference!($int),
                " is ",
                $crate::int::macros::human_inverse!($operation),
                " [`other`](Self::other).",
            )]
            #[derive(Debug, $crate::int::macros::import::Error)]
            #[error(
                "received {int} {inverse} {other}",
                int = stringify!($int),
                inverse = $crate::int::macros::string_inverse!($operation)
            )]
            #[cfg_attr(
                feature = "diagnostics",
                derive($crate::int::macros::import::Diagnostic),
                diagnostic(
                    code(int::$int::$code),
                    help(
                        "make sure the value is {human} {other}",
                        human = $crate::int::macros::human!($operation)
                    )
                )
            )]
            pub struct [< $name Error >] {
                /// The other value (the `N`).
                pub other: $int,
            }

            impl [< $name Error >] {
                /// Constructs [`Self`].
                pub const fn new(other: $int) -> Self {
                    Self { other }
                }
            }

            #[doc = concat!(
                "Checks whether the given value is ",
                $crate::int::macros::human!($operation),
                " `N`."
            )]
            pub struct $name<const N: $int> {
                private: $crate::int::macros::import::PhantomData<()>,
            }

            impl<const N: $int> $crate::core::Predicate<$int> for $name<N> {
                type Error = [< $name Error >];

                fn check(value: &$int) -> Result<(), Self::Error> {
                    if *value $operation N {
                        Ok(())
                    } else {
                        Err(Self::Error::new(N))
                    }
                }

                fn expect(
                    formatter: &mut $crate::int::macros::import::fmt::Formatter<'_>
                ) -> $crate::int::macros::import::fmt::Result {
                    write!(
                        formatter,
                        "{int} {operation} {N}",
                        int = stringify!($int),
                        operation = stringify!($operation)
                    )
                }

                fn expect_code(
                    formatter: &mut $crate::int::macros::import::fmt::Formatter<'_>
                ) -> $crate::int::macros::import::fmt::Result {
                    write!(
                        formatter,
                        "{int}::{code}<{N}>",
                        int = stringify!($int),
                        code = stringify!($code)
                    )
                }
            }
        }
    };
}

pub(crate) use compare;

macro_rules! less {
    ($int: ty) => {
        $crate::int::macros::compare!($int => Less [lt] (<));
    }
}

pub(crate) use less;

macro_rules! less_or_equal {
    ($int: ty) => {
        $crate::int::macros::compare!($int => LessOrEqual [le] (<=));
    }
}

pub(crate) use less_or_equal;

macro_rules! greater {
    ($int: ty) => {
        $crate::int::macros::compare!($int => Greater [gt] (>));
    }
}

pub(crate) use greater;

macro_rules! greater_or_equal {
    ($int: ty) => {
        $crate::int::macros::compare!($int => GreaterOrEqual [ge] (>=));
    }
}

pub(crate) use greater_or_equal;

macro_rules! equal {
    ($int: ty) => {
        $crate::int::macros::compare!($int => Equal [eq] (==));
    }
}

pub(crate) use equal;

macro_rules! not_equal {
    ($int: ty) => {
        $crate::int::macros::compare!($int => NotEqual [ne] (!=));
    }
}

pub(crate) use not_equal;

macro_rules! comparing {
    ($int: ty) => {
        $crate::int::macros::less!($int);
        $crate::int::macros::less_or_equal!($int);
        $crate::int::macros::greater!($int);
        $crate::int::macros::greater_or_equal!($int);
        $crate::int::macros::equal!($int);
        $crate::int::macros::not_equal!($int);
    };
}

pub(crate) use comparing;

macro_rules! interval {
    (
        $int: ty => $name: ident<
            $left: ident as $open: literal, $right: ident as $close: literal
        >
    ) => {
        #[doc = concat!("Represents `", $open, "M, N", $close, "` intervals.")]
        pub type $name<const M: $int, const N: $int> = $crate::logic::And<$left<M>, $right<N>>;
    };
}

pub(crate) use interval;

macro_rules! open {
    ($int: ty) => {
        $crate::int::macros::interval!($int => Open<Greater as "(", Less as ")">);
    };
}

pub(crate) use open;

macro_rules! open_closed {
    ($int: ty) => {
        $crate::int::macros::interval!($int => OpenClosed<Greater as "(", LessOrEqual as "]">);
    };
}

pub(crate) use open_closed;

macro_rules! closed_open {
    ($int: ty) => {
        $crate::int::macros::interval!($int => ClosedOpen<GreaterOrEqual as "[", Less as ")">);
    };
}

pub(crate) use closed_open;

macro_rules! closed {
    ($int: ty) => {
        $crate::int::macros::interval!($int => Closed<GreaterOrEqual as "[", LessOrEqual as "]">);
    };
}

pub(crate) use closed;

macro_rules! intervals {
    ($int: ty) => {
        $crate::int::macros::open!($int);
        $crate::int::macros::open_closed!($int);
        $crate::int::macros::closed_open!($int);
        $crate::int::macros::closed!($int);
    };
}

pub(crate) use intervals;

macro_rules! zeros {
    ($int: ty) => {
        /// Checks whether the given value is equal to zero (`0`).
        pub type Zero = Equal<0>;

        /// Checks whether the given value is not equal to zero (`0`).
        pub type NonZero = NotEqual<0>;
    };
}

pub(crate) use zeros;

macro_rules! modulo {
    ($int: ty) => {
        #[doc = concat!(
            "Represents errors that occur when the provided ",
            $crate::int::macros::reference!($int),
            " divided by [`divisor`](Self::divisor) does not equal [`modulo`](Self::modulo).",
        )]
        #[derive(Debug, $crate::int::macros::import::Error)]
        #[error("received {int} % {divisor} != {modulo}", int = stringify!($int))]
        #[cfg_attr(
            feature = "diagnostics",
            derive($crate::int::macros::import::Diagnostic),
            diagnostic(
                code(int::$int::modulo),
                help("make sure the value divided by {divisor} and has modulo {modulo}")
            )
        )]
        pub struct ModuloError {
            /// The divisor used (the `D`).
            pub divisor: $int,
            /// The expected modulo (the `M`).
            pub modulo: $int,
        }

        impl ModuloError {
            /// Constructs [`Self`].
            pub const fn new(divisor: $int, modulo: $int) -> Self {
                Self { divisor, modulo }
            }
        }

        #[doc = concat!(
            "Checks whether ", $crate::int::macros::reference!($int), " divided by `D` has modulo `M`."
        )]
        pub struct Modulo<const D: $int, const M: $int> {
            private: $crate::int::macros::import::PhantomData<()>,
        }

        impl<const D: $int, const M: $int> $crate::core::Predicate<$int> for Modulo<D, M> {
            type Error = ModuloError;

            fn check(value: &$int) -> Result<(), Self::Error> {
                if *value % D == M {
                    Ok(())
                } else {
                    Err(Self::Error::new(D, M))
                }
            }

            fn expect(
                formatter: &mut $crate::int::macros::import::fmt::Formatter<'_>,
            ) -> $crate::int::macros::import::fmt::Result {
                write!(formatter, "{int} % {D} == {M}", int = stringify!($int))
            }

            fn expect_code(
                formatter: &mut $crate::int::macros::import::fmt::Formatter<'_>,
            ) -> $crate::int::macros::import::fmt::Result {
                write!(formatter, "{int}::mod<{D}, {M}>", int = stringify!($int))
            }
        }
    };
}

pub(crate) use modulo;

macro_rules! divisible {
    ($int: ty) => {
        /// Checks whether the given value is divisible by `D`.
        pub type Divisible<const D: $int> = Modulo<D, 0>;

        /// Checks whether the given value is even.
        pub type Even = Divisible<2>;

        /// Checks whether the given value is odd.
        pub type Odd = $crate::logic::Not<Even>;
    };
}

pub(crate) use divisible;

macro_rules! common {
    ($int: ty) => {
        $crate::int::macros::comparing!($int);
        $crate::int::macros::intervals!($int);
        $crate::int::macros::zeros!($int);
        $crate::int::macros::modulo!($int);
        $crate::int::macros::divisible!($int);
    };
}

pub(crate) use common;

macro_rules! unsigned {
    ($int: ty) => {
        $crate::int::macros::common!($int);
    };
}

pub(crate) use unsigned;

macro_rules! around {
    ($int: ty) => {
        /// Checks whether the given value is positive.
        pub type Positive = Greater<0>;

        /// Checks whether the given value is negative.
        pub type Negative = Less<0>;

        /// Checks whether the given value is non-negative (positive or zero).
        pub type NonNegative = GreaterOrEqual<0>;

        /// Checks whether the given value is non-positive (negative or zero).
        pub type NonPositive = LessOrEqual<0>;
    };
}

pub(crate) use around;

macro_rules! signed {
    ($int: ty) => {
        $crate::int::macros::common!($int);
        $crate::int::macros::around!($int);
    };
}

pub(crate) use signed;

macro_rules! unsigned_module {
    ($int: ty => $name: ident) => {
        #[doc = concat!("Predicates for ", $crate::int::macros::reference!($int), " values.")]
        pub mod $name {
            $crate::int::macros::unsigned!($int);
        }
    };
}

pub(crate) use unsigned_module;

macro_rules! signed_module {
    ($int: ty => $name: ident) => {
        #[doc = concat!("Predicates for ", $crate::int::macros::reference!($int), " values.")]
        pub mod $name {
            $crate::int::macros::signed!($int);
        }
    };
}

pub(crate) use signed_module;
