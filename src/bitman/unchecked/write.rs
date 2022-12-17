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
                assert!((index as u32) < Self::BITS, "Invalid index.");

                // Move high bit to target index.
                let mask = (0b1 as Self).shl(index);

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
                assert!((index as u32) < Self::BITS, "Invalid index.");

                // Move low bit to target index.
                let mask = (0b1 as Self).shl(index);

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
                assert!((index as u32) < Self::BITS, "Invalid index.");
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
                let bits = Self::BITS as Self;
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

macro_rules! ImplementClearBits {
    ($type:ty) => {
        impl ClearBits for $type {
            type Type = Self;
            fn clear_bits(&self, range: RangeInclusive<Self::Type>) -> Self::Type {
                let bits = Self::BITS as Self;
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

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: tests for clear_bits, set_bits

    #[test]
    fn test_set_bit() {
        assert_eq!(0b0000_0000u8.set_bit(0), 0b0000_0001u8);
        assert_eq!(0b0000_0000u8.set_bit(1), 0b0000_0010u8);
        assert_eq!(0b0000_0000u8.set_bit(2), 0b0000_0100u8);
        assert_eq!(0b0000_0000u8.set_bit(3), 0b0000_1000u8);
        assert_eq!(0b0000_0000u8.set_bit(4), 0b0001_0000u8);
        assert_eq!(0b0000_0000u8.set_bit(5), 0b0010_0000u8);
        assert_eq!(0b0000_0000u8.set_bit(6), 0b0100_0000u8);
        assert_eq!(0b0000_0000u8.set_bit(7), 0b1000_0000u8);
    }

    #[test]
    fn test_set_bit_noisy() {
        assert_eq!(0b1010_0110u8.set_bit(0), 0b1010_0111u8);
        assert_eq!(0b1010_0110u8.set_bit(1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.set_bit(2), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.set_bit(3), 0b1010_1110u8);
        assert_eq!(0b1010_0110u8.set_bit(4), 0b1011_0110u8);
        assert_eq!(0b1010_0110u8.set_bit(5), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.set_bit(6), 0b1110_0110u8);
        assert_eq!(0b1010_0110u8.set_bit(7), 0b1010_0110u8);
    }

    #[test]
    #[should_panic]
    fn test_set_bit_panics() {
        0b0000_0000u8.set_bit(8);
    }

    #[test]
    fn test_clear_bit() {
        assert_eq!(0b1111_1111u8.clear_bit(0), 0b1111_1110u8);
        assert_eq!(0b1111_1111u8.clear_bit(1), 0b1111_1101u8);
        assert_eq!(0b1111_1111u8.clear_bit(2), 0b1111_1011u8);
        assert_eq!(0b1111_1111u8.clear_bit(3), 0b1111_0111u8);
        assert_eq!(0b1111_1111u8.clear_bit(4), 0b1110_1111u8);
        assert_eq!(0b1111_1111u8.clear_bit(5), 0b1101_1111u8);
        assert_eq!(0b1111_1111u8.clear_bit(6), 0b1011_1111u8);
        assert_eq!(0b1111_1111u8.clear_bit(7), 0b0111_1111u8);
    }

    #[test]
    fn test_clear_bit_noisy() {
        assert_eq!(0b1010_0110u8.clear_bit(0), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.clear_bit(1), 0b1010_0100u8);
        assert_eq!(0b1010_0110u8.clear_bit(2), 0b1010_0010u8);
        assert_eq!(0b1010_0110u8.clear_bit(3), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.clear_bit(4), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.clear_bit(5), 0b1000_0110u8);
        assert_eq!(0b1010_0110u8.clear_bit(6), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.clear_bit(7), 0b0010_0110u8);
    }

    #[test]
    #[should_panic]
    fn test_clear_bit_panics() {
        0b1111_1111u8.clear_bit(8);
    }

    #[test]
    fn test_set_bits() {
        assert_eq!(0b0000_0000u8.set_bits(0..=7), 0b1111_1111u8);
    }

    #[test]
    fn test_clear_bits() {
        assert_eq!(0b1111_1111u8.clear_bits(0..=7), 0b0000_0000u8);
    }

    #[test]
    fn test_write_bits() {
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 2), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 3), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 4), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 5), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 6), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 7), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 8), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1010_0110u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1010_0110u8, 2), 0b0000_0010u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1010_0110u8, 3), 0b0000_0110u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1010_0110u8, 4), 0b0000_0110u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1010_0110u8, 5), 0b0000_0110u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1010_0110u8, 6), 0b0010_0110u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1010_0110u8, 7), 0b0010_0110u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1010_0110u8, 8), 0b1010_0110u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1100_1010u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1100_1010u8, 2), 0b0000_0010u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1100_1010u8, 3), 0b0000_0010u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1100_1010u8, 4), 0b0000_1010u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1100_1010u8, 5), 0b0000_1010u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1100_1010u8, 6), 0b0000_1010u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1100_1010u8, 7), 0b0100_1010u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1100_1010u8, 8), 0b1100_1010u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1111_1111u8, 1), 0b0000_0001u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1111_1111u8, 2), 0b0000_0011u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1111_1111u8, 3), 0b0000_0111u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1111_1111u8, 4), 0b0000_1111u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1111_1111u8, 5), 0b0001_1111u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1111_1111u8, 6), 0b0011_1111u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1111_1111u8, 7), 0b0111_1111u8);
        assert_eq!(0b0000_0000u8.write_bits(0, 0b1111_1111u8, 8), 0b1111_1111u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b0000_0000u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b0000_0000u8, 2), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b0000_0000u8, 3), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b0000_0000u8, 4), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b0000_0000u8, 5), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b0000_0000u8, 6), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b0000_0000u8, 7), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1010_0110u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1010_0110u8, 2), 0b0000_0100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1010_0110u8, 3), 0b0000_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1010_0110u8, 4), 0b0000_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1010_0110u8, 5), 0b0000_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1010_0110u8, 6), 0b0100_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1010_0110u8, 7), 0b0100_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1100_1010u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1100_1010u8, 2), 0b0000_0100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1100_1010u8, 3), 0b0000_0100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1100_1010u8, 4), 0b0001_0100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1100_1010u8, 5), 0b0001_0100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1100_1010u8, 6), 0b0001_0100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1100_1010u8, 7), 0b1001_0100u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1111_1111u8, 1), 0b0000_0010u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1111_1111u8, 2), 0b0000_0110u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1111_1111u8, 3), 0b0000_1110u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1111_1111u8, 4), 0b0001_1110u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1111_1111u8, 5), 0b0011_1110u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1111_1111u8, 6), 0b0111_1110u8);
        assert_eq!(0b0000_0000u8.write_bits(1, 0b1111_1111u8, 7), 0b1111_1110u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b0000_0000u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b0000_0000u8, 2), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b0000_0000u8, 3), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b0000_0000u8, 4), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b0000_0000u8, 5), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b0000_0000u8, 6), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1010_0110u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1010_0110u8, 2), 0b0000_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1010_0110u8, 3), 0b0001_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1010_0110u8, 4), 0b0001_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1010_0110u8, 5), 0b0001_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1010_0110u8, 6), 0b1001_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1100_1010u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1100_1010u8, 2), 0b0000_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1100_1010u8, 3), 0b0000_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1100_1010u8, 4), 0b0010_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1100_1010u8, 5), 0b0010_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1100_1010u8, 6), 0b0010_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1111_1111u8, 1), 0b0000_0100u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1111_1111u8, 2), 0b0000_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1111_1111u8, 3), 0b0001_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1111_1111u8, 4), 0b0011_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1111_1111u8, 5), 0b0111_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(2, 0b1111_1111u8, 6), 0b1111_1100u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b0000_0000u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b0000_0000u8, 2), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b0000_0000u8, 3), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b0000_0000u8, 4), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b0000_0000u8, 5), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1010_0110u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1010_0110u8, 2), 0b0001_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1010_0110u8, 3), 0b0011_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1010_0110u8, 4), 0b0011_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1010_0110u8, 5), 0b0011_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1100_1010u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1100_1010u8, 2), 0b0001_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1100_1010u8, 3), 0b0001_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1100_1010u8, 4), 0b0101_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1100_1010u8, 5), 0b0101_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1111_1111u8, 1), 0b0000_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1111_1111u8, 2), 0b0001_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1111_1111u8, 3), 0b0011_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1111_1111u8, 4), 0b0111_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(3, 0b1111_1111u8, 5), 0b1111_1000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b0000_0000u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b0000_0000u8, 2), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b0000_0000u8, 3), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b0000_0000u8, 4), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1010_0110u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1010_0110u8, 2), 0b0010_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1010_0110u8, 3), 0b0110_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1010_0110u8, 4), 0b0110_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1100_1010u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1100_1010u8, 2), 0b0010_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1100_1010u8, 3), 0b0010_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1100_1010u8, 4), 0b1010_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1111_1111u8, 1), 0b0001_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1111_1111u8, 2), 0b0011_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1111_1111u8, 3), 0b0111_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(4, 0b1111_1111u8, 4), 0b1111_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b0000_0000u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b0000_0000u8, 2), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b0000_0000u8, 3), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b1010_0110u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b1010_0110u8, 2), 0b0100_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b1010_0110u8, 3), 0b1100_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b1100_1010u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b1100_1010u8, 2), 0b0100_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b1100_1010u8, 3), 0b0100_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b1111_1111u8, 1), 0b0010_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b1111_1111u8, 2), 0b0110_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(5, 0b1111_1111u8, 3), 0b1110_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(6, 0b0000_0000u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(6, 0b0000_0000u8, 2), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(6, 0b1010_0110u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(6, 0b1010_0110u8, 2), 0b1000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(6, 0b1100_1010u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(6, 0b1100_1010u8, 2), 0b1000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(6, 0b1111_1111u8, 1), 0b0100_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(6, 0b1111_1111u8, 2), 0b1100_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(7, 0b0000_0000u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(7, 0b1010_0110u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(7, 0b1100_1010u8, 1), 0b0000_0000u8);
        assert_eq!(0b0000_0000u8.write_bits(7, 0b1111_1111u8, 1), 0b1000_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b0000_0000u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b0000_0000u8, 2), 0b1010_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b0000_0000u8, 3), 0b1010_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b0000_0000u8, 4), 0b1010_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b0000_0000u8, 5), 0b1010_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b0000_0000u8, 6), 0b1000_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b0000_0000u8, 7), 0b1000_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b0000_0000u8, 8), 0b0000_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1010_0110u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1010_0110u8, 2), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1010_0110u8, 3), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1010_0110u8, 4), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1010_0110u8, 5), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1010_0110u8, 6), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1010_0110u8, 7), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1010_0110u8, 8), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1100_1010u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1100_1010u8, 2), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1100_1010u8, 3), 0b1010_0010u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1100_1010u8, 4), 0b1010_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1100_1010u8, 5), 0b1010_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1100_1010u8, 6), 0b1000_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1100_1010u8, 7), 0b1100_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1100_1010u8, 8), 0b1100_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1111_1111u8, 1), 0b1010_0111u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1111_1111u8, 2), 0b1010_0111u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1111_1111u8, 3), 0b1010_0111u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1111_1111u8, 4), 0b1010_1111u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1111_1111u8, 5), 0b1011_1111u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1111_1111u8, 6), 0b1011_1111u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1111_1111u8, 7), 0b1111_1111u8);
        assert_eq!(0b1010_0110u8.write_bits(0, 0b1111_1111u8, 8), 0b1111_1111u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b0000_0000u8, 1), 0b1010_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b0000_0000u8, 2), 0b1010_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b0000_0000u8, 3), 0b1010_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b0000_0000u8, 4), 0b1010_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b0000_0000u8, 5), 0b1000_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b0000_0000u8, 6), 0b1000_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b0000_0000u8, 7), 0b0000_0000u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1010_0110u8, 1), 0b1010_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1010_0110u8, 2), 0b1010_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1010_0110u8, 3), 0b1010_1100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1010_0110u8, 4), 0b1010_1100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1010_0110u8, 5), 0b1000_1100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1010_0110u8, 6), 0b1100_1100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1010_0110u8, 7), 0b0100_1100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1100_1010u8, 1), 0b1010_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1100_1010u8, 2), 0b1010_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1100_1010u8, 3), 0b1010_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1100_1010u8, 4), 0b1011_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1100_1010u8, 5), 0b1001_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1100_1010u8, 6), 0b1001_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1100_1010u8, 7), 0b1001_0100u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1111_1111u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1111_1111u8, 2), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1111_1111u8, 3), 0b1010_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1111_1111u8, 4), 0b1011_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1111_1111u8, 5), 0b1011_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1111_1111u8, 6), 0b1111_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(1, 0b1111_1111u8, 7), 0b1111_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b0000_0000u8, 1), 0b1010_0010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b0000_0000u8, 2), 0b1010_0010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b0000_0000u8, 3), 0b1010_0010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b0000_0000u8, 4), 0b1000_0010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b0000_0000u8, 5), 0b1000_0010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b0000_0000u8, 6), 0b0000_0010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1010_0110u8, 1), 0b1010_0010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1010_0110u8, 2), 0b1010_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1010_0110u8, 3), 0b1011_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1010_0110u8, 4), 0b1001_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1010_0110u8, 5), 0b1001_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1010_0110u8, 6), 0b1001_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1100_1010u8, 1), 0b1010_0010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1100_1010u8, 2), 0b1010_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1100_1010u8, 3), 0b1010_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1100_1010u8, 4), 0b1010_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1100_1010u8, 5), 0b1010_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1100_1010u8, 6), 0b0010_1010u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1111_1111u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1111_1111u8, 2), 0b1010_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1111_1111u8, 3), 0b1011_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1111_1111u8, 4), 0b1011_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1111_1111u8, 5), 0b1111_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(2, 0b1111_1111u8, 6), 0b1111_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b0000_0000u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b0000_0000u8, 2), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b0000_0000u8, 3), 0b1000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b0000_0000u8, 4), 0b1000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b0000_0000u8, 5), 0b0000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1010_0110u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1010_0110u8, 2), 0b1011_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1010_0110u8, 3), 0b1011_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1010_0110u8, 4), 0b1011_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1010_0110u8, 5), 0b0011_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1100_1010u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1100_1010u8, 2), 0b1011_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1100_1010u8, 3), 0b1001_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1100_1010u8, 4), 0b1101_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1100_1010u8, 5), 0b0101_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1111_1111u8, 1), 0b1010_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1111_1111u8, 2), 0b1011_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1111_1111u8, 3), 0b1011_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1111_1111u8, 4), 0b1111_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(3, 0b1111_1111u8, 5), 0b1111_1110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b0000_0000u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b0000_0000u8, 2), 0b1000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b0000_0000u8, 3), 0b1000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b0000_0000u8, 4), 0b0000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1010_0110u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1010_0110u8, 2), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1010_0110u8, 3), 0b1110_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1010_0110u8, 4), 0b0110_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1100_1010u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1100_1010u8, 2), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1100_1010u8, 3), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1100_1010u8, 4), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1111_1111u8, 1), 0b1011_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1111_1111u8, 2), 0b1011_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1111_1111u8, 3), 0b1111_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(4, 0b1111_1111u8, 4), 0b1111_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b0000_0000u8, 1), 0b1000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b0000_0000u8, 2), 0b1000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b0000_0000u8, 3), 0b0000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b1010_0110u8, 1), 0b1000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b1010_0110u8, 2), 0b1100_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b1010_0110u8, 3), 0b1100_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b1100_1010u8, 1), 0b1000_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b1100_1010u8, 2), 0b1100_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b1100_1010u8, 3), 0b0100_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b1111_1111u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b1111_1111u8, 2), 0b1110_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(5, 0b1111_1111u8, 3), 0b1110_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(6, 0b0000_0000u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(6, 0b0000_0000u8, 2), 0b0010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(6, 0b1010_0110u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(6, 0b1010_0110u8, 2), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(6, 0b1100_1010u8, 1), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(6, 0b1100_1010u8, 2), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(6, 0b1111_1111u8, 1), 0b1110_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(6, 0b1111_1111u8, 2), 0b1110_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(7, 0b0000_0000u8, 1), 0b0010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(7, 0b1010_0110u8, 1), 0b0010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(7, 0b1100_1010u8, 1), 0b0010_0110u8);
        assert_eq!(0b1010_0110u8.write_bits(7, 0b1111_1111u8, 1), 0b1010_0110u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b0000_0000u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b0000_0000u8, 2), 0b1100_1000u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b0000_0000u8, 3), 0b1100_1000u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b0000_0000u8, 4), 0b1100_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b0000_0000u8, 5), 0b1100_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b0000_0000u8, 6), 0b1100_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b0000_0000u8, 7), 0b1000_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b0000_0000u8, 8), 0b0000_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1010_0110u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1010_0110u8, 2), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1010_0110u8, 3), 0b1100_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1010_0110u8, 4), 0b1100_0110u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1010_0110u8, 5), 0b1100_0110u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1010_0110u8, 6), 0b1110_0110u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1010_0110u8, 7), 0b1010_0110u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1010_0110u8, 8), 0b1010_0110u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1100_1010u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1100_1010u8, 2), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1100_1010u8, 3), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1100_1010u8, 4), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1100_1010u8, 5), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1100_1010u8, 6), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1100_1010u8, 7), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1100_1010u8, 8), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1111_1111u8, 1), 0b1100_1011u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1111_1111u8, 2), 0b1100_1011u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1111_1111u8, 3), 0b1100_1111u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1111_1111u8, 4), 0b1100_1111u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1111_1111u8, 5), 0b1101_1111u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1111_1111u8, 6), 0b1111_1111u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1111_1111u8, 7), 0b1111_1111u8);
        assert_eq!(0b1100_1010u8.write_bits(0, 0b1111_1111u8, 8), 0b1111_1111u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b0000_0000u8, 1), 0b1100_1000u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b0000_0000u8, 2), 0b1100_1000u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b0000_0000u8, 3), 0b1100_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b0000_0000u8, 4), 0b1100_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b0000_0000u8, 5), 0b1100_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b0000_0000u8, 6), 0b1000_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b0000_0000u8, 7), 0b0000_0000u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1010_0110u8, 1), 0b1100_1000u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1010_0110u8, 2), 0b1100_1100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1010_0110u8, 3), 0b1100_1100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1010_0110u8, 4), 0b1100_1100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1010_0110u8, 5), 0b1100_1100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1010_0110u8, 6), 0b1100_1100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1010_0110u8, 7), 0b0100_1100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1100_1010u8, 1), 0b1100_1000u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1100_1010u8, 2), 0b1100_1100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1100_1010u8, 3), 0b1100_0100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1100_1010u8, 4), 0b1101_0100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1100_1010u8, 5), 0b1101_0100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1100_1010u8, 6), 0b1001_0100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1100_1010u8, 7), 0b1001_0100u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1111_1111u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1111_1111u8, 2), 0b1100_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1111_1111u8, 3), 0b1100_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1111_1111u8, 4), 0b1101_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1111_1111u8, 5), 0b1111_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1111_1111u8, 6), 0b1111_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(1, 0b1111_1111u8, 7), 0b1111_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b0000_0000u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b0000_0000u8, 2), 0b1100_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b0000_0000u8, 3), 0b1100_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b0000_0000u8, 4), 0b1100_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b0000_0000u8, 5), 0b1000_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b0000_0000u8, 6), 0b0000_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1010_0110u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1010_0110u8, 2), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1010_0110u8, 3), 0b1101_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1010_0110u8, 4), 0b1101_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1010_0110u8, 5), 0b1001_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1010_0110u8, 6), 0b1001_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1100_1010u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1100_1010u8, 2), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1100_1010u8, 3), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1100_1010u8, 4), 0b1110_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1100_1010u8, 5), 0b1010_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1100_1010u8, 6), 0b0010_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1111_1111u8, 1), 0b1100_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1111_1111u8, 2), 0b1100_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1111_1111u8, 3), 0b1101_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1111_1111u8, 4), 0b1111_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1111_1111u8, 5), 0b1111_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(2, 0b1111_1111u8, 6), 0b1111_1110u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b0000_0000u8, 1), 0b1100_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b0000_0000u8, 2), 0b1100_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b0000_0000u8, 3), 0b1100_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b0000_0000u8, 4), 0b1000_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b0000_0000u8, 5), 0b0000_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1010_0110u8, 1), 0b1100_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1010_0110u8, 2), 0b1101_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1010_0110u8, 3), 0b1111_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1010_0110u8, 4), 0b1011_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1010_0110u8, 5), 0b0011_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1100_1010u8, 1), 0b1100_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1100_1010u8, 2), 0b1101_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1100_1010u8, 3), 0b1101_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1100_1010u8, 4), 0b1101_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1100_1010u8, 5), 0b0101_0010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1111_1111u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1111_1111u8, 2), 0b1101_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1111_1111u8, 3), 0b1111_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1111_1111u8, 4), 0b1111_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(3, 0b1111_1111u8, 5), 0b1111_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b0000_0000u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b0000_0000u8, 2), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b0000_0000u8, 3), 0b1000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b0000_0000u8, 4), 0b0000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1010_0110u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1010_0110u8, 2), 0b1110_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1010_0110u8, 3), 0b1110_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1010_0110u8, 4), 0b0110_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1100_1010u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1100_1010u8, 2), 0b1110_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1100_1010u8, 3), 0b1010_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1100_1010u8, 4), 0b1010_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1111_1111u8, 1), 0b1101_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1111_1111u8, 2), 0b1111_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1111_1111u8, 3), 0b1111_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(4, 0b1111_1111u8, 4), 0b1111_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b0000_0000u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b0000_0000u8, 2), 0b1000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b0000_0000u8, 3), 0b0000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b1010_0110u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b1010_0110u8, 2), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b1010_0110u8, 3), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b1100_1010u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b1100_1010u8, 2), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b1100_1010u8, 3), 0b0100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b1111_1111u8, 1), 0b1110_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b1111_1111u8, 2), 0b1110_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(5, 0b1111_1111u8, 3), 0b1110_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(6, 0b0000_0000u8, 1), 0b1000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(6, 0b0000_0000u8, 2), 0b0000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(6, 0b1010_0110u8, 1), 0b1000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(6, 0b1010_0110u8, 2), 0b1000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(6, 0b1100_1010u8, 1), 0b1000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(6, 0b1100_1010u8, 2), 0b1000_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(6, 0b1111_1111u8, 1), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(6, 0b1111_1111u8, 2), 0b1100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(7, 0b0000_0000u8, 1), 0b0100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(7, 0b1010_0110u8, 1), 0b0100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(7, 0b1100_1010u8, 1), 0b0100_1010u8);
        assert_eq!(0b1100_1010u8.write_bits(7, 0b1111_1111u8, 1), 0b1100_1010u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b0000_0000u8, 1), 0b1111_1110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b0000_0000u8, 2), 0b1111_1100u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b0000_0000u8, 3), 0b1111_1000u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b0000_0000u8, 4), 0b1111_0000u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b0000_0000u8, 5), 0b1110_0000u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b0000_0000u8, 6), 0b1100_0000u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b0000_0000u8, 7), 0b1000_0000u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b0000_0000u8, 8), 0b0000_0000u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1010_0110u8, 1), 0b1111_1110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1010_0110u8, 2), 0b1111_1110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1010_0110u8, 3), 0b1111_1110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1010_0110u8, 4), 0b1111_0110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1010_0110u8, 5), 0b1110_0110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1010_0110u8, 6), 0b1110_0110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1010_0110u8, 7), 0b1010_0110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1010_0110u8, 8), 0b1010_0110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1100_1010u8, 1), 0b1111_1110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1100_1010u8, 2), 0b1111_1110u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1100_1010u8, 3), 0b1111_1010u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1100_1010u8, 4), 0b1111_1010u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1100_1010u8, 5), 0b1110_1010u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1100_1010u8, 6), 0b1100_1010u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1100_1010u8, 7), 0b1100_1010u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1100_1010u8, 8), 0b1100_1010u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1111_1111u8, 1), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1111_1111u8, 2), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1111_1111u8, 3), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1111_1111u8, 4), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1111_1111u8, 5), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1111_1111u8, 6), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1111_1111u8, 7), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(0, 0b1111_1111u8, 8), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b0000_0000u8, 1), 0b1111_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b0000_0000u8, 2), 0b1111_1001u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b0000_0000u8, 3), 0b1111_0001u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b0000_0000u8, 4), 0b1110_0001u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b0000_0000u8, 5), 0b1100_0001u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b0000_0000u8, 6), 0b1000_0001u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b0000_0000u8, 7), 0b0000_0001u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1010_0110u8, 1), 0b1111_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1010_0110u8, 2), 0b1111_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1010_0110u8, 3), 0b1111_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1010_0110u8, 4), 0b1110_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1010_0110u8, 5), 0b1100_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1010_0110u8, 6), 0b1100_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1010_0110u8, 7), 0b0100_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1100_1010u8, 1), 0b1111_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1100_1010u8, 2), 0b1111_1101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1100_1010u8, 3), 0b1111_0101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1100_1010u8, 4), 0b1111_0101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1100_1010u8, 5), 0b1101_0101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1100_1010u8, 6), 0b1001_0101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1100_1010u8, 7), 0b1001_0101u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1111_1111u8, 1), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1111_1111u8, 2), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1111_1111u8, 3), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1111_1111u8, 4), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1111_1111u8, 5), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1111_1111u8, 6), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(1, 0b1111_1111u8, 7), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b0000_0000u8, 1), 0b1111_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b0000_0000u8, 2), 0b1111_0011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b0000_0000u8, 3), 0b1110_0011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b0000_0000u8, 4), 0b1100_0011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b0000_0000u8, 5), 0b1000_0011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b0000_0000u8, 6), 0b0000_0011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1010_0110u8, 1), 0b1111_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1010_0110u8, 2), 0b1111_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1010_0110u8, 3), 0b1111_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1010_0110u8, 4), 0b1101_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1010_0110u8, 5), 0b1001_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1010_0110u8, 6), 0b1001_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1100_1010u8, 1), 0b1111_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1100_1010u8, 2), 0b1111_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1100_1010u8, 3), 0b1110_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1100_1010u8, 4), 0b1110_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1100_1010u8, 5), 0b1010_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1100_1010u8, 6), 0b0010_1011u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1111_1111u8, 1), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1111_1111u8, 2), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1111_1111u8, 3), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1111_1111u8, 4), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1111_1111u8, 5), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(2, 0b1111_1111u8, 6), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b0000_0000u8, 1), 0b1111_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b0000_0000u8, 2), 0b1110_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b0000_0000u8, 3), 0b1100_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b0000_0000u8, 4), 0b1000_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b0000_0000u8, 5), 0b0000_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1010_0110u8, 1), 0b1111_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1010_0110u8, 2), 0b1111_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1010_0110u8, 3), 0b1111_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1010_0110u8, 4), 0b1011_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1010_0110u8, 5), 0b0011_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1100_1010u8, 1), 0b1111_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1100_1010u8, 2), 0b1111_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1100_1010u8, 3), 0b1101_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1100_1010u8, 4), 0b1101_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1100_1010u8, 5), 0b0101_0111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1111_1111u8, 1), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1111_1111u8, 2), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1111_1111u8, 3), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1111_1111u8, 4), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(3, 0b1111_1111u8, 5), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b0000_0000u8, 1), 0b1110_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b0000_0000u8, 2), 0b1100_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b0000_0000u8, 3), 0b1000_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b0000_0000u8, 4), 0b0000_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1010_0110u8, 1), 0b1110_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1010_0110u8, 2), 0b1110_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1010_0110u8, 3), 0b1110_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1010_0110u8, 4), 0b0110_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1100_1010u8, 1), 0b1110_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1100_1010u8, 2), 0b1110_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1100_1010u8, 3), 0b1010_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1100_1010u8, 4), 0b1010_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1111_1111u8, 1), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1111_1111u8, 2), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1111_1111u8, 3), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(4, 0b1111_1111u8, 4), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b0000_0000u8, 1), 0b1101_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b0000_0000u8, 2), 0b1001_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b0000_0000u8, 3), 0b0001_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b1010_0110u8, 1), 0b1101_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b1010_0110u8, 2), 0b1101_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b1010_0110u8, 3), 0b1101_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b1100_1010u8, 1), 0b1101_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b1100_1010u8, 2), 0b1101_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b1100_1010u8, 3), 0b0101_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b1111_1111u8, 1), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b1111_1111u8, 2), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(5, 0b1111_1111u8, 3), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(6, 0b0000_0000u8, 1), 0b1011_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(6, 0b0000_0000u8, 2), 0b0011_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(6, 0b1010_0110u8, 1), 0b1011_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(6, 0b1010_0110u8, 2), 0b1011_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(6, 0b1100_1010u8, 1), 0b1011_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(6, 0b1100_1010u8, 2), 0b1011_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(6, 0b1111_1111u8, 1), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(6, 0b1111_1111u8, 2), 0b1111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(7, 0b0000_0000u8, 1), 0b0111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(7, 0b1010_0110u8, 1), 0b0111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(7, 0b1100_1010u8, 1), 0b0111_1111u8);
        assert_eq!(0b1111_1111u8.write_bits(7, 0b1111_1111u8, 1), 0b1111_1111u8);
    }

    #[test]
    #[should_panic]
    fn test_write_bits_panics_1() {
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 0), 0b0000_0000u8);
    }

    #[test]
    #[should_panic]
    fn test_write_bits_panics_2() {
        assert_eq!(0b0000_0000u8.write_bits(0, 0b0000_0000u8, 9), 0b0000_0000u8);
    }

    #[test]
    #[should_panic]
    fn test_write_bits_panics_3() {
        assert_eq!(0b0000_0000u8.write_bits(8, 0b0000_0000u8, 7), 0b0000_0000u8);
    }

    #[test]
    #[should_panic]
    fn test_write_bits_panics_4() {
        assert_eq!(0b0000_0000u8.write_bits(7, 0b0000_0000u8, 2), 0b0000_0000u8);
    }
}
