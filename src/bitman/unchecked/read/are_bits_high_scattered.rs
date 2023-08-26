use crate::bitman::IsBitLow;

/// Can check if multiple bits are high in non-continuous manner.
pub trait AreBitsHighScattered {
    /// My type.
    type Type;

    /// Check if multiple bits are high in non-continuous manner.
    fn are_bits_high_scattered(&self, indices: &[Self::Type]) -> bool;
}

/// Implement [`AreBitsHighScattered`] for given type.
macro_rules! ImplementAreBitsHighScattered {
    ($type:ty) => {
        impl AreBitsHighScattered for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn are_bits_high_scattered(&self, indices: &[Self::Type]) -> bool {
                for index in indices {
                    if self.is_bit_low(*index) {
                        return false;
                    }
                }
                true
            }
        }
    };
}

ImplementAreBitsHighScattered!(u8);
ImplementAreBitsHighScattered!(u32);
ImplementAreBitsHighScattered!(u64);

// TODO: tests
