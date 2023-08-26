/// Can create all zeros.
pub trait Zeros {
    /// My type.
    type Type;

    /// Create all zeros.
    fn zeros() -> Self::Type;
}

/// Implement [`Zeros`] for given type.
macro_rules! ImplementZeros {
    ($type:ty) => {
        impl Zeros for $type {
            type Type = $type;
            fn zeros() -> Self::Type {
                0 as Self::Type
            }
        }
    };
}

ImplementZeros!(u8);
ImplementZeros!(u32);
ImplementZeros!(u64);

#[cfg(test)]
mod tests {
    use super::Zeros;

    #[test]
    fn test_zeros() {
        assert_eq!(u8::zeros(), 0b0000_0000);
        assert_eq!(u32::zeros(), 0b0000_0000_0000_0000_0000_0000_0000_0000);
        assert_eq!(
            u64::zeros(),
            0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000
        );
    }
}
