//! Tools to index single bit.

mod generated;

/// Can index 8-bit unsigned integer.
pub trait IndexU8 {
    /// Index's value.
    fn value(self) -> usize;
}

/// Can index 32-bit unsigned integer.
pub trait IndexU32 {
    /// Index's value.
    fn value(self) -> usize;
}

/// Index.
#[allow(clippy::exhaustive_structs)]
pub struct I<const T: usize>;

/// TODO
#[macro_export]
macro_rules! index {
    ($indexor:literal) => {
        I::<$indexor>
    };
}
