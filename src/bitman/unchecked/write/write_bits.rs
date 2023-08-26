use core::ops::BitOr;
use core::ops::Not;
use core::ops::RangeInclusive;
use core::ops::Shl;

use crate::bitman::ClearBits;
use crate::bitman::ClearNLeft;
use crate::bitman::InclusiveRangeExtension;

/// Can write multiple bits.
pub trait WriteBits {
    /// My type.
    type Type;

    /// Write multiple bits.
    fn write_bits(&self, value: Self::Type, range: RangeInclusive<Self::Type>) -> Self::Type;
}

/// Implement [`WriteBits`] for given type.
macro_rules! ImplementWriteBits {
    ($type:ty) => {
        impl WriteBits for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn write_bits(
                &self,
                value: Self::Type,
                range: RangeInclusive<Self::Type>,
            ) -> Self::Type {
                assert!(range.is_empty().not(), "can not write empty range of bits");
                let (start, _, length) = range.start_end_length();
                let mask = value.shl(start).keep_n_right(length);
                self.clear_bits(range).bitor(mask)
            }
        }
    };
}

ImplementWriteBits!(u8);
ImplementWriteBits!(u32);
ImplementWriteBits!(u64);

#[cfg(test)]
mod tests {
    use super::WriteBits;
    use crate::bitman::Zeros;

    #[test]
    fn test_write_bits() {
        // TODO: more comprehensive testing
        assert_eq!(u8::zeros().write_bits(0b0000_1010, 0..=0), 0b0000_0000);
        assert_eq!(u8::zeros().write_bits(0b0000_1010, 0..=1), 0b0000_0010);
        assert_eq!(u8::zeros().write_bits(0b0000_1010, 0..=2), 0b0000_0010);
        assert_eq!(u8::zeros().write_bits(0b0000_1010, 0..=3), 0b0000_1010);
        assert_eq!(u8::zeros().write_bits(0b0000_1010, 0..=4), 0b0000_1010);
        assert_eq!(u8::zeros().write_bits(0b0000_1010, 0..=5), 0b0000_1010);
        assert_eq!(u8::zeros().write_bits(0b0000_1010, 0..=6), 0b0000_1010);
        assert_eq!(u8::zeros().write_bits(0b0000_1010, 0..=7), 0b0000_1010);
        assert_eq!(u8::zeros().write_bits(0b0000_1010, 0..=8), 0b0000_1010);
    }
}
