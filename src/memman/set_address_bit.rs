use crate::bitman::SetBit;
use crate::memman::ReadFromAddress;
use crate::memman::WriteToAddress;

pub trait SetAddressBit {
    type Type;
    type Index;
    fn set_address_bit(self, index: Self::Index);
}

/// Implement [`SetAddressBit`] for given type.
macro_rules! ImplementSetAddressBit {
    ($type:ty) => {
        impl SetAddressBit for *mut $type {
            type Type = *mut $type;
            type Index = $type;
            fn set_address_bit(self, index: Self::Index) {
                let value_old = self.read_from_address();
                let value_new = value_old.set_bit(index);
                self.write_to_address(value_new);
            }
        }
    };
}

ImplementSetAddressBit!(u8);
// TODO: ImplementSetAddressBit!(u16);
ImplementSetAddressBit!(u32);
ImplementSetAddressBit!(u64);

// TODO: tests
