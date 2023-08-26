use crate::bitman::unchecked::write::ClearNLeft;
use crate::bitman::unchecked::Bits;
use crate::bitman::unchecked::Ones;
use crate::bitman::unchecked::Zeros;

/// Can create N ones starting from right.
pub trait NOnes {
    /// My type.
    type Type;

    /// Create N ones starting from right.
    fn n_ones(n: Self::Type) -> Self::Type;
}

/// Implement [`NOnes`] for given type.
macro_rules! ImplementNOnes {
    ($type:ty) => {
        impl NOnes for $type {
            type Type = $type;
            fn n_ones(n: Self::Type) -> Self::Type {
                if n == 0 {
                    Self::zeros()
                } else if (1..Self::bits()).contains(&n) {
                    Self::ones().clear_n_left(Self::bits() - n)
                } else {
                    Self::ones()
                }
            }
        }
    };
}

ImplementNOnes!(u8);
ImplementNOnes!(u32);
ImplementNOnes!(u64);

#[cfg(test)]
mod tests {
    use super::NOnes;

    #[test]
    fn test_n_ones() {
        assert_eq!(u8::n_ones(0), 0b0000_0000);
        assert_eq!(u8::n_ones(1), 0b0000_0001);
        assert_eq!(u8::n_ones(2), 0b0000_0011);
        assert_eq!(u8::n_ones(3), 0b0000_0111);
        assert_eq!(u8::n_ones(4), 0b0000_1111);
        assert_eq!(u8::n_ones(5), 0b0001_1111);
        assert_eq!(u8::n_ones(6), 0b0011_1111);
        assert_eq!(u8::n_ones(7), 0b0111_1111);
        assert_eq!(u8::n_ones(8), 0b1111_1111);
        assert_eq!(u8::n_ones(9), 0b1111_1111);
    }
}
