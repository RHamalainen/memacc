use core::ops::Not;

use crate::bitman::unchecked::Zeros;

/// Can create all ones.
pub trait Ones {
    /// My type.
    type Type;

    /// Create all ones.
    fn ones() -> Self::Type;
}

/// Implement [`Ones`] for given type.
macro_rules! ImplementOnes {
    ($type:ty) => {
        impl Ones for $type {
            type Type = $type;
            fn ones() -> Self::Type {
                Self::zeros().not()
            }
        }
    };
}

ImplementOnes!(u8);
ImplementOnes!(u32);
ImplementOnes!(u64);

#[cfg(test)]
mod tests {
    use super::Ones;

    #[test]
    fn test_ones() {
        assert_eq!(u8::ones(), 0b1111_1111);
        assert_eq!(u32::ones(), 0b1111_1111_1111_1111_1111_1111_1111_1111);
        assert_eq!(
            u64::ones(),
            0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111
        );
    }
}
