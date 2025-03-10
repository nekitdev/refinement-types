//! Type-level regular expressions.

pub use regex::Regex;

use crate::static_str::StaticStr;

#[doc(hidden)]
pub mod import {
    pub use std::{fmt, sync::LazyLock};
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
///
/// ```
/// use refinement_types::type_regex;
///
/// type_regex!(Integer = "^0|[1-9][0-9]*$");
/// ```
///
/// Is equivalent to:
///
/// ```
/// use std::{fmt, sync::LazyLock};
///
/// use refinement_types::{Regex, StaticRegex, TypeRegex};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// struct Integer;
///
/// impl TypeRegex for Integer {
///     fn get() -> StaticRegex {
///         static REGEX: LazyLock<Regex> = LazyLock::new(|| {
///             Regex::new("^0|[1-9][0-9]*$").expect("invalid regex")
///         });
///
///         LazyLock::force(&REGEX)
///     }
/// }
///
/// impl fmt::Display for Integer {
///     fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
///         Self::get().fmt(formatter)
///     }
/// }
/// ```
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

        impl $crate::type_regex::import::fmt::Display for $name {
            fn fmt(
                &self, formatter: &mut $crate::type_regex::import::fmt::Formatter<'_>
            ) -> $crate::type_regex::import::fmt::Result {
                use $crate::type_regex::TypeRegex;

                Self::get().fmt(formatter)
            }
        }
    };
}
