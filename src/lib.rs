#![feature(type_alias_impl_trait)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod iterators;
mod moogcell;
mod internal_structures;

// == junctions ==
mod raw_junctions;
mod shared_junctions;
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

// == structures ==
mod raw_structures;
mod shared_structures;
pub use raw_structures::set as raw_set;
pub use raw_structures::to_one as raw_to_one;
pub use raw_structures::to_set as raw_to_set;
pub use raw_structures::RawSet;
pub use raw_structures::RawToOne;
pub use raw_structures::RawToSet;

pub use shared_structures::set as shared_set;
pub use shared_structures::to_one as shared_to_one;
pub use shared_structures::to_set as shared_to_set;
pub use shared_structures::Set;
pub use shared_structures::ToOne;
pub use shared_structures::ToSet;

// == poms ==
pub mod raw_poms;
mod shared_poms;
pub mod poms { pub use crate::shared_poms::*; }

pub use raw_poms::RawPom;
pub use shared_poms::Pom;

// == misc ==
mod id;
pub use id::Id;

pub mod methods;
pub use methods::*;

#[cfg(test)]
mod test_props;