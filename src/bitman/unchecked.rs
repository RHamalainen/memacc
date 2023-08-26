//! Tools for unsafe access.

mod bits;
mod n_ones;
mod n_ones_offset;
mod ones;
pub mod read;
pub mod write;
mod zeros;

pub use bits::Bits;
pub use n_ones::NOnes;
pub use n_ones_offset::NOnesOffset;
pub use ones::Ones;
pub use zeros::Zeros;

/*
trait ZeroNLeft {
    type MyType;
    fn zero_n_left(n: Self::MyType) -> Self::MyType;
}

trait ZeroNRight {
    type MyType;
    fn zero_n_right(n: Self::MyType) -> Self::MyType;
}

impl ZeroNLeft for u8 {
    type MyType = u8;
    fn zero_n_left(n: Self::MyType) -> Self::MyType {
        0
    }
}
*/
