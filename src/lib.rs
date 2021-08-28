#![feature(type_alias_impl_trait)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod id;

mod iterators;
pub mod methods;
mod moogcell;
mod raw_junctions;
mod shared_junctions;
mod structures;

pub mod raw_poms;
mod shared_poms;
pub mod poms { pub use crate::shared_poms::*; }

pub use raw_poms::RawPom;

pub use poms::Pom;

pub use raw_junctions::one_to_one as raw_one_to_one;
pub use raw_junctions::one_to_set as raw_one_to_set;
pub use raw_junctions::set_to_one as raw_set_to_one;
pub use raw_junctions::set_to_set as raw_set_to_set;
pub use raw_junctions::RawOneToOne;
pub use raw_junctions::RawOneToSet;
pub use raw_junctions::RawSetToOne;
pub use raw_junctions::RawSetToSet;

pub use shared_junctions::{one_to_one, OneToOne};
pub use shared_junctions::{one_to_set, OneToSet};
pub use shared_junctions::{set_to_one, SetToOne};
pub use shared_junctions::{set_to_set, SetToSet};

pub use id::Id;

pub use methods::*;

#[cfg(test)]
mod test_props;