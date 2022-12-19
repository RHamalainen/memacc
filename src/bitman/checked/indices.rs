//! Tools to index multiple bits.

mod generated;

/// Can index 8-bit unsigned integer.
pub trait IndexRangeU8 {
    /// Least significant accessed index.
    fn start(self) -> u8;

    /// Most significant accessed index.
    fn end(self) -> u8;
}

/// Can index 32-bit unsigned integer.
pub trait IndexRangeU32 {
    /// Least significant accessed index.
    fn start(self) -> u32;

    /// Most significant accessed index.
    fn end(self) -> u32;
}

/// Index range.
#[derive(Clone, Copy)]
#[allow(clippy::exhaustive_structs)]
pub struct IR<const START: usize, const END: usize>;

/// TODO
#[macro_export]
macro_rules! indices {
    ($start:literal, $end:literal) => {};
}
