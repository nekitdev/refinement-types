//! Core functionality.

use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Deref,
};

#[cfg(feature = "unsafe-assert")]
use core::hint::assert_unchecked;

pub use core::error::Error as ErrorCore;

#[cfg(feature = "diagnostics")]
use miette::Diagnostic;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

use crate::{static_str::StaticStr, type_str, type_str::TypeStr};

type_str!(pub NoContext = "no context" => "Represents the abscence of context.");

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

    /// Formats the expectation code of the predicate.
    ///
    /// # Errors
    ///
    /// These can rarely occur, but any [`fmt::Error`] values are simply propagated.
    fn expect_code(formatter: &mut fmt::Formatter<'_>) -> fmt::Result;

    /// Checks whether the given value satisfies the predicate.
    ///
    /// This is the same as calling [`is_ok`] on the [`check`] result.
    ///
    /// [`is_ok`]: Result::is_ok
    /// [`check`]: Predicate::check
    fn is_satisfied(value: &T) -> bool {
        Self::check(value).is_ok()
    }

    /// Returns the expectation of the predicate.
    fn expected() -> Expected<T, Self> {
        Expected::new()
    }

    /// Returns the expectation code of the predicate.
    fn expected_code() -> ExpectedCode<T, Self> {
        ExpectedCode::new()
    }
}

/// Represents expectations of predicates.
pub struct Expected<T: ?Sized, P: ?Sized> {
    value: PhantomData<T>,
    predicate: PhantomData<P>,
}

impl<T: ?Sized, P: ?Sized> Expected<T, P> {
    const fn new() -> Self {
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

/// Represents expectation codes of predicates.
pub struct ExpectedCode<T: ?Sized, P: ?Sized> {
    value: PhantomData<T>,
    predicate: PhantomData<P>,
}

impl<T: ?Sized, P: ?Sized> ExpectedCode<T, P> {
    const fn new() -> Self {
        Self {
            value: PhantomData,
            predicate: PhantomData,
        }
    }
}

impl<T: ?Sized, P: Predicate<T> + ?Sized> fmt::Display for ExpectedCode<T, P> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        P::expect_code(formatter)
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
        // SAFETY: by construction, the value satisfies the predicate
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

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> AsRef<T> for Refinement<T, P, C> {
    fn as_ref(&self) -> &T {
        self.get()
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
        // SAFETY: the caller must ensure that the default value satisfies the predicate
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
pub struct Error<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized = NoContext> {
    /// The value that failed to satisfy the predicate.
    pub value: T,

    /// The error produced by the predicate.
    pub error: P::Error,

    /// The context of the refinement.
    pub context: PhantomData<C>,
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> fmt::Display for Error<T, P, C> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{EXPECTED} {expected} (code `{code}`) [{context}]",
            expected = P::expected(),
            code = P::expected_code(),
            context = Self::context(),
        )
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> fmt::Debug for Error<T, P, C> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, formatter)
    }
}

impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> ErrorCore for Error<T, P, C>
where
    P::Error: ErrorCore + 'static,
{
    fn source(&self) -> Option<&(dyn ErrorCore + 'static)> {
        Some(self.error())
    }
}

#[cfg(feature = "diagnostics")]
impl<T, P: Predicate<T> + ?Sized, C: TypeStr + ?Sized> Diagnostic for Error<T, P, C>
where
    P::Error: Diagnostic + 'static,
{
    fn code(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new(P::expected_code()))
    }

    fn help(&self) -> Option<Box<dyn fmt::Display + '_>> {
        Some(Box::new(Self::context()))
    }

    fn diagnostic_source(&self) -> Option<&dyn Diagnostic> {
        Some(self.error())
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
        match Self::check(&value) {
            // SAFETY: the value satisfies the predicate if the check is successful
            Ok(()) => Ok(unsafe { Self::unchecked(value) }),
            Err(error) => Err(Error::new(value, error)),
        }
    }

    /// Checks the given value.
    ///
    /// This is the same as calling [`P::check`] on the value, where `P` is the predicate type.
    ///
    /// # Errors
    ///
    /// Returns the error produced by the predicate if the value does not satisfy it.
    ///
    /// [`P::check`]: Predicate::check
    pub fn check(value: &T) -> Result<(), P::Error> {
        P::check(value)
    }

    /// Checks whether the given value satisfies the predicate.
    ///
    /// This is the same as calling [`P::is_satisfied`] on the value, where `P` is the predicate type.
    ///
    /// [`P::is_satisfied`]: Predicate::is_satisfied
    pub fn is_fine(value: &T) -> bool {
        P::is_satisfied(value)
    }

    /// Maps the value of the refinement.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] if the resulting value does not satisfy the predicate.
    pub fn map<F: FnOnce(T) -> T>(self, function: F) -> Result<Self, Error<T, P, C>> {
        Self::refine(function(self.take()))
    }

    /// Maps thte value of the refinement without checking the new value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the new value satisfies the predicate.
    pub unsafe fn map_unchecked<F: FnOnce(T) -> T>(self, function: F) -> Self {
        // SAFETY: the caller must ensure that the new value satisfies the predicate
        unsafe { Self::unchecked(function(self.take())) }
    }

    /// Replaces the value of the refinement.
    ///
    /// # Errors
    ///
    /// Returns [`struct@Error`] if the new value does not satisfy the predicate.
    pub fn replace(self, value: T) -> Result<Self, Error<T, P, C>> {
        Self::refine(value)
    }

    /// Replaces the value of the refinement without checking the new value.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the new value satisfies the predicate.
    pub unsafe fn replace_unchecked(self, value: T) -> Self {
        // SAFETY: the caller must ensure that the new value satisfies the predicate
        unsafe { Self::unchecked(value) }
    }

    #[cfg(feature = "unsafe-assert")]
    fn assert_refined(&self) {
        unsafe { assert_unchecked(Self::is_fine(&self.value)) }
    }

    /// Takes the value from the refinement.
    pub fn take(self) -> T {
        #[cfg(feature = "unsafe-assert")]
        self.assert_refined();

        self.value
    }

    /// Returns a reference to the value of the refinement.
    #[allow(clippy::missing_const_for_fn)] // conditionally const
    pub fn get(&self) -> &T {
        #[cfg(feature = "unsafe-assert")]
        self.assert_refined();

        &self.value
    }
}
