//! Predicates for characters.

pub mod ascii;
pub mod unicode;

pub use unicode::{
    Alphabetic, Alphanumeric, Ascii, Control, Lowercase, Numeric, Uppercase, Whitespace,
};

pub(crate) mod macros;
