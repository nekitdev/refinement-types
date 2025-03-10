//! Refinement types.
//!
//! # Examples
//!
//! TODO
//!
//! # Features
//!
//! TODO

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod char;
pub mod core;
pub mod empty;
pub mod int;
pub mod length;
#[macro_use]
pub mod logic;
pub mod static_str;
pub mod str;
#[macro_use]
pub mod type_str;

#[cfg(feature = "regex")]
#[macro_use]
pub mod type_regex;

pub use core::{Error, ErrorCore, Predicate, Refinement};

pub use static_str::StaticStr;
pub use type_str::TypeStr;

#[cfg(feature = "regex")]
pub use type_regex::{Regex, StaticRegex, TypeRegex};
