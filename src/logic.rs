//! Logical operations on predicates.

use core::{fmt, marker::PhantomData};

use thiserror::Error;

use crate::{core::Predicate, static_str::StaticStr};

/// Represents predicates that are always satisfied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct True;

/// Represents errors that are never encountered.
///
/// This is essentially `!`, the never type.
#[derive(Debug, Error)]
pub enum NeverError {}

/// The `anything` string.
pub const ANYTHING: StaticStr = "anything";

impl<T: ?Sized> Predicate<T> for True {
    type Error = NeverError;

    fn check(_value: &T) -> Result<(), Self::Error> {
        Ok(())
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(ANYTHING)
    }
}

/// Represents predicates that are never satisfied (`0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct False;

/// Represents errors that are always encountered.
#[derive(Debug, Error)]
#[error("always errors")]
pub struct AlwaysError;

/// The `nothing` string.
pub const NOTHING: StaticStr = "nothing";

impl<T: ?Sized> Predicate<T> for False {
    type Error = AlwaysError;

    fn check(_value: &T) -> Result<(), Self::Error> {
        Err(AlwaysError)
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(NOTHING)
    }
}

/// Represents errors returned by [`And`].
#[derive(Debug, Error)]
pub enum EitherError<E, F> {
    /// Left error (`P` failed).
    #[error("left error: {0}")]
    Left(E),
    /// Right error (`Q` failed).
    #[error("right error: {0}")]
    Right(F),
}

/// Represents predicates that are satisfied when both `P` and `Q` are satisfied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct And<P: ?Sized, Q: ?Sized> {
    left: PhantomData<P>,
    right: PhantomData<Q>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized, Q: Predicate<T> + ?Sized> Predicate<T> for And<P, Q> {
    type Error = EitherError<P::Error, Q::Error>;

    fn check(value: &T) -> Result<(), Self::Error> {
        P::check(value)
            .map_err(EitherError::Left)
            .and_then(|()| Q::check(value).map_err(EitherError::Right))
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}) and ({})", P::expected(), Q::expected())
    }
}

/// Represents errors returned by [`Or`].
#[derive(Debug, Error)]
#[error("both errors encountered: {left} and {right}")]
pub struct BothError<E, F> {
    /// Left error (`P` failed).
    pub left: E,
    /// Right error (`Q` failed).
    pub right: F,
}

impl<E, F> BothError<E, F> {
    /// Constructs [`Self`]
    pub const fn new(left: E, right: F) -> Self {
        Self { left, right }
    }
}

/// Represents predicates that are satisfied when either `P` or `Q` (or both) are satisfied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
}

/// Represents errors returned by [`Not`].
#[derive(Debug, Error)]
#[error("negated error")]
pub struct NotError;

/// Represents predicates that are satisfied when `P` is not satisfied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Not<P: ?Sized> {
    predicate: PhantomData<P>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> Predicate<T> for Not<P> {
    type Error = NotError;

    fn check(value: &T) -> Result<(), Self::Error> {
        match P::check(value) {
            Ok(()) => Err(NotError),
            Err(_) => Ok(()),
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "not ({})", P::expected())
    }
}

/// Represents errors returned by [`Xor`].
#[derive(Debug, Error)]
pub enum NeitherOrBoth<E, F> {
    /// Neither error encountered.
    #[error("neither error encountered")]
    Neither,
    /// Both errors encountered.
    #[error(transparent)]
    Both(BothError<E, F>),
}

/// Represents predicates that are satisfied when either `P` or `Q` (but *not* both) are satisfied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
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
}

/// Composes [`Not`] and [`And`].
pub type Nand<P, Q> = Not<And<P, Q>>;
/// Composes [`Not`] and [`Or`].
pub type Nor<P, Q> = Not<Or<P, Q>>;
/// Composes [`Not`] and [`Xor`].
pub type Xnor<P, Q> = Not<Xor<P, Q>>;

/// Represents predicates that are satisfied when `P` implies `Q`.
pub type Imply<P, Q> = Or<Not<P>, Q>;
