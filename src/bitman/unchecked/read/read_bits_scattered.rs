use crate::bitman::Bits;
use crate::bitman::ReadBit;
use crate::bitman::SetBit;

/// Can read values of multiple bits in non-continuous manner.
pub trait ReadBitsScattered {
    /// My type.
    type Type;

    /// Read multiple bits in non-continuous manner.
    fn read_bits_scattered(&self, indices: &[Self::Type]) -> Self::Type;
}

/// Implement [`ReadBitsScattered`] for given type.
macro_rules! ImplementReadBitsScattered {
    ($type:ty) => {
        impl ReadBitsScattered for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn read_bits_scattered(&self, indices: &[Self::Type]) -> Self::Type {
                assert!(
                    Self::try_from(indices.len()).unwrap() <= Self::bits(),
                    "amount of bits read must be less than or equal to result type's bit width"
                );
                let mut result = 0;
                for (index_result, index_source) in indices.iter().enumerate() {
                    if self.read_bit(Self::from(*index_source)) {
                        result = result.set_bit(Self::try_from(index_result).unwrap());
                    }
                }
                result
            }
        }
    };
}

ImplementReadBitsScattered!(u8);
ImplementReadBitsScattered!(u32);
ImplementReadBitsScattered!(u64);

// TODO: tests
