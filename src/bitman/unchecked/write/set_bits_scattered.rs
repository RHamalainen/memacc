use crate::bitman::SetBit;

/// Can set multiple bits in non-continuous manner.
pub trait SetBitsScattered {
    /// My type.
    type Type;

    /// Set multiple bits in non-continuous manner.
    fn set_bits_scattered(self, indices: &[Self::Type]) -> Self::Type;
}

/// Implement [`SetBitsScattered`] for given type.
macro_rules! ImplementSetBitsScattered {
    ($type:ty) => {
        impl SetBitsScattered for $type {
            type Type = Self;
            #[inline]
            fn set_bits_scattered(self, indices: &[Self::Type]) -> Self::Type {
                let mut result = self;
                for index in indices {
                    result = result.set_bit(*index);
                }
                result
            }
        }
    };
}

ImplementSetBitsScattered!(u8);
ImplementSetBitsScattered!(u32);
ImplementSetBitsScattered!(u64);

#[cfg(test)]
mod tests {
    use super::SetBitsScattered;
    use crate::bitman::Zeros;

    #[test]
    fn test_set_bits_scattered() {
        // TODO: more comprehensive testing
        assert_eq!(u8::zeros().set_bits_scattered(&[]), 0b0000_0000);
        assert_eq!(u8::zeros().set_bits_scattered(&[0]), 0b0000_0001);
        assert_eq!(u8::zeros().set_bits_scattered(&[1]), 0b0000_0010);
        assert_eq!(u8::zeros().set_bits_scattered(&[2]), 0b0000_0100);
        assert_eq!(u8::zeros().set_bits_scattered(&[3]), 0b0000_1000);
        assert_eq!(u8::zeros().set_bits_scattered(&[4]), 0b0001_0000);
        assert_eq!(u8::zeros().set_bits_scattered(&[5]), 0b0010_0000);
        assert_eq!(u8::zeros().set_bits_scattered(&[6]), 0b0100_0000);
        assert_eq!(u8::zeros().set_bits_scattered(&[7]), 0b1000_0000);

        assert_eq!(u8::zeros().set_bits_scattered(&[0, 0]), 0b0000_0001);
        assert_eq!(u8::zeros().set_bits_scattered(&[1, 0]), 0b0000_0011);
        assert_eq!(u8::zeros().set_bits_scattered(&[2, 0]), 0b0000_0101);
        assert_eq!(u8::zeros().set_bits_scattered(&[3, 0]), 0b0000_1001);
        assert_eq!(u8::zeros().set_bits_scattered(&[4, 0]), 0b0001_0001);
        assert_eq!(u8::zeros().set_bits_scattered(&[5, 0]), 0b0010_0001);
        assert_eq!(u8::zeros().set_bits_scattered(&[6, 0]), 0b0100_0001);
        assert_eq!(u8::zeros().set_bits_scattered(&[7, 0]), 0b1000_0001);
    }

    #[test]
    #[should_panic]
    fn test_set_bits_scattered_panics() {
        assert_eq!(u8::zeros().set_bits_scattered(&[8]), 0b0000_0000);
    }
}
