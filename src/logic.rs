//! Logical operations on predicates.

use core::{fmt, marker::PhantomData};

#[cfg(feature = "diagnostics")]
use miette::Diagnostic;

use thiserror::Error;

use crate::{
    core::{ErrorCore, Predicate},
    static_str::StaticStr,
};

/// Represents predicates that are always satisfied.
pub struct True {
    private: PhantomData<()>,
}

/// Represents errors that are never encountered.
///
/// This is essentially `!`, the never type.
#[derive(Debug, Error)]
#[error("never errors")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(logic::never), help("this error is never returned"))
)]
pub enum NeverError {}

/// The `anything` string.
pub const ANYTHING: StaticStr = "anything";

/// The `true` string.
pub const TRUE: StaticStr = "true";

impl<T: ?Sized> Predicate<T> for True {
    type Error = NeverError;

    fn check(_value: &T) -> Result<(), Self::Error> {
        Ok(())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(ANYTHING)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(TRUE)
    }
}

/// Represents predicates that are never satisfied.
pub struct False {
    private: PhantomData<()>,
}

/// Represents errors that are always encountered.
#[derive(Debug, Error)]
#[error("always errors")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(logic::always), help("this error is always returned"))
)]
pub struct AlwaysError;

/// The `nothing` string.
pub const NOTHING: StaticStr = "nothing";

/// The `false` string.
pub const FALSE: StaticStr = "false";

impl<T: ?Sized> Predicate<T> for False {
    type Error = AlwaysError;

    fn check(_value: &T) -> Result<(), Self::Error> {
        Err(AlwaysError)
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(NOTHING)
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(FALSE)
    }
}

/// Represents errors returned by [`And`].
#[derive(Debug)]
pub enum EitherError<E, F> {
    /// Left error (`P` failed).
    Left(E),
    /// Right error (`Q` failed).
    Right(F),
}

impl<E: fmt::Display, F: fmt::Display> fmt::Display for EitherError<E, F> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left(left) => write!(formatter, "left error: {left}"),
            Self::Right(right) => write!(formatter, "right error: {right}"),
        }
    }
}

impl<E: ErrorCore + 'static, F: ErrorCore + 'static> ErrorCore for EitherError<E, F> {
    fn source(&self) -> Option<&(dyn ErrorCore + 'static)> {
        match self {
            Self::Left(left) => Some(left),
            Self::Right(right) => Some(right),
        }
    }
}

#[cfg(feature = "diagnostics")]
impl<E: Diagnostic + 'static, F: Diagnostic + 'static> Diagnostic for EitherError<E, F> {
    fn code(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new("logic::either"))
    }

    fn help(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new("make sure both predicates are satisfied"))
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::Left(left) => Some(left),
            Self::Right(right) => Some(right),
        }
    }
}

/// Represents predicates that are satisfied when both `P` and `Q` are satisfied.
pub struct And<P: ?Sized, Q: ?Sized> {
    left: PhantomData<P>,
    right: PhantomData<Q>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, Q: Predicate<T> + ?Sized> Predicate<T> for And<P, Q> {
    type Error = EitherError<P::Error, Q::Error>;

    fn check(value: &T) -> Result<(), Self::Error> {
        P::check(value)
            .map_err(Self::Error::Left)
            .and_then(|()| Q::check(value).map_err(Self::Error::Right))
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}) and ({})", P::expected(), Q::expected())
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "and<{}, {}>",
            P::expected_code(),
            Q::expected_code()
        )
    }
}

/// Represents errors returned by [`Or`].
#[derive(Debug)]
pub struct BothError<E, F> {
    /// Left error (`P` failed).
    pub left: E,
    /// Right error (`Q` failed).
    pub right: F,
}

impl<E, F> BothError<E, F> {
    /// Constructs [`Self`].
    pub const fn new(left: E, right: F) -> Self {
        Self { left, right }
    }
}

impl<E: fmt::Display, F: fmt::Display> fmt::Display for BothError<E, F> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "both errors occured: {left} and {right}",
            left = self.left,
            right = self.right
        )
    }
}

impl<E: ErrorCore, F: ErrorCore> ErrorCore for BothError<E, F> {}

#[cfg(feature = "diagnostics")]
impl<E: Diagnostic, F: Diagnostic> Diagnostic for BothError<E, F> {
    fn code(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new("logic::both"))
    }

    fn help(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new("make sure at least one predicate is satisfied"))
    }
}

/// Represents predicates that are satisfied when either `P` or `Q` (or both) are satisfied.
pub struct Or<P: ?Sized, Q: ?Sized> {
    left: PhantomData<P>,
    right: PhantomData<Q>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, Q: Predicate<T> + ?Sized> Predicate<T> for Or<P, Q> {
    type Error = BothError<P::Error, Q::Error>;

    fn check(value: &T) -> Result<(), Self::Error> {
        P::check(value)
            .or_else(|left| Q::check(value).map_err(|right| Self::Error::new(left, right)))
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}) or ({})", P::expected(), Q::expected())
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "or<{}, {}>",
            P::expected_code(),
            Q::expected_code()
        )
    }
}

/// Represents errors returned by [`Not`].
#[derive(Debug, Error, Default)]
#[error("negated error")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(logic::not), help("make sure the negated predicate is satisfied"))
)]
pub struct NotError;

impl NotError {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self
    }
}

/// Represents predicates that are satisfied when `P` is not satisfied.
pub struct Not<P: ?Sized> {
    predicate: PhantomData<P>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> Predicate<T> for Not<P> {
    type Error = NotError;

    fn check(value: &T) -> Result<(), Self::Error> {
        match P::check(value) {
            Ok(()) => Err(Self::Error::new()),
            Err(_) => Ok(()),
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "not ({})", P::expected())
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "not<{}>", P::expected_code())
    }
}

/// Represents errors returned by [`Xor`].
#[derive(Debug)]
pub enum NeitherOrBoth<E, F> {
    /// Neither error encountered.
    Neither,
    /// Both errors encountered.
    Both(BothError<E, F>),
}

impl<E: fmt::Display, F: fmt::Display> fmt::Display for NeitherOrBoth<E, F> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Neither => formatter.write_str("neither error encountered"),
            Self::Both(both) => both.fmt(formatter),
        }
    }
}

impl<E: ErrorCore + 'static, F: ErrorCore + 'static> ErrorCore for NeitherOrBoth<E, F> {
    fn source(&self) -> Option<&(dyn ErrorCore + 'static)> {
        match self {
            Self::Neither => None,
            Self::Both(both) => Some(both),
        }
    }
}

#[cfg(feature = "diagnostics")]
impl<E: Diagnostic + 'static, F: Diagnostic + 'static> Diagnostic for NeitherOrBoth<E, F> {
    fn code(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new("logic::neither_or_both"))
    }

    fn help(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new("make sure only one predicate is satisfied"))
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::Neither => None,
            Self::Both(both) => Some(both),
        }
    }
}

/// Represents predicates that are satisfied when either `P` or `Q` (but *not* both) are satisfied.
pub struct Xor<P: ?Sized, Q: ?Sized> {
    left: PhantomData<P>,
    right: PhantomData<Q>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, Q: Predicate<T> + ?Sized> Predicate<T> for Xor<P, Q> {
    type Error = NeitherOrBoth<P::Error, Q::Error>;

    fn check(value: &T) -> Result<(), Self::Error> {
        match (P::check(value), Q::check(value)) {
            (Ok(()), Ok(())) => Err(NeitherOrBoth::Neither),
            (Err(left), Err(right)) => Err(NeitherOrBoth::Both(BothError::new(left, right))),
            _ => Ok(()),
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}) xor ({})", P::expected(), Q::expected())
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "xor<{}, {}>",
            P::expected_code(),
            Q::expected_code()
        )
    }
}

/// Composes [`Not`] and [`And`].
pub type Nand<P, Q> = Not<And<P, Q>>;

/// Composes [`Not`] and [`Or`].
pub type Nor<P, Q> = Not<Or<P, Q>>;

/// Composes [`Not`] and [`Xor`].
pub type Xnor<P, Q> = Not<Xor<P, Q>>;

/// Represents predicates that are satisfied when `P` implies `Q`.
pub type Imply<P, Q> = Or<Not<P>, Q>;

/// Negates the given predicate.
///
/// For predicate `P`, `not!(P)` is [`Not<P>`].
#[macro_export]
macro_rules! not {
    ($predicate: ty) => {
        $crate::logic::Not<$predicate>
    }
}

/// Given two or more predicates, composes them together with [`And`].
///
/// For predicates `P` and `Q`, `and!(P, Q)` is [`And<P, Q>`].
///
/// For predicates `P`, `Q`, and `R`, `and!(P, Q, R)` is [`And<P, And<Q, R>>`].
///
/// Ad infinitum...
#[macro_export]
macro_rules! and {
    ($first: ty, $second: ty) => {
        $crate::logic::And<$first, $second>
    };

    ($first: ty, $second: ty, $($rest: ty),+ $(,)?) => {
        $crate::and!($first, $crate::and!($second, $($rest),+))
    }
}

/// Given two or more predicates, composes them together with [`Or`].
///
/// For predicates `P` and `Q`, `or!(P, Q)` is [`Or<P, Q>`].
///
/// For predicates `P`, `Q`, and `R`, `or!(P, Q, R)` is [`Or<P, Or<Q, R>>`].
///
/// Ad infinitum...
#[macro_export]
macro_rules! or {
    ($first: ty, $second: ty) => {
        $crate::logic::Or<$first, $second>
    };

    ($first: ty, $second: ty, $($rest: ty),+ $(,)?) => {
        $crate::or!($first, $crate::or!($second, $($rest),+))
    }
}

/// Given two or more predicates, composes them together with [`Xor`].
///
/// For predicates `P` and `Q`, `xor!(P, Q)` is [`Xor<P, Q>`].
///
/// For predicates `P`, `Q`, and `R`, `xor!(P, Q, R)` is [`Xor<P, Xor<Q, R>>`].
///
/// Ad infinitum...
#[macro_export]
macro_rules! xor {
    ($first: ty, $second: ty) => {
        $crate::logic::Xor<$first, $second>
    };

    ($first: ty, $second: ty, $($rest: ty),+ $(,)?) => {
        $crate::xor!($first, $crate::xor!($second, $($rest),+))
    }
}
