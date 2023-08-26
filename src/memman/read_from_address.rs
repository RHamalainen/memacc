pub trait ReadFromAddress {
    type Type;
    fn read_from_address(self) -> Self::Type;
}

/// Implement [`ReadFromAddress`] for given type.
macro_rules! ImplementReadFromAddress {
    ($type:ty) => {
        impl ReadFromAddress for *mut $type {
            type Type = $type;
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            fn read_from_address(self) -> Self::Type {
                // SAFETY:
                // Programmer is responsible for reading from valid address.
                unsafe { self.read_volatile() }
            }
        }
    };
}

ImplementReadFromAddress!(u8);
ImplementReadFromAddress!(u16);
ImplementReadFromAddress!(u32);
ImplementReadFromAddress!(u64);

#[cfg(test)]
mod tests {
    use super::ReadFromAddress;

    #[test]
    fn test_read_from_address_u8() {
        let mut value = 0b1010_0110u8;
        let reference = &mut value;
        let address = reference as *mut u8;
        assert_eq!(address.read_from_address(), value);
    }

    #[test]
    fn test_read_from_address_u16() {
        let mut value = 0b1010_0110_1010_0110u16;
        let reference = &mut value;
        let address = reference as *mut u16;
        assert_eq!(address.read_from_address(), value);
    }

    #[test]
    fn test_read_from_address_u32() {
        let mut value = 0b1010_0110_1010_0110_1010_0110_1010_0110u32;
        let reference = &mut value;
        let address = reference as *mut u32;
        assert_eq!(address.read_from_address(), value);
    }

    #[test]
    fn test_read_from_address_u64() {
        let mut value =
            0b1010_0110_1010_0110_1010_0110_1010_0110_1010_0110_1010_0110_1010_0110_1010_0110u64;
        let reference = &mut value;
        let address = reference as *mut u64;
        assert_eq!(address.read_from_address(), value);
    }
}
