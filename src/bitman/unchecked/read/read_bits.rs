use core::ops::Not;
use core::ops::RangeInclusive;
use core::ops::Shr;

use crate::bitman::ClearNLeft;
use crate::bitman::InclusiveRangeExtension;

/// Can read values of multiple bits.
pub trait ReadBits {
    /// My type.
    type Type;

    /// Read multiple bits.
    fn read_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type;
}

/// Implement [`ReadBits`] for given type.
macro_rules! ImplementReadBits {
    ($type:ty) => {
        impl ReadBits for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn read_bits(&self, range: RangeInclusive<Self>) -> Self {
                assert!(range.is_empty().not(), "can not read empty range of bits");
                let (start, _, length) = range.start_end_length();
                self.shr(start).keep_n_right(length)
            }
        }
    };
}

ImplementReadBits!(u8);
ImplementReadBits!(u32);
ImplementReadBits!(u64);

#[cfg(test)]
mod tests {
    use super::ReadBits;
    use crate::bitman::Ones;

    #[test]
    fn test_read_bits() {
        // TODO: more comprehensive testing
        assert_eq!(u8::ones().read_bits(0..=0), 0b0000_0001);
        assert_eq!(u8::ones().read_bits(0..=1), 0b0000_0011);
        assert_eq!(u8::ones().read_bits(0..=2), 0b0000_0111);
        assert_eq!(u8::ones().read_bits(0..=3), 0b0000_1111);
        assert_eq!(u8::ones().read_bits(0..=4), 0b0001_1111);
        assert_eq!(u8::ones().read_bits(0..=5), 0b0011_1111);
        assert_eq!(u8::ones().read_bits(0..=6), 0b0111_1111);
        assert_eq!(u8::ones().read_bits(0..=7), 0b1111_1111);
        assert_eq!(u8::ones().read_bits(0..=8), 0b1111_1111);

        assert_eq!(u8::ones().read_bits(1..=1), 0b0000_0001);
        assert_eq!(u8::ones().read_bits(1..=2), 0b0000_0011);
        assert_eq!(u8::ones().read_bits(1..=3), 0b0000_0111);
        assert_eq!(u8::ones().read_bits(1..=4), 0b0000_1111);
        assert_eq!(u8::ones().read_bits(1..=5), 0b0001_1111);
        assert_eq!(u8::ones().read_bits(1..=6), 0b0011_1111);
        assert_eq!(u8::ones().read_bits(1..=7), 0b0111_1111);
        assert_eq!(u8::ones().read_bits(1..=8), 0b0111_1111);
    }

    #[test]
    #[should_panic]
    fn test_read_bits_panics() {
        #[allow(clippy::reversed_empty_ranges)]
        u8::ones().read_bits(1..=0);
    }
}
