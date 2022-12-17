//! Bit manipulation.

mod read;
mod write;

pub use read::IsBitHigh;
pub use read::IsBitLow;
pub use read::ReadBit;
pub use read::ReadBitRange;
pub use write::ClearBit;
pub use write::SetBit;
pub use write::SetBitRange;
pub use write::WriteBit;
pub use write::WriteBits;
