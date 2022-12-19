//! Unit tests.

#[cfg(test)]
mod tests {
    use crate::bitman::unchecked::read::*;

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

    #[test]
    fn test_read_bits_scattered() {
        // Test extreme cases.
        assert_eq!(
            0b1010_0110u8.read_bits_scattered(&[0, 1, 2, 3, 4, 5, 6, 7]),
            0b1010_0110u8
        );
        assert_eq!(
            0b1010_0110u8.read_bits_scattered(&[7, 6, 5, 4, 3, 2, 1, 0]),
            0b0110_0101u8
        );
    }

    #[test]
    #[should_panic]
    fn test_read_bits_scattered_panics_1() {
        let _ = 0b0000_0000u8.read_bits_scattered(&[8]);
    }

    #[test]
    #[should_panic]
    fn test_read_bits_scattered_panics_2() {
        let _ = 0b0000_0000u8.read_bits_scattered(&[0, 0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
