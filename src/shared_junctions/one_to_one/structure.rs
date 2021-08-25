use crate::keybound::Id;

use super::{Fwd, Bwd};

use crate::junctions::one_to_one::OneToOne;
use super::super::moogcell::MoogCell;

// == Data structure ==
pub struct SharedOneToOne<A: Id, B: Id> {
    pub(super) raw: MoogCell<OneToOne<A, B>>
}

// == Constructor et al ==
impl<A: Id, B: Id> SharedOneToOne<A, B> {
    pub fn new() -> SharedOneToOne<A, B> {
        SharedOneToOne { raw: MoogCell::new(OneToOne::new()) }
    }

    pub fn fwd(&self) -> Fwd<A, B> { Fwd { me: self } }
    pub fn bwd(&self) -> Bwd<A, B> { Bwd { me: self } }
}