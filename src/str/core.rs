//! Core functionality.

use core::{fmt, marker::PhantomData};

use thiserror::Error;

use crate::{core::Predicate, static_str::StaticStr, type_str::TypeStr};

#[cfg(feature = "regex")]
use crate::type_regex::TypeRegex;

/// Represents errors that occur when the string does not start with [`prefix`].
///
/// [`prefix`]: Self::prefix
#[derive(Debug, Error)]
#[error("expected string to start with `{prefix}`")]
pub struct StartsWithError {
    /// The expected prefix.
    pub prefix: StaticStr,
}

impl StartsWithError {
    /// Constructs [`Self`].
    pub const fn new(prefix: StaticStr) -> Self {
        Self { prefix }
    }
}

/// Checks if the string starts with the specified prefix `S`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StartsWith<S: TypeStr + ?Sized> {
    prefix: PhantomData<S>,
}

impl<T: AsRef<str> + ?Sized, S: TypeStr + ?Sized> Predicate<T> for StartsWith<S> {
    type Error = StartsWithError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.as_ref().starts_with(S::VALUE) {
            Ok(())
        } else {
            Err(Self::Error::new(S::VALUE))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string starting with `{prefix}`",
            prefix = S::VALUE
        )
    }
}

/// Represents errors that occur when the string does not end with [`suffix`].
///
/// [`suffix`]: Self::suffix
#[derive(Debug, Error)]
#[error("expected string to end with `{suffix}`")]
pub struct EndsWithError {
    /// The expected suffix.
    pub suffix: StaticStr,
}

impl EndsWithError {
    /// Constructs [`Self`].
    pub const fn new(suffix: StaticStr) -> Self {
        Self { suffix }
    }
}

/// Checks if the string ends with the specified suffix `S`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EndsWith<S: TypeStr + ?Sized> {
    suffix: PhantomData<S>,
}

impl<T: AsRef<str> + ?Sized, S: TypeStr + ?Sized> Predicate<T> for EndsWith<S> {
    type Error = EndsWithError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.as_ref().ends_with(S::VALUE) {
            Ok(())
        } else {
            Err(Self::Error::new(S::VALUE))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string ending with `{suffix}`",
            suffix = S::VALUE
        )
    }
}

/// Represents errors that occur when the string does not contain [`string`].
///
/// [`string`]: Self::string
#[derive(Debug, Error)]
#[error("expected string to contain `{string}`")]
pub struct ContainsError {
    /// The expected string.
    pub string: StaticStr,
}

impl ContainsError {
    /// Constructs [`Self`].
    pub const fn new(string: StaticStr) -> Self {
        Self { string }
    }
}

/// Checks if the string contains the specified string `S`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Contains<S: TypeStr + ?Sized> {
    string: PhantomData<S>,
}

impl<T: AsRef<str> + ?Sized, S: TypeStr + ?Sized> Predicate<T> for Contains<S> {
    type Error = ContainsError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.as_ref().contains(S::VALUE) {
            Ok(())
        } else {
            Err(Self::Error::new(S::VALUE))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "string containing `{string}`", string = S::VALUE)
    }
}

/// Represents errors that occur when the string does not start with [`start`] character.
///
/// [`start`]: Self::start
#[derive(Debug, Error)]
#[error("expected string to start with `{start}`")]
pub struct StartsWithCharError {
    /// The expected starting character.
    pub start: char,
}

impl StartsWithCharError {
    /// Constructs [`Self`].
    pub const fn new(start: char) -> Self {
        Self { start }
    }
}

/// Checks if the string starts with the specified character `C`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct StartsWithChar<const C: char>;

impl<T: AsRef<str> + ?Sized, const C: char> Predicate<T> for StartsWithChar<C> {
    type Error = StartsWithCharError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.as_ref().starts_with(C) {
            Ok(())
        } else {
            Err(Self::Error::new(C))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "string starting with `{C}`")
    }
}

/// Represents errors that occur when the string does not end with [`end`] character.
///
/// [`end`]: Self::end
#[derive(Debug, Error)]
#[error("expected string to end with `{end}`")]
pub struct EndsWithCharError {
    /// The expected ending character.
    pub end: char,
}

impl EndsWithCharError {
    /// Constructs [`Self`].
    pub const fn new(end: char) -> Self {
        Self { end }
    }
}

/// Checks if the string ends with the specified character `C`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EndsWithChar<const C: char>;

impl<T: AsRef<str> + ?Sized, const C: char> Predicate<T> for EndsWithChar<C> {
    type Error = EndsWithCharError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.as_ref().ends_with(C) {
            Ok(())
        } else {
            Err(Self::Error::new(C))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "string ending with `{C}`")
    }
}

/// Represents errors that occur when the string does not contain [`character`].
///
/// [`character`]: Self::character
#[derive(Debug, Error)]
#[error("expected string to contain `{character}`")]
pub struct ContainsCharError {
    /// The expected character.
    pub character: char,
}

impl ContainsCharError {
    /// Constructs [`Self`].
    pub const fn new(character: char) -> Self {
        Self { character }
    }
}

/// Checks if the string contains the specified character `C`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ContainsChar<const C: char>;

impl<T: AsRef<str> + ?Sized, const C: char> Predicate<T> for ContainsChar<C> {
    type Error = ContainsCharError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.as_ref().contains(C) {
            Ok(())
        } else {
            Err(Self::Error::new(C))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "string containing `{C}`")
    }
}

/// Represents errors that occur when the string is not trimmed at the start.
#[derive(Debug, Error, Default)]
#[error("expected string to be trimmed at the start")]
pub struct TrimmedStartError;

impl TrimmedStartError {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self
    }
}

/// Checks if the string is trimmed at the start.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TrimmedStart;

impl<T: AsRef<str> + ?Sized> Predicate<T> for TrimmedStart {
    type Error = TrimmedStartError;

    fn check(value: &T) -> Result<(), Self::Error> {
        let string = value.as_ref();

        if string.trim() == string {
            Ok(())
        } else {
            Err(Self::Error::new())
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("string trimmed at the start")
    }
}

/// Represents errors that occur when the string is not trimmed at the end.
#[derive(Debug, Error, Default)]
#[error("expected string to be trimmed at the end")]
pub struct TrimmedEndError;

impl TrimmedEndError {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self
    }
}

/// Checks if the string is trimmed at the end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TrimmedEnd;

impl<T: AsRef<str> + ?Sized> Predicate<T> for TrimmedEnd {
    type Error = TrimmedEndError;

    fn check(value: &T) -> Result<(), Self::Error> {
        let string = value.as_ref();

        if string.trim() == string {
            Ok(())
        } else {
            Err(Self::Error::new())
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("string trimmed at the end")
    }
}

/// Represents errors that occur when the string is not trimmed.
#[derive(Debug, Error, Default)]
#[error("expected string to be trimmed")]
pub struct TrimmedError;

impl TrimmedError {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self
    }
}

/// Checks if the string is trimmed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Trimmed;

impl<T: AsRef<str> + ?Sized> Predicate<T> for Trimmed {
    type Error = TrimmedError;

    fn check(value: &T) -> Result<(), Self::Error> {
        let string = value.as_ref();

        if string.trim() == string {
            Ok(())
        } else {
            Err(Self::Error::new())
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("trimmed string")
    }
}

/// Represents errors that occur when the string is not valid ASCII.
#[derive(Debug, Error, Default)]
#[error("expected string to be ascii")]
pub struct AsciiError;

impl AsciiError {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self
    }
}

/// Checks if the string is valid ASCII.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Ascii;

impl<T: AsRef<str> + ?Sized> Predicate<T> for Ascii {
    type Error = AsciiError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.as_ref().is_ascii() {
            Ok(())
        } else {
            Err(Self::Error::new())
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("ascii string")
    }
}

/// Represents errors that occur when the string does not match the expected [`pattern`].
///
/// [`pattern`]: Self::pattern
#[cfg(feature = "regex")]
#[derive(Debug, Error)]
#[error("received string that does not match the `{pattern}` pattern")]
pub struct MismatchError {
    /// The expected pattern.
    pub pattern: StaticStr,
}

#[cfg(feature = "regex")]
impl MismatchError {
    /// Constructs [`Self`].
    pub const fn new(pattern: StaticStr) -> Self {
        Self { pattern }
    }
}

/// Checks if the string matches the specified pattern `S`.
#[cfg(feature = "regex")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Matches<S: TypeRegex + ?Sized> {
    pattern: PhantomData<S>,
}

#[cfg(feature = "regex")]
impl<T: AsRef<str> + ?Sized, S: TypeRegex + ?Sized> Predicate<T> for Matches<S> {
    type Error = MismatchError;

    fn check(value: &T) -> Result<(), Self::Error> {
        let regex = S::get();

        if regex.is_match(value.as_ref()) {
            Ok(())
        } else {
            Err(Self::Error::new(regex.as_str()))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "string matching the `{pattern}` pattern",
            pattern = S::get().as_str()
        )
    }
}
