//! Type-level strings.

use crate::static_str::StaticStr;

/// Represents type-level strings.
pub trait TypeStr {
    /// The string value.
    const VALUE: StaticStr;
}

#[doc(hidden)]
pub mod import {
    pub use core::{fmt, marker::PhantomData};
}

/// Lifts static strings to type-level strings.
///
/// # Examples
///
/// ```
/// use refinement_types::type_str;
///
/// type_str!(HelloWorld = "Hello, world!");
/// ```
///
/// Is equivalent to:
///
/// ```
/// use core::{fmt, marker::PhantomData};
///
/// use refinement_types::{StaticStr, TypeStr};
///
/// struct HelloWorld {
///     private: PhantomData<()>,
/// }
///
/// impl TypeStr for HelloWorld {
///     const VALUE: StaticStr = "Hello, world!";
/// }
/// ```
#[macro_export]
macro_rules! type_str {
    ($vis: vis $name: ident = $value: expr $(=> $doc: expr)?) => {
        $(
            #[doc = $doc]
        )?
        $vis struct $name {
            private: $crate::type_str::import::PhantomData<()>,
        }

        impl $crate::type_str::TypeStr for $name {
            const VALUE: $crate::static_str::StaticStr = $value;
        }
    };
}
