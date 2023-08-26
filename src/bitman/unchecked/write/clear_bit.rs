use crate::bitman::Bits;
use crate::bitman::NOnesOffset;

use core::ops::BitAnd;
use core::ops::Not;

/// Can set single bit low.
pub trait ClearBit {
    /// My type.
    type Type;

    /// Set single bit low.
    fn clear_bit(&self, index: Self::Type) -> Self::Type;
}

/// Implement [`ClearBit`] for given type.
macro_rules! ImplementClearBit {
    ($type:ty) => {
        impl ClearBit for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn clear_bit(&self, index: Self) -> Self {
                assert!(index < Self::bits(), "invalid bit index");
                let mask = Self::n_ones_offset(1, index).not();
                self.bitand(mask)
            }
        }
    };
}

ImplementClearBit!(u8);
ImplementClearBit!(u32);
ImplementClearBit!(u64);

#[cfg(test)]
mod tests {
    use super::ClearBit;
    use crate::bitman::Ones;

    #[test]
    fn test_clear_bit() {
        // TODO: more comprehensive testing
        assert_eq!(u8::ones().clear_bit(0), 0b1111_1110);
        assert_eq!(u8::ones().clear_bit(1), 0b1111_1101);
        assert_eq!(u8::ones().clear_bit(2), 0b1111_1011);
        assert_eq!(u8::ones().clear_bit(3), 0b1111_0111);
        assert_eq!(u8::ones().clear_bit(4), 0b1110_1111);
        assert_eq!(u8::ones().clear_bit(5), 0b1101_1111);
        assert_eq!(u8::ones().clear_bit(6), 0b1011_1111);
        assert_eq!(u8::ones().clear_bit(7), 0b0111_1111);
    }

    #[test]
    #[should_panic]
    fn test_clear_bit_panics() {
        assert_eq!(u8::ones().clear_bit(8), 0b1111_1111);
    }
}
