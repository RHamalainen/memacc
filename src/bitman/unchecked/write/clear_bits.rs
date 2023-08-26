use core::ops::BitAnd;
use core::ops::Not;
use core::ops::RangeInclusive;

use crate::bitman::InclusiveRangeExtension;
use crate::bitman::NOnesOffset;

/// Can clear multiple bits.
pub trait ClearBits {
    /// My type.
    type Type;

    /// Clear multiple bits.
    fn clear_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type;
}

/// Implement [`ClearBits`] for given type.
macro_rules! ImplementClearBits {
    ($type:ty) => {
        impl ClearBits for $type {
            type Type = Self;
            #[inline]
            fn clear_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type {
                assert!(range.is_empty().not(), "can not clear empty range of bits");
                let (start, _, length) = range.start_end_length();
                let mask = Self::n_ones_offset(length, start).not();
                self.bitand(mask)
            }
        }
    };
}

ImplementClearBits!(u8);
ImplementClearBits!(u32);
ImplementClearBits!(u64);

#[cfg(test)]
mod tests {
    use super::ClearBits;
    use crate::bitman::Ones;

    #[test]
    fn test_clear_bits() {
        // TODO: more comprehensive testing
        assert_eq!(u8::ones().clear_bits(0..=0), 0b1111_1110);
        assert_eq!(u8::ones().clear_bits(0..=1), 0b1111_1100);
        assert_eq!(u8::ones().clear_bits(0..=2), 0b1111_1000);
        assert_eq!(u8::ones().clear_bits(0..=3), 0b1111_0000);
        assert_eq!(u8::ones().clear_bits(0..=4), 0b1110_0000);
        assert_eq!(u8::ones().clear_bits(0..=5), 0b1100_0000);
        assert_eq!(u8::ones().clear_bits(0..=6), 0b1000_0000);
        assert_eq!(u8::ones().clear_bits(0..=7), 0b0000_0000);
        assert_eq!(u8::ones().clear_bits(0..=8), 0b0000_0000);
    }
}
