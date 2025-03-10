//! Predicates for integers.

pub(crate) mod macros;

macros::unsigned_module!(u8 => u8);
macros::unsigned_module!(u16 => u16);
macros::unsigned_module!(u32 => u32);
macros::unsigned_module!(u64 => u64);
macros::unsigned_module!(u128 => u128);
macros::unsigned_module!(usize => usize);

macros::signed_module!(i8 => i8);
macros::signed_module!(i16 => i16);
macros::signed_module!(i32 => i32);
macros::signed_module!(i64 => i64);
macros::signed_module!(i128 => i128);
macros::signed_module!(isize => isize);
