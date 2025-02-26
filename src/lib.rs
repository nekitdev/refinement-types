//! Refinement types.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(auto_doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod char;
pub mod core;
pub mod empty;
pub mod int;
pub mod length;
pub mod logic;
pub mod static_str;
pub mod str;
#[macro_use]
pub mod type_str;

#[cfg(feature = "regex")]
#[macro_use]
pub mod type_regex;

pub use core::{Error, Predicate, Refinement};
pub use logic::{And, False, Imply, Nand, Nor, Not, Or, True, Xnor, Xor};
pub use type_str::TypeStr;

#[cfg(feature = "regex")]
pub use type_regex::TypeRegex;
