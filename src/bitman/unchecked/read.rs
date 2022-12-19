//! Tools to read bits.

// TODO: missing tests

mod tests;
mod tests_generated;

use crate::bitman::SetBit;
use core::assert;
use core::ops::BitAnd;
use core::ops::Not;
use core::ops::RangeInclusive;
use core::ops::Shl;
use core::ops::Shr;

/// Can check if single bit is low.
pub trait IsBitLow {
    /// My type.
    type Type;

    /// Check if single bit is low.
    fn is_bit_low(&self, index: Self::Type) -> bool;
}

/// Implement `IsBitLow` for given type.
macro_rules! ImplementIsBitLow {
    ($type:ty) => {
        impl IsBitLow for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn is_bit_low(&self, index: Self) -> bool {
                assert!(u32::from(index) < Self::BITS, "Invalid index.");

                // Move target bit to index 0.
                let temporary_1 = self.shr(index);

                // Clear all bits except index 0.
                let temporary_2 = 0b1.bitand(temporary_1);

                // If value is zero, then the bit at given index is low.
                temporary_2 == 0
            }
        }
    };
}

ImplementIsBitLow!(u8);
ImplementIsBitLow!(u32);

/// Can check if single bit is high.
pub trait IsBitHigh {
    /// My type.
    type Type;

    /// Check if single bit is high.
    fn is_bit_high(&self, index: Self::Type) -> bool;
}

/// Implement `IsBitHigh` for given type.
macro_rules! ImplementIsBitHigh {
    ($type:ty) => {
        impl IsBitHigh for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn is_bit_high(&self, index: Self) -> bool {
                assert!(u32::from(index) < Self::BITS, "Invalid index.");

                // Move target bit to index 0.
                let temporary_1 = self.shr(index);

                // Clear all bits except index 0.
                let temporary_2 = 0b1.bitand(temporary_1);

                // If value is one, then the bit at given index is high.
                temporary_2 == 1
            }
        }
    };
}

ImplementIsBitHigh!(u8);
ImplementIsBitHigh!(u32);

/// Can read single bit value.
pub trait ReadBit {
    /// My type.
    type Type;

    /// Read single bit.
    fn read_bit(&self, index: Self::Type) -> bool;
}

/// Implement `ReadBit` for given type.
macro_rules! ImplementReadBit {
    ($type:ty) => {
        impl ReadBit for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn read_bit(&self, index: Self) -> bool {
                assert!(u32::from(index) < Self::BITS, "Invalid index.");

                // Move target bit to index 0.
                let temporary_1 = self.shr(index);

                // Clear all bits except index 0.
                let temporary_2 = 0b1.bitand(temporary_1);

                // If byte value is one, then the bit at given index is high.
                temporary_2 == 1
            }
        }
    };
}

ImplementReadBit!(u8);
ImplementReadBit!(u32);

/// Can check if multiple bits are low.
pub trait AreBitsLow {
    /// My type.
    type Type;

    /// Check if multiple bits are low.
    fn are_bits_low(&self, range: RangeInclusive<Self::Type>) -> bool;
}

/// Implement `AreBitsLow` for given type.
macro_rules! ImplementAreBitsLow {
    ($type:ty) => {
        impl AreBitsLow for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn are_bits_low(&self, range: RangeInclusive<Self>) -> bool {
                for index in range {
                    if self.is_bit_high(index) {
                        return false;
                    }
                }
                true
            }
        }
    };
}

ImplementAreBitsLow!(u8);
ImplementAreBitsLow!(u32);

/// Can check if multiple bits are high.
pub trait AreBitsHigh {
    /// My type.
    type Type;

    /// Check if multiple bits are high.
    fn are_bits_high(&self, range: RangeInclusive<Self::Type>) -> bool;
}

/// Implement `AreBitsHigh` for given type.
macro_rules! ImplementAreBitsHigh {
    ($type:ty) => {
        impl AreBitsHigh for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn are_bits_high(&self, range: RangeInclusive<Self>) -> bool {
                for index in range {
                    if self.is_bit_low(index) {
                        return false;
                    }
                }
                true
            }
        }
    };
}

ImplementAreBitsHigh!(u8);
ImplementAreBitsHigh!(u32);

/// Can read values of multiple bits.
pub trait ReadBits {
    /// My type.
    type Type;

    /// Read multiple bits.
    fn read_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type;
}

/// Implement `ReadBits` for given type.
macro_rules! ImplementReadBits {
    ($type:ty) => {
        impl ReadBits for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn read_bits(&self, range: RangeInclusive<Self>) -> Self {
                let bits = Self::try_from(Self::BITS).unwrap();
                let start = *range.start();
                let end = *range.end();

                assert!(range.is_empty().not(), "Can not read empty range of bits.");
                assert!(
                    end < bits,
                    "Range end {end} must be less than type's bitwidth {bits}.",
                );

                // Clear bits lower than range start.
                let temporary_1 = self.shr(start);
                let temporary_2 = temporary_1.shl(start);

                // Clear bits higher than range end.
                #[allow(clippy::arithmetic_side_effects)]
                let amount = bits - end - 1;
                let temporary_3 = temporary_2.shl(amount);
                let temporary_4 = temporary_3.shr(amount);

                // Move bit range to index 0.
                let temporary_5 = temporary_4.shr(start);

                temporary_5
            }
        }
    };
}

ImplementReadBits!(u8);
ImplementReadBits!(u32);

/// Can check if multiple bits are low in non-continuous manner.
pub trait AreBitsLowScattered {
    /// My type.
    type Type;

    /// Check if multiple bits are low in non-continuous manner.
    fn are_bits_low_scattered(&self, indices: &[Self::Type]) -> bool;
}

/// Implement `AreBitsLowScattered` for given type.
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

/// Can check if multiple bits are high in non-continuous manner.
pub trait AreBitsHighScattered {
    /// My type.
    type Type;

    /// Check if multiple bits are high in non-continuous manner.
    fn are_bits_high_scattered(&self, indices: &[Self::Type]) -> bool;
}

/// Implement `AreBitsHighScattered` for given type.
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

/// Can read values of multiple bits in non-continuous manner.
pub trait ReadBitsScattered {
    /// My type.
    type Type;

    /// Read multiple bits in non-continuous manner.
    fn read_bits_scattered(&self, indices: &[Self::Type]) -> Self::Type;
}

/// Implement `ReadBitsScattered` for given type.
macro_rules! ImplementReadBitsScattered {
    ($type:ty) => {
        impl ReadBitsScattered for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn read_bits_scattered(&self, indices: &[Self::Type]) -> Self::Type {
                let bits = Self::try_from(Self::BITS).unwrap();
                assert!(
                    indices.len() <= usize::try_from(bits).unwrap(),
                    "Amount of bits read must be less than or equal to result type's bit width."
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
