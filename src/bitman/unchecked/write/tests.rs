//! Unit tests.

#[cfg(test)]
mod tests {
    use crate::bitman::unchecked::write::*;

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

    #[test]
    fn test_write_bits_scattered() {
        // TODO
        assert_eq!(
            0b1010_0110u8.write_bits_scattered(&[0, 1, 3], 0b0000_0111u8),
            0b1010_1111u8
        );
    }
}
