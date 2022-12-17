//! ```
//! # use memacc::bitman::checked::index::IndexU8;
//! # use memacc::bitman::checked::index::I;
//! # use memacc::bitman::checked::read::ReadBitSafe;
//! let state = 0b1010_0110u8.read_bit_safe(I::<0>);
//! assert_eq!(state, false);
//! ```
//!
//! ```
//! # use memacc::bitman::checked::index::IndexU8;
//! # use memacc::bitman::checked::index::I;
//! # use memacc::bitman::checked::read::ReadBitSafe;
//! # use memacc::index;
//! let state = 0b1010_0110u8.read_bit_safe(index!(0));
//! assert_eq!(state, false);
//! ```
//!
//! ```
//! # use memacc::bitman::checked::index::IndexU8;
//! # use memacc::bitman::checked::index::I;
//! # use memacc::bitman::checked::read::ReadBitSafe;
//! let state = 0b1010_0110u8.read_bit_safe(I::<1>);
//! assert_eq!(state, true);
//! ```
//!
//! ```compile_fail
//! # use memacc::bitman::checked::index::IndexU8;
//! # use memacc::bitman::checked::index::I;
//! # use memacc::bitman::checked::read::ReadBitSafe;
//! let state = 0b1010_0110u8.read_bit_safe(I::<8>);
//! assert_eq!(state, false);
//! ```
//!
//! ```compile_fail
//! # use memacc::bitman::checked::index::IndexU8;
//! # use memacc::bitman::checked::index::I;
//! # use memacc::bitman::checked::read::ReadBitSafe;
//! # use memacc::index;
//! let state = 0b1010_0110u8.read_bit_safe(index!(8));
//! assert_eq!(state, false);
//! ```
//!
//! ```
//! # use memacc::bitman::checked::index::IndexU8;
//! # use memacc::bitman::checked::indices::IndexRangeU8;
//! # use memacc::bitman::checked::index::I;
//! # use memacc::bitman::checked::indices::IR;
//! # use memacc::bitman::checked::read::ReadBitsSafe;
//! let bits = 0b1010_0110u8.read_bits_safe(IR::<0, 4>);
//! assert_eq!(bits, 0b0000_0110u8);
//! ```

use crate::bitman::checked::index::IndexU8;
use crate::bitman::checked::indices::IndexRangeU8;
use core::ops::BitAnd;
use core::ops::Shl;
use core::ops::Shr;

/// Can read single bit value.
pub trait ReadBitSafe {
    /// My type.
    type Type;

    /// Read single bit.
    fn read_bit_safe(&self, index: impl IndexU8) -> bool;
}

impl ReadBitSafe for u8 {
    type Type = Self;
    fn read_bit_safe(&self, index: impl IndexU8) -> bool {
        // Move target bit to index 0.
        let temporary_1 = self.shr(index.value());

        // Clear all bits except index 0.
        let temporary_2 = 0b1.bitand(temporary_1);

        // If byte value is one, then the bit at given index is high.
        temporary_2 == 1
    }
}

pub trait ReadBitsSafe {
    type Type;
    fn read_bits_safe(&self, range: impl IndexRangeU8 + core::marker::Copy) -> u8;
}

impl ReadBitsSafe for u8 {
    type Type = Self;
    fn read_bits_safe(&self, range: impl IndexRangeU8 + core::marker::Copy) -> u8 {
        let bits = Self::BITS as Self;
        let start = range.start();
        let end = range.end();

        // Clear bits lower than range start.
        let temporary_1 = self.shr(start);
        let temporary_2 = temporary_1.shl(start);

        // Clear bits higher than range end.
        let amount = bits - end - 1;
        let temporary_3 = temporary_2.shl(amount);
        let temporary_4 = temporary_3.shr(amount);

        // Move bit range to index 0.
        let temporary_5 = temporary_4.shr(start);

        temporary_5
    }
}

// TODO
