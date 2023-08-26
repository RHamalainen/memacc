use crate::bitman::unchecked::write::ClearNRight;
use crate::bitman::unchecked::NOnes;

/// Can create N ones starting from right with offset.
pub trait NOnesOffset {
    /// My type.
    type Type;

    /// Create N ones starting from right with offset.
    fn n_ones_offset(n: Self::Type, offset: Self::Type) -> Self::Type;
}

/// Implement [`NOnesOffset`] for given type.
macro_rules! ImplementNOnesOffset {
    ($type:ty) => {
        impl NOnesOffset for $type {
            type Type = $type;
            fn n_ones_offset(n: Self::Type, offset: Self::Type) -> Self::Type {
                Self::n_ones(offset + n).clear_n_right(offset)
            }
        }
    };
}

ImplementNOnesOffset!(u8);
ImplementNOnesOffset!(u32);
ImplementNOnesOffset!(u64);

#[cfg(test)]
mod tests {
    use super::NOnesOffset;

    #[test]
    fn test_n_ones_offset() {
        assert_eq!(u8::n_ones_offset(3, 0), 0b0000_0111);
        assert_eq!(u8::n_ones_offset(3, 1), 0b0000_1110);
        assert_eq!(u8::n_ones_offset(3, 2), 0b0001_1100);
        assert_eq!(u8::n_ones_offset(3, 3), 0b0011_1000);
        assert_eq!(u8::n_ones_offset(3, 4), 0b0111_0000);
        assert_eq!(u8::n_ones_offset(3, 5), 0b1110_0000);
        assert_eq!(u8::n_ones_offset(3, 6), 0b1100_0000);
        assert_eq!(u8::n_ones_offset(3, 7), 0b1000_0000);
        assert_eq!(u8::n_ones_offset(3, 8), 0b0000_0000);
        assert_eq!(u8::n_ones_offset(3, 9), 0b0000_0000);
    }
}
