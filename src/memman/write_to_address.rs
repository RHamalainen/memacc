pub trait WriteToAddress {
    type Type;
    fn write_to_address(self, value: Self::Type);
}

/// Implement [`WriteToAddress`] for given type.
macro_rules! ImplementWriteToAddress {
    ($type:ty) => {
        impl WriteToAddress for *mut $type {
            type Type = $type;
            #[allow(clippy::not_unsafe_ptr_arg_deref)]
            fn write_to_address(self, value: Self::Type) {
                // SAFETY:
                // Programmer is responsible for reading from valid address.
                unsafe { self.write_volatile(value) }
            }
        }
    };
}

ImplementWriteToAddress!(u8);
ImplementWriteToAddress!(u16);
ImplementWriteToAddress!(u32);
ImplementWriteToAddress!(u64);

// TODO: tests
