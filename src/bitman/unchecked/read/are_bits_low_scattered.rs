use crate::bitman::IsBitHigh;

/// Can check if multiple bits are low in non-continuous manner.
pub trait AreBitsLowScattered {
    /// My type.
    type Type;

    /// Check if multiple bits are low in non-continuous manner.
    fn are_bits_low_scattered(&self, indices: &[Self::Type]) -> bool;
}

/// Implement [`AreBitsLowScattered`] for given type.
macro_rules! ImplementAreBitsLowScattered {
    ($type:ty) => {
        impl AreBitsLowScattered for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn are_bits_low_scattered(&self, indices: &[Self::Type]) -> bool {
                for index in indices {
                    if self.is_bit_high(*index) {
                        return false;
                    }
                }
                true
            }
        }
    };
}

ImplementAreBitsLowScattered!(u8);
ImplementAreBitsLowScattered!(u32);
ImplementAreBitsLowScattered!(u64);

// TODO: tests
