//! Predicates based on length.

use core::fmt;

#[cfg(feature = "diagnostics")]
use miette::Diagnostic;

use thiserror::Error;

use crate::{
    core::Predicate,
    logic::{And, Not},
};

/// Represents types that have length defined for their values.
pub trait HasLength {
    /// Returns the value length.
    fn length(&self) -> usize;
}

/// Represents errors that occur when the provided value has
/// length greater than or equal to some bound.
#[derive(Debug, Error)]
#[error("received value with length >= {other}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(length::lt), help("make sure the length is less than {other}"))
)]
pub struct LessError {
    /// The length against which the check was performed (the `N`).
    pub other: usize,
}

impl LessError {
    /// Constructs [`Self`].
    pub const fn new(other: usize) -> Self {
        Self { other }
    }
}

/// Checks whether the given value has length less than `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Less<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for Less<N> {
    type Error = LessError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.length() < N {
            Ok(())
        } else {
            Err(Self::Error::new(N))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length < {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::lt<{N}>")
    }
}

/// Represents errors that occur when the provided value has
/// length greater than some bound.
#[derive(Debug, Error)]
#[error("received value with length > {other}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(
        code(length::le),
        help("make sure the length is less than or equal to {other}")
    )
)]
pub struct LessOrEqualError {
    /// The length against which the check was performed (the `N`).
    pub other: usize,
}

impl LessOrEqualError {
    /// Constructs [`Self`].
    pub const fn new(other: usize) -> Self {
        Self { other }
    }
}

/// Checks whether the given value has length less than or equal to `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct LessOrEqual<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for LessOrEqual<N> {
    type Error = LessOrEqualError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.length() <= N {
            Ok(())
        } else {
            Err(Self::Error::new(N))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length <= {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::le<{N}>")
    }
}

/// Represents errors that occur when the provided value has
/// length less than or equal to some bound.
#[derive(Debug, Error)]
#[error("received value with length <= {other}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(length::gt), help("make sure the length is greater than {other}"))
)]
pub struct GreaterError {
    /// The length against which the check was performed (the `N`).
    pub other: usize,
}

impl GreaterError {
    /// Constructs [`Self`].
    pub const fn new(other: usize) -> Self {
        Self { other }
    }
}

/// Checks whether the given value has length greater than `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Greater<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for Greater<N> {
    type Error = GreaterError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.length() > N {
            Ok(())
        } else {
            Err(Self::Error::new(N))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length > {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::gt<{N}>")
    }
}

/// Represents errors that occur when the provided value has
/// length less than some bound.
#[derive(Debug, Error)]
#[error("received value with length < {other}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(
        code(length::ge),
        help("make sure the length is greater than or equal to {other}")
    )
)]
pub struct GreaterOrEqualError {
    /// The length against which the check was performed (the `N`).
    pub other: usize,
}

impl GreaterOrEqualError {
    /// Constructs [`Self`].
    pub const fn new(other: usize) -> Self {
        Self { other }
    }
}

/// Checks whether the given value has length greater than or equal to `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct GreaterOrEqual<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for GreaterOrEqual<N> {
    type Error = GreaterOrEqualError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.length() >= N {
            Ok(())
        } else {
            Err(Self::Error::new(N))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length >= {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::ge<{N}>")
    }
}

/// Represents errors that occur when the provided value has
/// length not equal to some bound.
#[derive(Debug, Error)]
#[error("received value with length != {other}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(length::eq), help("make sure the length is equal to {other}"))
)]
pub struct EqualError {
    /// The length against which the check was performed (the `N`).
    pub other: usize,
}

impl EqualError {
    /// Constructs [`Self`].
    pub const fn new(other: usize) -> Self {
        Self { other }
    }
}

/// Checks whether the given value has length equal to `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Equal<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for Equal<N> {
    type Error = EqualError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.length() == N {
            Ok(())
        } else {
            Err(Self::Error::new(N))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length == {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::eq<{N}>")
    }
}

/// Represents errors that occur when the provided value has
/// length equal to some bound.
#[derive(Debug, Error)]
#[error("received value with length == {other}")]
#[cfg_attr(
    feature = "diagnostics",
    derive(Diagnostic),
    diagnostic(code(length::ne), help("make sure the length is not equal to {other}"))
)]
pub struct NotEqualError {
    /// The length against which the check was performed (the `N`).
    pub other: usize,
}

impl NotEqualError {
    /// Constructs [`Self`].
    pub const fn new(other: usize) -> Self {
        Self { other }
    }
}

/// Checks whether the given value has length not equal to `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct NotEqual<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for NotEqual<N> {
    type Error = NotEqualError;

    #[allow(clippy::if_not_else)]
    fn check(value: &T) -> Result<(), Self::Error> {
        if value.length() != N {
            Ok(())
        } else {
            Err(Self::Error::new(N))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "value with length != {N}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::ne<{N}>")
    }
}

/// Represents `(M, N)` intervals.
pub type Open<const M: usize, const N: usize> = And<Greater<M>, Less<N>>;

/// Represents `[M, N)` intervals.
pub type ClosedOpen<const M: usize, const N: usize> = And<GreaterOrEqual<M>, Less<N>>;

/// Represents `(M, N]` intervals.
pub type OpenClosed<const M: usize, const N: usize> = And<Greater<M>, LessOrEqual<N>>;

/// Represents `[M, N]` intervals.
pub type Closed<const M: usize, const N: usize> = And<GreaterOrEqual<M>, LessOrEqual<N>>;

/// Checks whether the given value has zero length.
pub type Zero = Equal<0>;

/// Checks whether the given value has non-zero length.
pub type NonZero = NotEqual<0>;

/// Represents errors when the provided value has
/// length divided by [`divisor`] not equal to [`modulo`].
///
/// [`divisor`]: Self::divisor
/// [`modulo`]: Self::modulo
#[derive(Debug, Error)]
#[error("received value % {divisor} != {modulo}")]
pub struct ModuloError {
    /// The divisor that the value length should be divided by (the `D`).
    pub divisor: usize,
    /// The expected modulo of the length division (the `M`).
    pub modulo: usize,
}

impl ModuloError {
    /// Constructs [`Self`].
    pub const fn new(divisor: usize, modulo: usize) -> Self {
        Self { divisor, modulo }
    }
}

/// Checks whether the given value length divided by `D` has modulo `M`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Modulo<const D: usize, const M: usize>;

impl<const D: usize, const M: usize, T: HasLength + ?Sized> Predicate<T> for Modulo<D, M> {
    type Error = ModuloError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.length() % D == M {
            Ok(())
        } else {
            Err(Self::Error::new(D, M))
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length % {D} == {M}")
    }

    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "length::mod<{D}, {M}>")
    }
}

/// Checks whether the given value length is divisible by `D`.
pub type Divisible<const D: usize> = Modulo<D, 0>;

/// Checks whether the given value length is even.
pub type Even = Divisible<2>;

/// Checks whether the given value length is odd.
pub type Odd = Not<Even>;

// core

impl HasLength for str {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for [T] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T: HasLength + ?Sized> HasLength for &T {
    fn length(&self) -> usize {
        T::length(self)
    }
}

// prelude imports

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String, vec::Vec};

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: HasLength + ?Sized> HasLength for Box<T> {
    fn length(&self) -> usize {
        T::length(self)
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl HasLength for String {
    fn length(&self) -> usize {
        self.len()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasLength for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

// clone-on-write

#[cfg(feature = "alloc")]
use alloc::borrow::{Cow, ToOwned};

#[cfg(all(not(feature = "alloc"), feature = "std"))]
use std::borrow::{Cow, ToOwned};

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: ToOwned + HasLength + ?Sized> HasLength for Cow<'_, T> {
    fn length(&self) -> usize {
        T::length(self)
    }
}

// pointers

#[cfg(feature = "alloc")]
use alloc::rc::Rc;

#[cfg(all(not(feature = "alloc"), feature = "std"))]
use std::rc::Rc;

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: HasLength + ?Sized> HasLength for Rc<T> {
    fn length(&self) -> usize {
        T::length(self)
    }
}

#[cfg(feature = "alloc")]
use alloc::sync::Arc;

#[cfg(all(not(feature = "alloc"), feature = "std"))]
use std::sync::Arc;

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: HasLength + ?Sized> HasLength for Arc<T> {
    fn length(&self) -> usize {
        T::length(self)
    }
}

// shared collections

#[cfg(feature = "alloc")]
use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

#[cfg(all(not(feature = "alloc"), feature = "std"))]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

#[cfg(any(feature = "alloc", feature = "std"))]
impl<K, V> HasLength for BTreeMap<K, V> {
    fn length(&self) -> usize {
        self.len()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasLength for BTreeSet<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasLength for BinaryHeap<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasLength for LinkedList<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasLength for VecDeque<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

// collections

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "std")]
impl<K, V, S> HasLength for HashMap<K, V, S> {
    fn length(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl<T, S> HasLength for HashSet<T, S> {
    fn length(&self) -> usize {
        self.len()
    }
}

// OS strings

#[cfg(feature = "std")]
use std::ffi::{OsStr, OsString};

#[cfg(feature = "std")]
impl HasLength for OsStr {
    fn length(&self) -> usize {
        self.len()
    }
}

#[cfg(feature = "std")]
impl HasLength for OsString {
    fn length(&self) -> usize {
        self.len()
    }
}

// paths (via underlying strings)

#[cfg(feature = "std")]
use std::path::{Path, PathBuf};

#[cfg(feature = "std")]
impl HasLength for Path {
    fn length(&self) -> usize {
        self.as_os_str().length()
    }
}

#[cfg(feature = "std")]
impl HasLength for PathBuf {
    fn length(&self) -> usize {
        self.as_os_str().length()
    }
}
