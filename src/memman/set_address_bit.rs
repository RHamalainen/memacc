use crate::bitman::SetBit;

use super::ReadFromAddress;
use super::WriteToAddress;

pub trait SetAddressBit {
    type Type;
    type Index;
    fn set_address_bit(self, index: Self::Index);
}

impl SetAddressBit for *mut u32 {
    type Type = *mut u32;
    type Index = u32;
    fn set_address_bit(self, index: Self::Index) {
        let value_old = self.read_from_address();
        let value_new = value_old.set_bit(index);
        self.write_to_address(value_new);
    }
}
