//! Bit manipulation.

pub mod checked;
pub mod unchecked;

use core::ops::Not;
use core::ops::RangeInclusive;

pub use unchecked::read::AreBitsHigh;
pub use unchecked::read::AreBitsHighScattered;
pub use unchecked::read::AreBitsLow;
pub use unchecked::read::AreBitsLowScattered;
pub use unchecked::read::IsBitHigh;
pub use unchecked::read::IsBitLow;
pub use unchecked::read::ReadBit;
pub use unchecked::read::ReadBits;
pub use unchecked::read::ReadBitsScattered;
pub use unchecked::write::ClearBit;
pub use unchecked::write::ClearBits;
pub use unchecked::write::ClearBitsScattered;
pub use unchecked::write::ClearNLeft;
pub use unchecked::write::ClearNRight;
pub use unchecked::write::SetBit;
pub use unchecked::write::SetBits;
pub use unchecked::write::SetBitsScattered;
pub use unchecked::write::WriteBit;
pub use unchecked::write::WriteBits;
pub use unchecked::write::WriteBitsScattered;
pub use unchecked::Bits;
pub use unchecked::NOnes;
pub use unchecked::NOnesOffset;
pub use unchecked::Ones;
pub use unchecked::Zeros;

trait InclusiveRangeExtension {
    type Type;
    fn start_end_length(&self) -> (Self::Type, Self::Type, Self::Type);
}

/// Implement [`InclusiveRangeExtension`] for given type.
macro_rules! ImplementInclusiveRangeExtension {
    ($type:ty) => {
        impl InclusiveRangeExtension for RangeInclusive<$type> {
            type Type = $type;
            #[inline]
            #[must_use]
            fn start_end_length(&self) -> (Self::Type, Self::Type, Self::Type) {
                assert!(self.is_empty().not(), "can not write empty range of bits");
                let start = *self.start();
                let end = *self.end();
                let length = end - start + 1;
                (start, end, length)
            }
        }
    };
}

ImplementInclusiveRangeExtension!(u8);
ImplementInclusiveRangeExtension!(u32);
ImplementInclusiveRangeExtension!(u64);
