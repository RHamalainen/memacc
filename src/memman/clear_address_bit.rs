use crate::bitman::ClearBit;
use crate::memman::ReadFromAddress;
use crate::memman::WriteToAddress;

pub trait ClearAddressBit {
    type Type;
    type Index;
    fn clear_address_bit(self, index: Self::Index);
}

/// Implement [`ClearAddressBit`] for given type.
macro_rules! ImplementClearAddressBit {
    ($type:ty) => {
        impl ClearAddressBit for *mut $type {
            type Type = *mut $type;
            type Index = $type;
            fn clear_address_bit(self, index: Self::Index) {
                let value_old = self.read_from_address();
                let value_new = value_old.clear_bit(index);
                self.write_to_address(value_new);
            }
        }
    };
}

ImplementClearAddressBit!(u8);
// TODO: ImplementClearAddressBit!(u16);
ImplementClearAddressBit!(u32);
ImplementClearAddressBit!(u64);

// TODO: tests
