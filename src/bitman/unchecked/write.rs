//! Tools to write bits.

mod tests;
mod tests_generated_write_bits;
mod tests_generated_write_bits_scattered;

use crate::bitman::ReadBit;
use core::assert;
use core::ops::BitAnd;
use core::ops::BitOr;
use core::ops::Not;
use core::ops::RangeInclusive;
use core::ops::Shl;
use core::ops::Shr;

/// Can set single bit high.
pub trait SetBit {
    /// My type.
    type Type;

    /// Set single bit high.
    fn set_bit(&self, index: Self::Type) -> Self::Type;
}

/// Implement `SetBit` for given type.
macro_rules! ImplementSetBit {
    ($type:ty) => {
        impl SetBit for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn set_bit(&self, index: Self) -> Self {
                assert!(u32::from(index) < Self::BITS, "Invalid index.");

                // Move high bit to target index.
                let mask = Self::from(0b1u8).shl(index);

                // Set bit at target index to high.
                self.bitor(mask)
            }
        }
    };
}

ImplementSetBit!(u8);
ImplementSetBit!(u32);

/// Can set single bit low.
pub trait ClearBit {
    /// My type.
    type Type;

    /// Set single bit low.
    fn clear_bit(&self, index: Self::Type) -> Self::Type;
}

/// Implement `ClearBit` for given type.
macro_rules! ImplementClearBit {
    ($type:ty) => {
        impl ClearBit for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn clear_bit(&self, index: Self) -> Self {
                assert!(u32::from(index) < Self::BITS, "Invalid index.");

                // Move low bit to target index.
                let mask = Self::from(0b1u8).shl(index);

                // Make other indices high and target index low.
                let mask = mask.not();

                // Set bit at target index to low.
                self.bitand(mask)
            }
        }
    };
}

ImplementClearBit!(u8);
ImplementClearBit!(u32);

/// Can write single bit value.
pub trait WriteBit {
    /// My type.
    type Type;

    /// Write single bit.
    fn write_bit(&self, index: Self::Type, value: bool) -> Self::Type;
}

/// Implement `WriteBit` for given type.
macro_rules! ImplementWriteBit {
    ($type:ty) => {
        impl WriteBit for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn write_bit(&self, index: Self, value: bool) -> Self {
                assert!(u32::from(index) < Self::BITS, "Invalid index.");
                if value {
                    self.set_bit(index)
                } else {
                    self.clear_bit(index)
                }
            }
        }
    };
}

ImplementWriteBit!(u8);
ImplementWriteBit!(u32);

/// Can set multiple bits.
pub trait SetBits {
    /// My type.
    type Type;

    /// Set multiple bits.
    fn set_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type;
}

/// Implement `SetBits` for given type.
macro_rules! ImplementSetBits {
    ($type:ty) => {
        impl SetBits for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn set_bits(&self, range: RangeInclusive<Self>) -> Self {
                let bits = Self::try_from(Self::BITS).unwrap();
                let start = *range.start();
                let end = *range.end();

                assert!(range.is_empty().not(), "Can not set empty range of bits.");
                assert!(
                    end < bits,
                    "Range end {} must be less than type's bitwidth {}.",
                    end,
                    bits,
                );

                let mask: Self = 0;
                let mask = mask.not();

                // Clear bits lower than range start.
                let mask = mask.shr(start);
                let mask = mask.shl(start);

                // Clear bits higher than range end.
                #[allow(clippy::arithmetic_side_effects)]
                let amount = bits - end - 1;
                let mask = mask.shl(amount);
                let mask = mask.shr(amount);

                // Set masked bits.
                let value = self.bitor(mask);
                value
            }
        }
    };
}

ImplementSetBits!(u8);
ImplementSetBits!(u32);

/// Can clear multiple bits.
pub trait ClearBits {
    /// My type.
    type Type;

    /// Clear multiple bits.
    fn clear_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type;
}

/// Implement `ClearBits` for given type.
macro_rules! ImplementClearBits {
    ($type:ty) => {
        impl ClearBits for $type {
            type Type = Self;
            #[inline]
            fn clear_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type {
                let bits = Self::try_from(Self::BITS).unwrap();
                let start = *range.start();
                let end = *range.end();

                assert!(range.is_empty().not(), "Can not clear empty range of bits.");
                assert!(
                    end < bits,
                    "Range end {} must be less than type's bitwidth {}.",
                    end,
                    bits,
                );

                let mask: Self = 0;
                let mask = mask.not();

                // Clear bits lower than range start.
                let mask = mask.shr(start);
                let mask = mask.shl(start);

                // Clear bits higher than range end.
                #[allow(clippy::arithmetic_side_effects)]
                let amount = bits - end - 1;
                let mask = mask.shl(amount);
                let mask = mask.shr(amount);

                // Inverse mask.
                let mask = mask.not();

                // Set masked bits.
                let value = self.bitand(mask);
                value
            }
        }
    };
}

ImplementClearBits!(u8);
ImplementClearBits!(u32);

/// Can write multiple bits.
pub trait WriteBits {
    /// My type.
    type Type;

    /// Write multiple bits.
    fn write_bits(&self, start: Self::Type, value: Self::Type, length: Self::Type) -> Self::Type;
}

/// Implement `WriteBit` for given type.
macro_rules! ImplementWriteBits {
    ($type:ty) => {
        impl WriteBits for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn write_bits(&self, start: Self, value: Self, length: Self) -> Self {
                assert!(u32::from(start) < Self::BITS);
                assert!(0 < u32::from(length));
                assert!(u32::from(length) <= Self::BITS);
                assert!(u32::from(start + length) <= Self::BITS);
                let mut result = *self;
                let mut written = 0;
                for index_base in 0..Self::BITS {
                    if u32::from(start) <= index_base {
                        if length <= written {
                            break;
                        }
                        let index_value = index_base - u32::from(start);
                        let action = if value.read_bit(index_value as Self) {
                            Self::set_bit
                        } else {
                            Self::clear_bit
                        };
                        result = action(&result, index_base as Self);
                        written += 1;
                    }
                }
                result
            }
        }
    };
}

/// Implement `WriteBit` for given type.
/// TODO: this implementation fails.
macro_rules! ImplementWriteBits2 {
    ($type:ty) => {
        impl WriteBits for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn write_bits(&self, start: Self, value: Self, length: Self) -> Self {
                let bits = Self::BITS as Self;

                assert!(
                    start < bits,
                    "Start {} must be less than type's bitwidth {}.",
                    start,
                    bits
                );
                assert!(0 < length, "Length {} must be greater than zero.", length);
                assert!(
                    length <= bits,
                    "Length {} must be less or equal to type's bitwidth {}.",
                    length,
                    bits,
                );

                let mask1: Self = 0;
                let mask1 = mask1.set_bits(0..=length - 1);
                let mask1 = mask1.shl(start);
                let mask1 = mask1.not();

                let mask2 = value;
                let mask2 = mask2.shl(start);

                let value = self.bitand(mask1);
                let value = value.bitor(mask2);

                value
            }
        }
    };
}

ImplementWriteBits!(u8);
ImplementWriteBits!(u32);

/// Can write values of multiple bits in non-continuous manner.
pub trait WriteBitsScattered {
    /// My type.
    type Type;

    /// Write multiple bits in non-continuous manner.
    fn write_bits_scattered(&self, indices: &[Self::Type], value: Self::Type) -> Self::Type;
}

/// Implement `WriteBitsScattered` for given type.
macro_rules! ImplementWriteBitsScattered {
    ($type:ty) => {
        impl WriteBitsScattered for $type {
            type Type = Self;
            #[inline]
            fn write_bits_scattered(
                &self,
                indices: &[Self::Type],
                value: Self::Type,
            ) -> Self::Type {
                let bits = Self::try_from(Self::BITS).unwrap();
                assert!(
                    indices.len() <= usize::try_from(bits).unwrap(),
                    "Amount of bits written must be less than or equal to result type's bit width."
                );

                let mut result = *self;
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
