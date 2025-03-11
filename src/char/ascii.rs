//! ASCII character predicates.

use core::{fmt, marker::PhantomData};

#[cfg(feature = "diagnostics")]
use miette::Diagnostic;

use thiserror::Error;

use crate::{char::macros::predicate, core::Predicate};

/// Represents integer base for checks.
pub type Base = u32;

/// Non-digit character encountered in the given [`base`].
///
/// [`base`]: Self::base
#[derive(Debug, Error)]
#[error("non-digit character in base `{base}`")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(
        code(char::digit),
        help("make sure the character is base `{base}` digit")
    )
)]
pub struct DigitError {
    /// The base in which the non-digit character was encountered.
    pub base: Base,
}

impl DigitError {
    /// Constructs [`Self`].
    #[must_use]
    pub const fn new(base: Base) -> Self {
        Self { base }
    }
}

/// Checks whether the given character is a digit in the specified base `B`.
pub struct Digit<const B: Base = 10> {
    private: PhantomData<()>,
}

/// Checks whether the given character is an octal digit.
pub type OctDigit = Digit<8>;

/// Checks whether the given character is a hexadecimal digit.
pub type HexDigit = Digit<16>;

impl<const B: Base> Predicate<char> for Digit<B> {
    type Error = DigitError;

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

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "char::digit<{B}>")
    }
}

predicate! {
    Name = Alphabetic,
    Check = is_ascii_alphabetic,
    Doc = "Checks whether the given character is ASCII alphabetic.",
    Expected = "ascii alphabetic character",
    Code = char::ascii::alphabetic,
    Error = "Non-ASCII-alphabetic character encountered.",
    Message = "non-ascii-alphabetic character",
    Help = "make sure the character is ascii alphabetic",
}

predicate! {
    Name = Alphanumeric,
    Check = is_ascii_alphanumeric,
    Doc = "Checks whether the given character is ASCII alphanumeric.",
    Expected = "ascii alphanumeric character",
    Code = char::ascii::alphanumeric,
    Error = "Non-ASCII-alphanumeric character encountered.",
    Message = "non-ascii-alphanumeric character",
    Help = "make sure the character is ascii alphanumeric",
}

predicate! {
    Name = Control,
    Check = is_ascii_control,
    Doc = "Checks whether the given character is ASCII control.",
    Expected = "ascii control character",
    Code = char::ascii::control,
    Error = "Non-ASCII-control character encountered.",
    Message = "non-ascii-control character",
    Help = "make sure the character is ascii control",
}

predicate! {
    Name = Graphic,
    Check = is_ascii_graphic,
    Doc = "Checks whether the given character is ASCII graphic.",
    Expected = "ascii graphic character",
    Code = char::ascii::graphic,
    Error = "Non-ASCII-graphic character encountered.",
    Message = "non-ascii-graphic character",
    Help = "make sure the character is ascii graphic",
}

predicate! {
    Name = Punctuation,
    Check = is_ascii_punctuation,
    Doc = "Checks whether the given character is ASCII punctuation.",
    Expected = "ascii punctuation character",
    Code = char::ascii::punctuation,
    Error = "Non-ASCII-punctuation character encountered.",
    Message = "non-ascii-punctuation character",
    Help = "make sure the character is ascii punctuation",
}

predicate! {
    Name = Lowercase,
    Check = is_ascii_lowercase,
    Doc = "Checks whether the given character is ASCII lowercase.",
    Expected = "ascii lowercase character",
    Code = char::ascii::lowercase,
    Error = "Non-ASCII-lowercase character encountered.",
    Message = "non-ascii-lowercase character",
    Help = "make sure the character is ascii lowercase",
}

predicate! {
    Name = Uppercase,
    Check = is_ascii_uppercase,
    Doc = "Checks whether the given character is ASCII uppercase.",
    Expected = "ascii uppercase character",
    Code = char::ascii::uppercase,
    Error = "Non-ASCII-uppercase character encountered.",
    Message = "non-ascii-uppercase character",
    Help = "make sure the character is ascii uppercase",
}

predicate! {
    Name = Whitespace,
    Check = is_ascii_whitespace,
    Doc = "Checks whether the given character is ASCII whitespace.",
    Expected = "ascii whitespace character",
    Code = char::ascii::whitespace,
    Error = "Non-ASCII-whitespace character encountered.",
    Message = "non-ascii-whitespace character",
    Help = "make sure the character is ascii whitespace",
}
