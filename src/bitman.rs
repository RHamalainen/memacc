//! Bit manipulation.

pub mod checked;
pub mod unchecked;

pub use unchecked::read::IsBitHigh;
pub use unchecked::read::IsBitLow;
pub use unchecked::read::ReadBit;
pub use unchecked::read::ReadBitRange;
pub use unchecked::write::ClearBit;
pub use unchecked::write::SetBit;
pub use unchecked::write::SetBitRange;
pub use unchecked::write::WriteBit;
pub use unchecked::write::WriteBits;
