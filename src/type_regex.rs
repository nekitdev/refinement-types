//! Type-level regular expressions.

pub use regex::Regex;

use crate::static_str::StaticStr;

#[doc(hidden)]
pub mod import {
    pub use std::sync::LazyLock;
}

/// Represents static regular expressions (as returned in [`get`] of [`TypeRegex`]).
///
/// [`get`]: TypeRegex::get
pub type StaticRegex = &'static Regex;

/// Represents type-level regular expressions.
pub trait TypeRegex {
    /// Returns the compiled regular expression.
    fn get() -> StaticRegex;
}

/// The `invalid regex` literal.
pub const INVALID: StaticStr = "invalid regex";

/// Lifts strings to type-level regular expressions.
#[macro_export]
macro_rules! type_regex {
    ($vis: vis $name: ident = $regex: expr $(=> $doc: expr)?) => {
        $(
            #[doc = $doc]
        )?
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        $vis struct $name;

        impl $crate::type_regex::TypeRegex for $name {
            fn get() -> $crate::type_regex::StaticRegex {
                use $crate::type_regex::import::LazyLock;

                static REGEX: LazyLock<$crate::type_regex::Regex> = LazyLock::new(|| {
                    $crate::type_regex::Regex::new($regex).expect($crate::type_regex::INVALID)
                });

                LazyLock::force(&REGEX)
            }
        }
    };
}
