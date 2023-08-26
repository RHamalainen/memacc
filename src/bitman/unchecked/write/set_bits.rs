use core::ops::BitOr;
use core::ops::Not;
use core::ops::RangeInclusive;

use crate::bitman::InclusiveRangeExtension;
use crate::bitman::NOnesOffset;

/// Can set multiple bits.
pub trait SetBits {
    /// My type.
    type Type;

    /// Set multiple bits.
    fn set_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type;
}

/// Implement [`SetBits`] for given type.
macro_rules! ImplementSetBits {
    ($type:ty) => {
        impl SetBits for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn set_bits(&self, range: RangeInclusive<Self>) -> Self {
                assert!(range.is_empty().not(), "can not set empty range of bits");
                let (start, _, length) = range.start_end_length();
                let mask = Self::n_ones_offset(length, start);
                self.bitor(mask)
            }
        }
    };
}

ImplementSetBits!(u8);
ImplementSetBits!(u32);
ImplementSetBits!(u64);

#[cfg(test)]
mod tests {
    use super::SetBits;
    use crate::bitman::Zeros;

    #[test]
    fn test_set_bits() {
        // TODO: more comprehensive testing
        assert_eq!(u8::zeros().set_bits(0..=0), 0b0000_0001);
        assert_eq!(u8::zeros().set_bits(0..=1), 0b0000_0011);
        assert_eq!(u8::zeros().set_bits(0..=2), 0b0000_0111);
        assert_eq!(u8::zeros().set_bits(0..=3), 0b0000_1111);
        assert_eq!(u8::zeros().set_bits(0..=4), 0b0001_1111);
        assert_eq!(u8::zeros().set_bits(0..=5), 0b0011_1111);
        assert_eq!(u8::zeros().set_bits(0..=6), 0b0111_1111);
        assert_eq!(u8::zeros().set_bits(0..=7), 0b1111_1111);
        assert_eq!(u8::zeros().set_bits(0..=8), 0b1111_1111);
    }
}
