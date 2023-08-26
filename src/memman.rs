//! Memory manipulation.

mod modify_address_value;
mod read_from_address;
mod write_to_address;

pub use modify_address_value::ModifyAddressValue;
pub use read_from_address::ReadFromAddress;
pub use write_to_address::WriteToAddress;

use crate::bitman::ClearBit;
use crate::bitman::ReadBit;
use crate::bitman::ReadBits;
use crate::bitman::SetBit;
use crate::bitman::WriteBits;
use core::ops::RangeInclusive;
use core::ptr::read_volatile;
use core::ptr::write_volatile;

/// Read value from memory address.
#[deprecated]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline]
#[must_use]
pub fn read_from_address(address: *mut u32) -> u32 {
    // SAFETY:
    // Programmer is responsible for reading from valid address.
    unsafe { read_volatile(address) }
}

/// Write value to memory address.
#[deprecated]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[inline]
pub fn write_to_address(address: *mut u32, value: u32) {
    // SAFETY:
    // Programmer is responsible for reading from valid address.
    unsafe { write_volatile(address, value) }
}

/// Set memory address value's bit high.
#[deprecated]
#[inline]
pub fn set_address_bit(address: *mut u32, index: u32) {
    let old = read_from_address(address);
    let new = old.set_bit(index);
    write_to_address(address, new);
}

/// Set memory address value's bit low.
#[deprecated]
#[inline]
pub fn clear_address_bit(address: *mut u32, index: u32) {
    let old = read_from_address(address);
    let new = old.clear_bit(index);
    write_to_address(address, new);
}

/// Read memory address value's bit's value.
#[deprecated]
#[inline]
#[must_use]
pub fn read_address_bit(address: *mut u32, index: u32) -> bool {
    let value = read_from_address(address);
    value.read_bit(index)
}

/// Read multiple bits from memory address.
#[deprecated]
#[inline]
#[must_use]
pub fn read_address_bits(address: *mut u32, indices: RangeInclusive<u32>) -> u32 {
    let value = read_from_address(address);
    value.read_bits(indices)
}

/// Write multiple bits to memory address.
#[deprecated]
#[inline]
pub fn write_address_bits(address: *mut u32, indices: RangeInclusive<u32>, value: u32) {
    // TODO
    unimplemented!()

    /*let start = *indices.start();
    #[allow(clippy::arithmetic_side_effects)]
    let length = indices.end() - indices.start();
    let old = read_from_address(address);
    let new = old.write_bits(start, value, length);
    write_to_address(address, new);*/
}
