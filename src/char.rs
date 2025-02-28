//! Predicates for characters.

use core::fmt;

use paste::paste;
use thiserror::Error;

use crate::{core::Predicate, logic::And};

/// Represents base for checks.
pub type Base = u32;

/// The default base in [`IsDigit`].
pub const DEFAULT_BASE: Base = 10;

/// Non-digit character encountered in the given base.
#[derive(Debug, Error)]
#[error("non-digit character in base `{base}`")]
pub struct NonDigitError {
    /// The base in which the non-digit character was encountered.
    pub base: Base,
}

impl NonDigitError {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(base: Base) -> Self {
        Self { base }
    }
}

/// Checks whether the given character is a digit in the specified base `B`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IsDigit<const B: Base = DEFAULT_BASE>;

/// The octal base.
pub const OCT_BASE: Base = 8;

/// The hexadecimal base.
pub const HEX_BASE: Base = 16;

/// Checks whether the given character is an octal digit.
pub type IsOctDigit = IsDigit<OCT_BASE>;

/// Checks whether the given character is a hexadecimal digit.
pub type IsHexDigit = IsDigit<HEX_BASE>;

impl<const B: Base> Predicate<char> for IsDigit<B> {
    type Error = NonDigitError;

    fn check(value: &char) -> Result<(), Self::Error> {
        if value.is_digit(B) {
            Ok(())
        } else {
            Err(Self::Error::new(B))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "digit in base {B}")
    }
}

macro_rules! predicate {
    (
        Name = $name: ident,
        Check = $check: ident,
        Doc = $doc: expr,
        Error = $error: expr,
        Message = $message: expr,
        Expected = $expected: expr,
    ) => {
        paste! {
            #[derive(Debug, Error, Default)]
            #[error($message)]
            #[doc = $error]
            pub struct [<Non $name Error>];

            impl [<Non $name Error>] {
                /// Constructs [`Self`].
                pub const fn new() -> Self {
                    Self
                }
            }

            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
            #[doc = $doc]
            pub struct [< Is $name >];

            impl Predicate<char> for [< Is $name >] {
                type Error = [<Non $name Error>];

                fn check(value: &char) -> Result<(), Self::Error> {
                    if value.$check() {
                        Ok(())
                    } else {
                        Err(Self::Error::new())
                    }
                }

                fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(formatter, $expected)
                }
            }
        }
    };
}

predicate! {
    Name = Ascii,
    Check = is_ascii,
    Doc = "Checks whether the given character is within the ASCII range.",
    Error = "Non-ASCII character encountered.",
    Message = "non-ascii character",
    Expected = "ascii character",
}

predicate! {
    Name = Alphabetic,
    Check = is_alphabetic,
    Doc = "Checks whether the given character is alphabetic.",
    Error = "Non-alphabetic character encountered.",
    Message = "non-alphabetic character",
    Expected = "alphabetic character",
}

predicate! {
    Name = Alphanumeric,
    Check = is_alphanumeric,
    Doc = "Checks whether the given character is alphanumeric.",
    Error = "Non-alphanumeric character encountered.",
    Message = "non-alphanumeric character",
    Expected = "alphanumeric character",
}

predicate! {
    Name = Control,
    Check = is_control,
    Doc = "Checks whether the given character is control.",
    Error = "Non-control character encountered.",
    Message = "non-control character",
    Expected = "control character",
}

predicate! {
    Name = Numeric,
    Check = is_numeric,
    Doc = "Checks whether the given character is numeric.",
    Error = "Non-numeric character encountered.",
    Message = "non-numeric character",
    Expected = "numeric character",
}

predicate! {
    Name = Lowercase,
    Check = is_lowercase,
    Doc = "Checks whether the given character is lowercase.",
    Error = "Non-lowercase character encountered.",
    Message = "non-lowercase character",
    Expected = "lowercase character",
}

predicate! {
    Name = Uppercase,
    Check = is_uppercase,
    Doc = "Checks whether the given character is uppercase.",
    Error = "Non-uppercase character encountered.",
    Message = "non-uppercase character",
    Expected = "uppercase character",
}

predicate! {
    Name = Whitespace,
    Check = is_whitespace,
    Doc = "Checks whether the given character is whitespace.",
    Error = "Non-whitespace character encountered.",
    Message = "non-whitespace character",
    Expected = "whitespace character",
}

/// Composition of [`IsAscii`] and [`IsDigit`].
pub type IsAsciiAlphabetic = And<IsAscii, IsAlphabetic>;

/// Composition of [`IsAscii`] and [`IsAlphanumeric`].
pub type IsAsciiAlphanumeric = And<IsAscii, IsAlphanumeric>;

/// Composition of [`IsAscii`] and [`IsControl`].
pub type IsAsciiControl = And<IsAscii, IsControl>;

/// Composition of [`IsAscii`] and [`IsNumeric`].
pub type IsAsciiNumeric = And<IsAscii, IsNumeric>;

/// Composition of [`IsAscii`] and [`IsLowercase`].
pub type IsAsciiLowercase = And<IsAscii, IsLowercase>;

/// Composition of [`IsAscii`] and [`IsUppercase`].
pub type IsAsciiUppercase = And<IsAscii, IsUppercase>;

/// Composition of [`IsAscii`] and [`IsWhitespace`].
pub type IsAsciiWhitespace = And<IsAscii, IsWhitespace>;
