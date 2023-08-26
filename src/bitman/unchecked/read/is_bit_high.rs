use core::ops::Not;

use crate::bitman::IsBitLow;

/// Can check if single bit is high.
pub trait IsBitHigh {
    /// My type.
    type Type;

    /// Check if single bit is high.
    fn is_bit_high(&self, index: Self::Type) -> bool;
}

/// Implement [`IsBitHigh`] for given type.
macro_rules! ImplementIsBitHigh {
    ($type:ty) => {
        impl IsBitHigh for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn is_bit_high(&self, index: Self) -> bool {
                self.is_bit_low(index).not()
            }
        }
    };
}

ImplementIsBitHigh!(u8);
ImplementIsBitHigh!(u32);
ImplementIsBitHigh!(u64);

// TODO: tests
