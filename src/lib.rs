#![feature(min_type_alias_impl_trait)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

pub mod methods;
mod junctions;
mod keybound;
mod structures;

pub use junctions::{one_to_one, OneToOne};
pub use junctions::{one_to_set, OneToSet};
pub use junctions::{set_to_one, SetToOne};
pub use junctions::{set_to_set, SetToSet};

pub use methods::*;