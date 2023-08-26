use core::ops::RangeInclusive;

use crate::bitman::IsBitHigh;

/// Can check if multiple bits are low.
pub trait AreBitsLow {
    /// My type.
    type Type;

    /// Check if multiple bits are low.
    fn are_bits_low(&self, range: RangeInclusive<Self::Type>) -> bool;
}

/// Implement [`AreBitsLow`] for given type.
macro_rules! ImplementAreBitsLow {
    ($type:ty) => {
        impl AreBitsLow for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn are_bits_low(&self, range: RangeInclusive<Self>) -> bool {
                for index in range {
                    if self.is_bit_high(index) {
                        return false;
                    }
                }
                true
            }
        }
    };
}

ImplementAreBitsLow!(u8);
ImplementAreBitsLow!(u32);
ImplementAreBitsLow!(u64);

// TODO: tests
