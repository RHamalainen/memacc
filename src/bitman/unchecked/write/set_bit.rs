use crate::bitman::Bits;
use crate::bitman::NOnesOffset;

use core::ops::BitOr;

/// Can set single bit high.
pub trait SetBit {
    /// My type.
    type Type;

    /// Set single bit high.
    fn set_bit(&self, index: Self::Type) -> Self::Type;
}

/// Implement [`SetBit`] for given type.
macro_rules! ImplementSetBit {
    ($type:ty) => {
        impl SetBit for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn set_bit(&self, index: Self) -> Self {
                assert!(index < Self::bits(), "invalid bit index");
                let mask = Self::n_ones_offset(1, index);
                self.bitor(mask)
            }
        }
    };
}

ImplementSetBit!(u8);
ImplementSetBit!(u32);
ImplementSetBit!(u64);

// TODO: tests
