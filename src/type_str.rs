//! Type-level strings.

use crate::static_str::StaticStr;

/// Represents type-level strings.
pub trait TypeStr {
    /// The string value.
    const VALUE: StaticStr;
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
/// use refinement_types::{StaticStr, TypeStr};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// struct HelloWorld;
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
        $vis struct $name;

        impl $crate::type_str::TypeStr for $name {
            const VALUE: $crate::static_str::StaticStr = $value;
        }
    };
}
