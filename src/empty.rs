//! Predicates based on emptiness.

use core::fmt;

use thiserror::Error;

use crate::static_str::StaticStr;
use crate::{core::Predicate, logic::Not};

/// Represents types that have emptiness-checking capabilities.
pub trait HasEmpty {
    /// Checks whether the value is empty.
    fn empty(&self) -> bool;
}

/// Represents errors that occur when the provided value is non-empty.
#[derive(Debug, Error, Default)]
#[error("received non-empty value")]
pub struct NonEmptyError;

impl NonEmptyError {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self
    }
}

/// The `empty value` literal.
pub const VALUE: StaticStr = "empty value";

/// Checks whether the value is empty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IsEmpty;

impl<T: HasEmpty + ?Sized> Predicate<T> for IsEmpty {
    type Error = NonEmptyError;

    fn check(value: &T) -> Result<(), Self::Error> {
        if value.empty() {
            Ok(())
        } else {
            Err(Self::Error::new())
        }
    }

    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(VALUE)
    }
}

/// Checks whether the value is non-empty.
pub type IsNonEmpty = Not<IsEmpty>;

// core

impl HasEmpty for str {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> HasEmpty for [T] {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T: HasEmpty + ?Sized> HasEmpty for &T {
    fn empty(&self) -> bool {
        T::empty(self)
    }
}

// prelude imports

#[cfg(feature = "alloc")]
use alloc::{boxed::Box, string::String, vec::Vec};

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: HasEmpty + ?Sized> HasEmpty for Box<T> {
    fn empty(&self) -> bool {
        T::empty(self)
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl HasEmpty for String {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasEmpty for Vec<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

// clone-on-write

#[cfg(feature = "alloc")]
use alloc::borrow::{Cow, ToOwned};

#[cfg(all(not(feature = "alloc"), feature = "std"))]
use std::borrow::{Cow, ToOwned};

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: ToOwned + HasEmpty + ?Sized> HasEmpty for Cow<'_, T> {
    fn empty(&self) -> bool {
        T::empty(self)
    }
}

// pointers

#[cfg(feature = "alloc")]
use alloc::rc::Rc;

#[cfg(all(not(feature = "alloc"), feature = "std"))]
use std::rc::Rc;

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: HasEmpty + ?Sized> HasEmpty for Rc<T> {
    fn empty(&self) -> bool {
        T::empty(self)
    }
}

#[cfg(feature = "alloc")]
use alloc::sync::Arc;

#[cfg(all(not(feature = "alloc"), feature = "std"))]
use std::sync::Arc;

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T: HasEmpty + ?Sized> HasEmpty for Arc<T> {
    fn empty(&self) -> bool {
        T::empty(self)
    }
}

// shared collections

#[cfg(feature = "alloc")]
use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

#[cfg(all(not(feature = "alloc"), feature = "std"))]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

#[cfg(any(feature = "alloc", feature = "std"))]
impl<K, V> HasEmpty for BTreeMap<K, V> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasEmpty for BTreeSet<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasEmpty for BinaryHeap<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasEmpty for LinkedList<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(any(feature = "alloc", feature = "std"))]
impl<T> HasEmpty for VecDeque<T> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

// collections

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "std")]
impl<K, V, S> HasEmpty for HashMap<K, V, S> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(feature = "std")]
impl<T, S> HasEmpty for HashSet<T, S> {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

// OS strings

#[cfg(feature = "std")]
use std::ffi::{OsStr, OsString};

#[cfg(feature = "std")]
impl HasEmpty for OsStr {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(feature = "std")]
impl HasEmpty for OsString {
    fn empty(&self) -> bool {
        self.is_empty()
    }
}

// paths (via underlying strings)

#[cfg(feature = "std")]
use std::path::{Path, PathBuf};

#[cfg(feature = "std")]
impl HasEmpty for Path {
    fn empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}

#[cfg(feature = "std")]
impl HasEmpty for PathBuf {
    fn empty(&self) -> bool {
        self.as_os_str().is_empty()
    }
}
