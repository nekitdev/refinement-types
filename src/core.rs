//! Core functionality.

use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

use crate::{static_str::StaticStr, type_str::TypeStr};

crate::type_str!(pub NoContext = "no context" => "Represents absence of context.");

/// Literal space string.
pub const SPACE: StaticStr = " ";

/// Literal `(` string.
pub const OPEN: StaticStr = "(";

/// Literal `)` string.
pub const CLOSE: StaticStr = ")";

/// Literal `expected` string.
pub const EXPECTED: StaticStr = "expected";

/// Represents predicates used to refine values.
pub trait Predicate<T: ?Sized> {
    /// The associated error type which is used to represent checks.
    type Error;

    /// Checks if the value of type `T` satisfies the predicate.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] if the value does not satisfy the predicate.
    fn check(value: &T) -> Result<(), Self::Error>;

    /// Formats the expectation of the predicate.
    ///
    /// # Errors
    ///
    /// These can rarely occur, but any [`fmt::Error`] values are simply propagated.
    fn expect(formatter: &mut fmt::Formatter<'_>) -> fmt::Result;

    /// Returns [`Expected`] type that uses [`expect`] to implement the [`fmt::Display`] trait,
    /// allowing to format errors without depending on `std` or `alloc`.
    ///
    /// [`expect`]: Self::expect
    fn expected() -> Expected<T, Self> {
        Expected::new()
    }
}

/// This structure is created by the [`expected`] method on [`Predicate`].
///
/// [`expected`]: Predicate::expected
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Expected<T: ?Sized, P: Predicate<T> + ?Sized> {
    value: PhantomData<T>,
    predicate: PhantomData<P>,
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> Expected<T, P> {
    /// Constructs [`Self`].
    pub const fn new() -> Self {
        Self {
            value: PhantomData,
            predicate: PhantomData,
        }
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> fmt::Display for Expected<T, P> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        P::expect(formatter)
    }
}

/// Represents refined values.
///
/// Values of this type are guaranteed to contain values of type `T`
/// that satisfy the predicate `P`.
pub struct Refinement<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized = NoContext> {
    value: T,
    predicate: PhantomData<P>,
    context: PhantomData<C>,
}

impl<T: fmt::Debug, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> fmt::Debug
    for Refinement<T, P, C>
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<T: fmt::Display, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> fmt::Display
    for Refinement<T, P, C>
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl<T: Clone, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Clone for Refinement<T, P, C> {
    fn clone(&self) -> Self {
        unsafe { Self::unchecked(self.get().clone()) }
    }
}

impl<T: Copy, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Copy for Refinement<T, P, C> {}

impl<T: PartialEq, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> PartialEq
    for Refinement<T, P, C>
{
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(other.get())
    }
}

impl<T: Eq, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Eq for Refinement<T, P, C> {}

impl<T: PartialOrd, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> PartialOrd
    for Refinement<T, P, C>
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get().partial_cmp(other.get())
    }
}

impl<T: Ord, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Ord for Refinement<T, P, C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get().cmp(other.get())
    }
}

impl<T: Hash, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Hash for Refinement<T, P, C> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.get().hash(hasher);
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Deref for Refinement<T, P, C> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T: Default, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Refinement<T, P, C> {
    /// Refines the default value of type `T`.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] if the default value does not satisfy the predicate.
    pub fn try_default() -> Result<Self, Error<T, P, C>> {
        Self::refine(T::default())
    }

    /// Constructs [`Self`] without checking the default value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the default value satisfies the predicate.
    pub unsafe fn unchecked_default() -> Self {
        unsafe { Self::unchecked(T::default()) }
    }
}

#[cfg(feature = "serde")]
impl<T: Serialize, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Serialize
    for Refinement<T, P, C>
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de>, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Deserialize<'de>
    for Refinement<T, P, C>
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = T::deserialize(deserializer)?;

        Self::refine(value).map_err(de::Error::custom)
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Refinement<T, P, C> {
    /// Constructs [`Self`] without checking the value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the value satisfies the predicate.
    ///
    /// This can be checked via simply calling the [`Predicate::check`] method.
    pub const unsafe fn unchecked(value: T) -> Self {
        Self {
            value,
            predicate: PhantomData,
            context: PhantomData,
        }
    }
}

/// Represents refinement errors.
///
/// This error is constructed from the value that failed to satisfy the predicate
/// and the error produced by the predicate.
#[derive(Debug, Error)]
#[error("expected {expected} [{context}]", expected = P::expected(), context = Self::context())]
pub struct Error<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized = NoContext> {
    /// The value that failed to satisfy the predicate.
    pub value: T,

    /// The error produced by the predicate.
    #[source]
    pub error: P::Error,

    /// The context of the refinement.
    pub context: PhantomData<C>,
}

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};

/// Represents errors that retain their message and invalid value.
#[cfg(any(feature = "alloc", feature = "std"))]
#[derive(Debug, Error)]
#[error("{message}")]
pub struct ErasedError<T> {
    /// The value that failed to satisfy the predicate.
    pub value: T,
    /// The expectation message.
    pub message: String,
}

impl<T> ErasedError<T> {
    /// Constructs [`Self`].
    pub const fn new(value: T, message: String) -> Self {
        Self { value, message }
    }
}

/// Represents errors that only retain their message.
#[cfg(any(feature = "alloc", feature = "std"))]
#[derive(Debug, Error)]
#[error("{string}")]
pub struct MessageError {
    /// The expectation message.
    pub string: String,
}

impl MessageError {
    /// Constructs [`Self`].
    pub const fn new(string: String) -> Self {
        Self { string }
    }

    /// Returns the expectation message.
    pub fn string(&self) -> &str {
        self.string.as_str()
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Error<T, P, C> {
    /// Constructs [`Self`].
    pub const fn new(value: T, error: P::Error) -> Self {
        Self {
            value,
            error,
            context: PhantomData,
        }
    }

    /// Returns the value that failed to satisfy the predicate.
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Returns the error that was produced by the predicate.
    pub const fn error(&self) -> &P::Error {
        &self.error
    }

    /// Returns the context of the refinement.
    pub const fn context() -> StaticStr {
        C::VALUE
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Error<T, P, C> {
    /// Erases the source error, retaining only the value and the expectation message.
    pub fn erase(self) -> ErasedError<T> {
        let string = self.to_string();

        ErasedError::new(self.value, string)
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Error<T, P, C> {
    /// Erases the source error and discards the value, retaining only the expectation message.
    pub fn message(&self) -> MessageError {
        MessageError::new(self.to_string())
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> From<Error<T, P, C>> for ErasedError<T> {
    fn from(error: Error<T, P, C>) -> Self {
        error.erase()
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> From<Error<T, P, C>> for MessageError {
    fn from(error: Error<T, P, C>) -> Self {
        error.message()
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Error<T, P, C> {
    /// Returns the contained value and the received error.
    pub fn into_parts(self) -> (T, P::Error) {
        (self.value, self.error)
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> From<(T, P::Error)> for Error<T, P, C> {
    fn from((value, error): (T, P::Error)) -> Self {
        Self::new(value, error)
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> From<Error<T, P, C>> for (T, P::Error) {
    fn from(error: Error<T, P, C>) -> Self {
        error.into_parts()
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Refinement<T, P, C> {
    /// Refines the given value.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] if the value does not satisfy the predicate.
    pub fn refine(value: T) -> Result<Self, Error<T, P, C>> {
        match P::check(&value) {
            // SAFETY: the value satisfies the predicate if the check is successful
            Ok(()) => Ok(unsafe { Self::unchecked(value) }),
            Err(error) => Err(Error::new(value, error)),
        }
    }

    /// Maps the value of the refinement.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] if the resulting value does not satisfy the predicate.
    pub fn map<F: FnOnce(T) -> T>(self, function: F) -> Result<Self, Error<T, P, C>> {
        Self::refine(function(self.take()))
    }

    /// Replaces the value of the refinement.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] if the new value does not satisfy the predicate.
    pub fn replace(self, value: T) -> Result<Self, Error<T, P, C>> {
        Self::refine(value)
    }

    /// Takes the value from the refinement.
    pub fn take(self) -> T {
        self.value
    }

    /// Returns a reference to the value of the refinement.
    pub const fn get(&self) -> &T {
        &self.value
    }
}
