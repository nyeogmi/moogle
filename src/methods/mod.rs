// named "methods" instead of "traits" to convey that you probably wanna import this if you'd like access to the methods
mod internal;
mod readers;
mod shared;
mod writers;

pub(crate) use self::internal::*;
pub use self::readers::*;
pub use self::shared::*;
pub use self::writers::*;