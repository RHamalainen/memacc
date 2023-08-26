/// Knows how many bits it contains.
pub trait Bits {
    /// My type.
    type Type;

    /// How many bits my type contains.
    fn bits() -> Self::Type;
}

/// Implement [`Bits`] for given type.
macro_rules! ImplementBits {
    ($type:ty) => {
        impl Bits for $type {
            type Type = $type;
            fn bits() -> Self::Type {
                Self::BITS as Self::Type
            }
        }
    };
}

ImplementBits!(u8);
ImplementBits!(u32);
ImplementBits!(u64);

#[cfg(test)]
mod tests {
    use super::Bits;

    #[test]
    fn test_bits() {
        assert_eq!(u8::bits(), 8);
        assert_eq!(u32::bits(), 32);
        assert_eq!(u64::bits(), 64);
    }
}
