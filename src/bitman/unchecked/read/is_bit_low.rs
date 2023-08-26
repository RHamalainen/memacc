use core::ops::BitAnd;
use core::ops::Shr;

use crate::bitman::Bits;
use crate::bitman::NOnes;

/// Can check if single bit is low.
pub trait IsBitLow {
    /// My type.
    type Type;

    /// Check if single bit is low.
    fn is_bit_low(&self, index: Self::Type) -> bool;
}

/// Implement [`IsBitLow`] for given type.
macro_rules! ImplementIsBitLow {
    ($type:ty) => {
        impl IsBitLow for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn is_bit_low(&self, index: Self) -> bool {
                assert!(index < Self::bits(), "invalid bit index");
                self.shr(index).bitand(Self::n_ones(1)) == 0
            }
        }
    };
}

ImplementIsBitLow!(u8);
ImplementIsBitLow!(u32);
ImplementIsBitLow!(u64);

// TODO: tests
