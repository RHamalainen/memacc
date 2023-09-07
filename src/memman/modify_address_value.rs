use super::ReadFromAddress;
use super::WriteToAddress;

pub trait ModifyAddressValue {
    type Word;
    fn modify_address_value<F>(self, f: F)
    where
        F: FnOnce(&Self::Word) -> Self::Word;
}

/// Implement [`ModifyAddressValue`] for given type.
macro_rules! ImplementModifyAddressValue {
    ($type:ty) => {
        impl ModifyAddressValue for *mut $type {
            type Word = $type;
            #[inline]
            fn modify_address_value<F>(self, f: F)
            where
                F: FnOnce(&Self::Word) -> Self::Word,
            {
                let value_old = self.read_from_address();
                let value_new = f(&value_old);
                self.write_to_address(value_new);
            }
        }
    };
}

ImplementModifyAddressValue!(u8);
ImplementModifyAddressValue!(u16);
ImplementModifyAddressValue!(u32);
ImplementModifyAddressValue!(u64);

#[cfg(test)]
mod tests {
    use crate::bitman::SetBit;
    use crate::bitman::SetBits;
    use crate::memman::ReadFromAddress;

    use super::ModifyAddressValue;

    #[test]
    fn test_modify_address_value_0() {
        let mut value = 0b0000_0000u8;
        let reference = &mut value;
        let address = reference as *mut u8;
        address.modify_address_value(|w| w.set_bit(0));
        assert_eq!(address.read_from_address(), 0b0000_0001);
    }

    #[test]
    fn test_modify_address_value_1() {
        let mut value = 0b0000_0000u8;
        let reference = &mut value;
        let address = reference as *mut u8;
        address.modify_address_value(|w| w.set_bits(0..=1));
        assert_eq!(address.read_from_address(), 0b0000_0011);
    }

    #[test]
    fn test_modify_address_value_2() {
        let mut value = 0b0000_0000u8;
        let reference = &mut value;
        let address = reference as *mut u8;
        let action = |w: &u8| w.set_bit(0);
        address.modify_address_value(action);
        assert_eq!(address.read_from_address(), 0b0000_0001);
    }

    #[test]
    fn test_modify_address_value_3() {
        let mut value = 0b0000_0000u8;
        let reference = &mut value;
        let address = reference as *mut u8;
        let action = |w: &u8| w.set_bits(0..=1);
        address.modify_address_value(action);
        assert_eq!(address.read_from_address(), 0b0000_0011);
    }
}
