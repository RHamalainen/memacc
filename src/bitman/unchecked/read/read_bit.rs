use crate::bitman::IsBitHigh;

/// Can read single bit value.
///
/// Basically alias for [`IsBitHigh`].
pub trait ReadBit {
    /// My type.
    type Type;

    /// Read single bit.
    fn read_bit(&self, index: Self::Type) -> bool;
}

/// Implement [`ReadBit`] for given type.
macro_rules! ImplementReadBit {
    ($type:ty) => {
        impl ReadBit for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn read_bit(&self, index: Self) -> bool {
                self.is_bit_high(index)
            }
        }
    };
}

ImplementReadBit!(u8);
ImplementReadBit!(u32);
ImplementReadBit!(u64);

// TODO: tests
