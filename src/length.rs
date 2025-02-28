//! Predicates based on length.

use core::fmt;

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
#[error("received value with length >= {against}")]
pub struct LtError {
    /// The length against which the check was performed (the `N`).
    pub against: usize,
}

impl LtError {
    /// Constructs [`Self`].
    pub const fn new(against: usize) -> Self {
        Self { against }
    }
}

/// Checks whether the given value has length less than `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Lt<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for Lt<N> {
    type Error = LtError;

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
}

/// Represents errors that occur when the provided value has
/// length greater than some bound.
#[derive(Debug, Error)]
#[error("received value with length > {against}")]
pub struct LeError {
    /// The length against which the check was performed (the `N`).
    pub against: usize,
}

impl LeError {
    /// Constructs [`Self`].
    pub const fn new(against: usize) -> Self {
        Self { against }
    }
}

/// Checks whether the given value has length less than or equal to `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Le<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for Le<N> {
    type Error = LeError;

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
}

/// Represents errors that occur when the provided value has
/// length less than or equal to some bound.
#[derive(Debug, Error)]
#[error("received value with length <= {against}")]
pub struct GtError {
    /// The length against which the check was performed (the `N`).
    pub against: usize,
}

impl GtError {
    /// Constructs [`Self`].
    pub const fn new(against: usize) -> Self {
        Self { against }
    }
}

/// Checks whether the given value has length greater than `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Gt<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for Gt<N> {
    type Error = GtError;

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
}

/// Represents errors that occur when the provided value has
/// length less than some bound.
#[derive(Debug, Error)]
#[error("received value with length < {against}")]
pub struct GeError {
    /// The length against which the check was performed (the `N`).
    pub against: usize,
}

impl GeError {
    /// Constructs [`Self`].
    pub const fn new(against: usize) -> Self {
        Self { against }
    }
}

/// Checks whether the given value has length greater than or equal to `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Ge<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for Ge<N> {
    type Error = GeError;

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
}

/// Represents errors that occur when the provided value has
/// length not equal to some bound.
#[derive(Debug, Error)]
#[error("received value with length != {against}")]
pub struct EqError {
    /// The length against which the check was performed (the `N`).
    pub against: usize,
}

impl EqError {
    /// Constructs [`Self`].
    pub const fn new(against: usize) -> Self {
        Self { against }
    }
}

/// Checks whether the given value has length equal to `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Eq<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for Eq<N> {
    type Error = EqError;

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
}

/// Represents errors that occur when the provided value has
/// length equal to some bound.
#[derive(Debug, Error)]
#[error("received value with length == {against}")]
pub struct NeError {
    /// The length against which the check was performed (the `N`).
    pub against: usize,
}

impl NeError {
    /// Constructs [`Self`].
    pub const fn new(against: usize) -> Self {
        Self { against }
    }
}

/// Checks whether the given value has length not equal to `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Ne<const N: usize>;

impl<const N: usize, T: HasLength + ?Sized> Predicate<T> for Ne<N> {
    type Error = NeError;

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
}

/// Represents `(M, N)` intervals.
pub type Open<const M: usize, const N: usize> = And<Gt<M>, Lt<N>>;

/// Represents `[M, N)` intervals.
pub type ClosedOpen<const M: usize, const N: usize> = And<Ge<M>, Lt<N>>;

/// Represents `(M, N]` intervals.
pub type OpenClosed<const M: usize, const N: usize> = And<Gt<M>, Le<N>>;

/// Represents `[M, N]` intervals.
pub type Closed<const M: usize, const N: usize> = And<Ge<M>, Le<N>>;

/// Checks whether the given value has zero length.
pub type Zero = Eq<0>;

/// Checks whether the given value has non-zero length.
pub type NonZero = Ne<0>;

/// Represents errors when the provided value has
/// length divided by [`divisor`] not equal to [`modulo`].
///
/// [`divisor`]: Self::divisor
/// [`modulo`]: Self::modulo
#[derive(Debug, Error)]
#[error("received value % {divisor} != {modulo}")]
pub struct ModError {
    /// The divisor that the value length should be divided by (the `D`).
    pub divisor: usize,
    /// The expected modulo of the length division (the `M`).
    pub modulo: usize,
}

impl ModError {
    /// Constructs [`Self`].
    pub const fn new(divisor: usize, modulo: usize) -> Self {
        Self { divisor, modulo }
    }
}

/// Checks whether the given value length divided by `D` has modulo `M`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Mod<const D: usize, const M: usize>;

impl<const D: usize, const M: usize, T: HasLength + ?Sized> Predicate<T> for Mod<D, M> {
    type Error = ModError;

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
}

/// Checks whether the given value length is divisible by `D`.
pub type Div<const D: usize> = Mod<D, 0>;

/// Checks whether the given value length is even.
pub type Even = Div<2>;

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
