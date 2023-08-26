use core::ops::RangeInclusive;

use crate::bitman::IsBitLow;

/// Can check if multiple bits are high.
pub trait AreBitsHigh {
    /// My type.
    type Type;

    /// Check if multiple bits are high.
    fn are_bits_high(&self, range: RangeInclusive<Self::Type>) -> bool;
}

/// Implement [`AreBitsHigh`] for given type.
macro_rules! ImplementAreBitsHigh {
    ($type:ty) => {
        impl AreBitsHigh for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn are_bits_high(&self, range: RangeInclusive<Self>) -> bool {
                for index in range {
                    if self.is_bit_low(index) {
                        return false;
                    }
                }
                true
            }
        }
    };
}

ImplementAreBitsHigh!(u8);
ImplementAreBitsHigh!(u32);
ImplementAreBitsHigh!(u64);

// TODO: tests
