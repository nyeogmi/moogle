// named "methods" instead of "traits" to convey that you probably wanna import this if you'd like access to the methods
mod internal;
mod junction_readers;
mod junction_shared;
mod junction_writers;
mod shared_standard;

pub(crate) use self::internal::*;
pub use self::junction_readers::*;
pub use self::junction_shared::*;
pub use self::junction_writers::*;
pub use self::shared_standard::*;