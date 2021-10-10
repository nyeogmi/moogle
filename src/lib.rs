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
pub use raw_junctions::one_to_many as raw_one_to_many;
pub use raw_junctions::many_to_one as raw_many_to_one;
pub use raw_junctions::many_to_many as raw_many_to_many;
pub use raw_junctions::RawOneToOne;
pub use raw_junctions::RawOneToMany;
pub use raw_junctions::RawManyToOne;
pub use raw_junctions::RawManyToMany;

pub use shared_junctions::{one_to_one, OneToOne};
pub use shared_junctions::{one_to_many, OneToMany};
pub use shared_junctions::{many_to_one, ManyToOne};
pub use shared_junctions::{many_to_many, ManyToMany};

// == structures ==
mod raw_structures;
mod shared_structures;
pub use raw_structures::set as raw_set;
pub use raw_structures::to_one as raw_to_one;
pub use raw_structures::to_many as raw_to_many;
pub use raw_structures::RawSet;
pub use raw_structures::RawToOne;
pub use raw_structures::RawToMany;

pub use shared_structures::set as shared_set;
pub use shared_structures::to_one as shared_to_one;
pub use shared_structures::to_many as shared_to_many;
pub use shared_structures::Set;
pub use shared_structures::ToOne;
pub use shared_structures::ToMany;

// == poms ==
pub mod raw_poms;
pub mod floating_poms;
mod shared_poms;
pub mod poms { pub use crate::shared_poms::*; }

pub use raw_poms::RawPom;
pub use floating_poms::{Floating, FloatingPom};
pub use shared_poms::Pom;

// == misc ==
mod id;
pub use id::{Id, IdLike};

pub mod methods;
pub use methods::*;

#[cfg(test)]
mod test_props;