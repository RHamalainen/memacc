use core::ops::Shl;
use core::ops::Shr;

use crate::bitman::Bits;
use crate::bitman::Zeros;

/// Can clear N bits starting from right.
pub trait ClearNRight {
    /// My type.
    type Type;

    /// Clear N bits starting from right.
    fn clear_n_right(self, n: Self::Type) -> Self::Type;

    /// Keep N bits starting from left.
    fn keep_n_left(self, n: Self::Type) -> Self::Type;
}

/// Implement [`ClearNRight`] for given type.
macro_rules! ImplementClearNRight {
    ($type:ty) => {
        impl ClearNRight for $type {
            type Type = $type;
            fn clear_n_right(self, n: Self::Type) -> Self::Type {
                if Self::bits() <= n {
                    Self::zeros()
                } else {
                    self.shr(n).shl(n)
                }
            }
            fn keep_n_left(self, n: Self::Type) -> Self::Type {
                if Self::bits() <= n {
                    self
                } else {
                    self.clear_n_right(Self::bits() - n)
                }
            }
        }
    };
}

ImplementClearNRight!(u8);
ImplementClearNRight!(u32);
ImplementClearNRight!(u64);

#[cfg(test)]
mod tests {
    use super::ClearNRight;
    use crate::bitman::Ones;

    #[test]
    fn test_clear_n_right() {
        // TODO: more comprehensive testing
        assert_eq!(u8::ones().clear_n_right(0), 0b1111_1111);
        assert_eq!(u8::ones().clear_n_right(1), 0b1111_1110);
        assert_eq!(u8::ones().clear_n_right(2), 0b1111_1100);
        assert_eq!(u8::ones().clear_n_right(3), 0b1111_1000);
        assert_eq!(u8::ones().clear_n_right(4), 0b1111_0000);
        assert_eq!(u8::ones().clear_n_right(5), 0b1110_0000);
        assert_eq!(u8::ones().clear_n_right(6), 0b1100_0000);
        assert_eq!(u8::ones().clear_n_right(7), 0b1000_0000);
        assert_eq!(u8::ones().clear_n_right(8), 0b0000_0000);
        assert_eq!(u8::ones().clear_n_right(9), 0b0000_0000);
    }

    #[test]
    fn test_keep_n_left() {
        // TODO: more comprehensive testing
        assert_eq!(u8::ones().keep_n_left(0), 0b0000_0000);
        assert_eq!(u8::ones().keep_n_left(1), 0b1000_0000);
        assert_eq!(u8::ones().keep_n_left(2), 0b1100_0000);
        assert_eq!(u8::ones().keep_n_left(3), 0b1110_0000);
        assert_eq!(u8::ones().keep_n_left(4), 0b1111_0000);
        assert_eq!(u8::ones().keep_n_left(5), 0b1111_1000);
        assert_eq!(u8::ones().keep_n_left(6), 0b1111_1100);
        assert_eq!(u8::ones().keep_n_left(7), 0b1111_1110);
        assert_eq!(u8::ones().keep_n_left(8), 0b1111_1111);
        assert_eq!(u8::ones().keep_n_left(9), 0b1111_1111);
    }
}
