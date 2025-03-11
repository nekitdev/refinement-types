pub(crate) mod import {
    pub use core::{fmt, marker::PhantomData};

    pub use paste::paste;
    pub use thiserror::Error;

    #[cfg(feature = "diagnostics")]
    pub use miette::Diagnostic;
}

macro_rules! predicate {
    (
        Name = $name: ident,
        Check = $check: ident,
        Doc = $doc: expr,
        Expected = $expected: expr,
        Code = $code: path,
        Error = $error: expr,
        Message = $message: expr,
        Help = $help: expr $(,)?
    ) => {
        $crate::char::macros::import::paste! {
            #[derive(Debug, $crate::char::macros::import::Error, Default)]
            #[error($message)]
            #[cfg_attr(
                feature = "diagnostics",
                derive($crate::char::macros::import::Diagnostic),
                diagnostic(code($code), help($help)),
            )]
            #[doc = $error]
            pub struct [< $name Error >];

            impl [< $name Error >] {
                /// Constructs [`Self`].
                pub const fn new() -> Self {
                    Self
                }
            }

            #[doc = $doc]
            pub struct $name {
                private: $crate::char::macros::import::PhantomData<()>,
            }

            impl $crate::core::Predicate<char> for $name {
                type Error = [< $name Error >];

                fn check(value: &char) -> Result<(), Self::Error> {
                    if value.$check() {
                        Ok(())
                    } else {
                        Err(Self::Error::new())
                    }
                }

                fn expect(
                    formatter: &mut $crate::char::macros::import::fmt::Formatter<'_>
                ) -> $crate::char::macros::import::fmt::Result {
                    write!(formatter, $expected)
                }

                fn expect_code(
                    formatter: &mut $crate::char::macros::import::fmt::Formatter<'_>
                ) -> $crate::char::macros::import::fmt::Result {
                    write!(formatter, stringify!($code))
                }
            }
        }
    };
}

pub(crate) use predicate;
