#![feature(type_alias_impl_trait)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod methods;
mod raw_junctions;
mod shared_junctions;
mod keybound;
mod structures;

pub use raw_junctions::{one_to_one, RawOneToOne};
pub use raw_junctions::{one_to_set, RawOneToSet};
pub use raw_junctions::{set_to_one, RawSetToOne};
pub use raw_junctions::{set_to_set, RawSetToSet};

pub use shared_junctions::one_to_one as shared_one_to_one;
pub use shared_junctions::SharedOneToOne;
pub use shared_junctions::one_to_set as shared_one_to_set;
pub use shared_junctions::SharedOneToSet;
pub use shared_junctions::set_to_one as shared_set_to_one;
pub use shared_junctions::SharedSetToOne;
pub use shared_junctions::set_to_set as shared_set_to_set;
pub use shared_junctions::SharedSetToSet;

pub use methods::*;