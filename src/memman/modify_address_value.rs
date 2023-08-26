use super::ReadFromAddress;
use super::WriteToAddress;

pub trait ModifyAddressValue {
    type Type;
    fn modify_address_value(self, f: fn(Self::Type) -> Self::Type);
}

/// Implement [`ModifyAddressValue`] for given type.
macro_rules! ImplementModifyAddressValue {
    ($type:ty) => {
        impl ModifyAddressValue for *mut $type {
            type Type = $type;
            fn modify_address_value(self, f: fn(Self::Type) -> Self::Type) {
                let value_old = self.read_from_address();
                let value_new = f(value_old);
                self.write_to_address(value_new);
            }
        }
    };
}

ImplementModifyAddressValue!(u8);
ImplementModifyAddressValue!(u16);
ImplementModifyAddressValue!(u32);
ImplementModifyAddressValue!(u64);

// TODO: tests
