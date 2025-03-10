//! Predicates based on strings.

pub mod bytes;
pub mod chars;
pub mod core;

pub use core::{
    Ascii, Contains, ContainsChar, EndsWith, EndsWithChar, StartsWith, StartsWithChar, Trimmed,
    TrimmedEnd, TrimmedStart,
};

#[cfg(feature = "regex")]
pub use core::Matches;
