// NOTE: Don't expose the eviction code from the internal_structures
// Otherwise, be roughly the same!

mod debug_impl;

pub mod set;
pub mod to_one;
pub mod to_set;

pub use set::RawSet;
pub use to_one::RawToOne;
pub use to_set::RawToSet;