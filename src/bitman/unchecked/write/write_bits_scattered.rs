use crate::bitman::Bits;
use crate::bitman::ClearBit;
use crate::bitman::ReadBit;
use crate::bitman::SetBit;

/// Can write values of multiple bits in non-continuous manner.
pub trait WriteBitsScattered {
    /// My type.
    type Type;

    /// Write multiple bits in non-continuous manner.
    fn write_bits_scattered(self, value: Self::Type, indices: &[Self::Type]) -> Self::Type;
}

/// Implement [`WriteBitsScattered`] for given type.
macro_rules! ImplementWriteBitsScattered {
    ($type:ty) => {
        impl WriteBitsScattered for $type {
            type Type = Self;
            #[inline]
            fn write_bits_scattered(self, value: Self::Type, indices: &[Self::Type]) -> Self::Type {
                assert!(
                    Self::try_from(indices.len()).unwrap() <= Self::bits(),
                    "amount of bits written must be less than or equal to result type's bit width"
                );
                let mut result = self;
                for (index_source, index_result) in indices.iter().enumerate() {
                    result = if value.read_bit(Self::try_from(index_source).unwrap()) {
                        result.set_bit(Self::from(*index_result))
                    } else {
                        result.clear_bit(Self::from(*index_result))
                    };
                }
                result
            }
        }
    };
}

ImplementWriteBitsScattered!(u8);
ImplementWriteBitsScattered!(u32);
ImplementWriteBitsScattered!(u64);

// TODO: tests
