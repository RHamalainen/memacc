use crate::bitman::ClearBit;
use crate::bitman::SetBit;

/// Can write single bit value.
pub trait WriteBit {
    /// My type.
    type Type;

    /// Write single bit.
    fn write_bit(&self, index: Self::Type, value: bool) -> Self::Type;
}

/// Implement [`WriteBit`] for given type.
macro_rules! ImplementWriteBit {
    ($type:ty) => {
        impl WriteBit for $type {
            type Type = Self;
            #[inline]
            #[must_use]
            fn write_bit(&self, index: Self, value: bool) -> Self {
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
ImplementWriteBit!(u64);

// TODO: tests
