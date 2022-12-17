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
                assert!((index as u32) < Self::BITS, "Invalid index.");

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
                assert!((index as u32) < Self::BITS, "Invalid index.");

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
                assert!((index as u32) < Self::BITS, "Invalid index.");

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
                let bits = Self::BITS as Self;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bit_low_zeros() {
        assert!(0b0000_0000u8.is_bit_low(0));
        assert!(0b0000_0000u8.is_bit_low(1));
        assert!(0b0000_0000u8.is_bit_low(2));
        assert!(0b0000_0000u8.is_bit_low(3));
        assert!(0b0000_0000u8.is_bit_low(4));
        assert!(0b0000_0000u8.is_bit_low(5));
        assert!(0b0000_0000u8.is_bit_low(6));
        assert!(0b0000_0000u8.is_bit_low(7));
    }

    #[test]
    fn test_is_bit_low_ones() {
        assert!(0b1111_1111u8.is_bit_low(0).not());
        assert!(0b1111_1111u8.is_bit_low(1).not());
        assert!(0b1111_1111u8.is_bit_low(2).not());
        assert!(0b1111_1111u8.is_bit_low(3).not());
        assert!(0b1111_1111u8.is_bit_low(4).not());
        assert!(0b1111_1111u8.is_bit_low(5).not());
        assert!(0b1111_1111u8.is_bit_low(6).not());
        assert!(0b1111_1111u8.is_bit_low(7).not());
    }

    #[test]
    fn test_is_bit_low_noisy() {
        assert!(0b1010_0110u8.is_bit_low(0));
        assert!(0b1010_0110u8.is_bit_low(1).not());
        assert!(0b1010_0110u8.is_bit_low(2).not());
        assert!(0b1010_0110u8.is_bit_low(3));
        assert!(0b1010_0110u8.is_bit_low(4));
        assert!(0b1010_0110u8.is_bit_low(5).not());
        assert!(0b1010_0110u8.is_bit_low(6));
        assert!(0b1010_0110u8.is_bit_low(7).not());
    }

    #[test]
    #[should_panic]
    fn test_is_bit_low_panics() {
        0b0000_0000u8.is_bit_low(8);
    }

    #[test]
    fn test_is_bit_high_zeros() {
        assert!(0b0000_0000u8.is_bit_high(0).not());
        assert!(0b0000_0000u8.is_bit_high(1).not());
        assert!(0b0000_0000u8.is_bit_high(2).not());
        assert!(0b0000_0000u8.is_bit_high(3).not());
        assert!(0b0000_0000u8.is_bit_high(4).not());
        assert!(0b0000_0000u8.is_bit_high(5).not());
        assert!(0b0000_0000u8.is_bit_high(6).not());
        assert!(0b0000_0000u8.is_bit_high(7).not());
    }

    #[test]
    fn test_is_bit_high_ones() {
        assert!(0b1111_1111u8.is_bit_high(0));
        assert!(0b1111_1111u8.is_bit_high(1));
        assert!(0b1111_1111u8.is_bit_high(2));
        assert!(0b1111_1111u8.is_bit_high(3));
        assert!(0b1111_1111u8.is_bit_high(4));
        assert!(0b1111_1111u8.is_bit_high(5));
        assert!(0b1111_1111u8.is_bit_high(6));
        assert!(0b1111_1111u8.is_bit_high(7));
    }

    #[test]
    fn test_is_bit_high_noisy() {
        assert!(0b1010_0110u8.is_bit_high(0).not());
        assert!(0b1010_0110u8.is_bit_high(1));
        assert!(0b1010_0110u8.is_bit_high(2));
        assert!(0b1010_0110u8.is_bit_high(3).not());
        assert!(0b1010_0110u8.is_bit_high(4).not());
        assert!(0b1010_0110u8.is_bit_high(5));
        assert!(0b1010_0110u8.is_bit_high(6).not());
        assert!(0b1010_0110u8.is_bit_high(7));
    }

    #[test]
    #[should_panic]
    fn test_is_bit_high_panics() {
        0b0000_0000u8.is_bit_high(8);
    }

    #[test]
    fn test_read() {
        assert!(0b1010_0110u8.read_bit(0).not());
        assert!(0b1010_0110u8.read_bit(1));
        assert!(0b1010_0110u8.read_bit(2));
        assert!(0b1010_0110u8.read_bit(3).not());
        assert!(0b1010_0110u8.read_bit(4).not());
        assert!(0b1010_0110u8.read_bit(5));
        assert!(0b1010_0110u8.read_bit(6).not());
        assert!(0b1010_0110u8.read_bit(7));
    }

    #[test]
    #[should_panic]
    fn test_read_panics() {
        0b1010_0110u8.read_bit(8);
    }

    #[test]
    fn test_read_bits() {
        assert_eq!(0b1010_0110u8.read_bits(0..=0), 0b0000_0000u8);
        assert_eq!(0b1010_0110u8.read_bits(0..=1), 0b0000_0010u8);
        assert_eq!(0b1010_0110u8.read_bits(0..=2), 0b0000_0110u8);
        assert_eq!(0b1010_0110u8.read_bits(0..=3), 0b0000_0110u8);
        assert_eq!(0b1010_0110u8.read_bits(0..=4), 0b0000_0110u8);
        assert_eq!(0b1010_0110u8.read_bits(0..=5), 0b0010_0110u8);
        assert_eq!(0b1010_0110u8.read_bits(0..=6), 0b0010_0110u8);
        assert_eq!(0b1010_0110u8.read_bits(0..=7), 0b1010_0110u8);
        assert_eq!(0b1010_0110u8.read_bits(1..=1), 0b0000_0001u8);
        assert_eq!(0b1010_0110u8.read_bits(1..=2), 0b0000_0011u8);
        assert_eq!(0b1010_0110u8.read_bits(1..=3), 0b0000_0011u8);
        assert_eq!(0b1010_0110u8.read_bits(1..=4), 0b0000_0011u8);
        assert_eq!(0b1010_0110u8.read_bits(1..=5), 0b0001_0011u8);
        assert_eq!(0b1010_0110u8.read_bits(1..=6), 0b0001_0011u8);
        assert_eq!(0b1010_0110u8.read_bits(1..=7), 0b0101_0011u8);
        assert_eq!(0b1010_0110u8.read_bits(2..=2), 0b0000_0001u8);
        assert_eq!(0b1010_0110u8.read_bits(2..=3), 0b0000_0001u8);
        assert_eq!(0b1010_0110u8.read_bits(2..=4), 0b0000_0001u8);
        assert_eq!(0b1010_0110u8.read_bits(2..=5), 0b0000_1001u8);
        assert_eq!(0b1010_0110u8.read_bits(2..=6), 0b0000_1001u8);
        assert_eq!(0b1010_0110u8.read_bits(2..=7), 0b0010_1001u8);
        assert_eq!(0b1010_0110u8.read_bits(3..=3), 0b0000_0000u8);
        assert_eq!(0b1010_0110u8.read_bits(3..=4), 0b0000_0000u8);
        assert_eq!(0b1010_0110u8.read_bits(3..=5), 0b0000_0100u8);
        assert_eq!(0b1010_0110u8.read_bits(3..=6), 0b0000_0100u8);
        assert_eq!(0b1010_0110u8.read_bits(3..=7), 0b0001_0100u8);
        assert_eq!(0b1010_0110u8.read_bits(4..=4), 0b0000_0000u8);
        assert_eq!(0b1010_0110u8.read_bits(4..=5), 0b0000_0010u8);
        assert_eq!(0b1010_0110u8.read_bits(4..=6), 0b0000_0010u8);
        assert_eq!(0b1010_0110u8.read_bits(4..=7), 0b0000_1010u8);
        assert_eq!(0b1010_0110u8.read_bits(5..=5), 0b0000_0001u8);
        assert_eq!(0b1010_0110u8.read_bits(5..=6), 0b0000_0001u8);
        assert_eq!(0b1010_0110u8.read_bits(5..=7), 0b0000_0101u8);
        assert_eq!(0b1010_0110u8.read_bits(6..=6), 0b0000_0000u8);
        assert_eq!(0b1010_0110u8.read_bits(6..=7), 0b0000_0010u8);
        assert_eq!(0b1010_0110u8.read_bits(7..=7), 0b0000_0001u8);
    }

    #[test]
    #[should_panic]
    fn test_read_bits_panics_1() {
        0b1010_0110u8.read_bits(0..=8);
    }

    #[test]
    #[should_panic]
    fn test_read_bits_panics_2() {
        0b1010_0110u8.read_bits(8..=8);
    }
}
