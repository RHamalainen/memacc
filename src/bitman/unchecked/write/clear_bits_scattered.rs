use crate::bitman::ClearBit;

/// Can clear multiple bits in non-continuous manner.
pub trait ClearBitsScattered {
    /// My type.
    type Type;

    /// Clear multiple bits in non-continuous manner.
    fn clear_bits_scattered(self, indices: &[Self::Type]) -> Self::Type;
}

/// Implement [`ClearBitsScattered`] for given type.
macro_rules! ImplementClearBitsScattered {
    ($type:ty) => {
        impl ClearBitsScattered for $type {
            type Type = Self;
            #[inline]
            fn clear_bits_scattered(self, indices: &[Self::Type]) -> Self::Type {
                let mut result = self;
                for index in indices {
                    result = result.clear_bit(*index);
                }
                result
            }
        }
    };
}

ImplementClearBitsScattered!(u8);
ImplementClearBitsScattered!(u32);
ImplementClearBitsScattered!(u64);

// TODO: tests
